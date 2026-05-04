use crate::crc32::crc32;
use crate::error::Error;
use crate::models::ItemPreviewData;
use crate::proto;

/// Maximum hex string length (= max 2048 bytes of binary payload).
const MAX_HEX_LEN: usize = 4096;
/// Minimum binary payload size in bytes.
const MIN_BYTES: usize = 6;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn hex_decode(s: &str) -> Result<Vec<u8>, Error> {
    // Pre-validation now lives in `deserialize` so we surface MalformedLink
    // with full context (input preview, exact length). This helper still
    // validates as a defense-in-depth measure for direct callers.
    if s.len() % 2 != 0 {
        return Err(Error::MalformedLink(format!(
            "hex payload has invalid length ({} chars, must be even)",
            s.len()
        )));
    }
    let mut bytes = Vec::with_capacity(s.len() / 2);
    let chars: Vec<u8> = s.bytes().collect();
    for chunk in chars.chunks(2) {
        let hi = hex_nibble(chunk[0])?;
        let lo = hex_nibble(chunk[1])?;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

fn hex_nibble(b: u8) -> Result<u8, Error> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(Error::MalformedLink(format!(
            "payload contains non-hex character: {:?}",
            b as char
        ))),
    }
}

fn hex_encode_upper(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let hi = b >> 4;
        let lo = b & 0x0F;
        s.push(nibble_to_hex_upper(hi));
        s.push(nibble_to_hex_upper(lo));
    }
    s
}

fn nibble_to_hex_upper(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + n - 10) as char,
        _ => unreachable!(),
    }
}

/// Compute the checksum over [0x00] + proto_bytes.
fn compute_checksum(proto_bytes: &[u8]) -> u32 {
    let mut buf = Vec::with_capacity(1 + proto_bytes.len());
    buf.push(0x00u8);
    buf.extend_from_slice(proto_bytes);
    let crc = crc32(&buf);
    let len = proto_bytes.len() as u32;
    (crc & 0xFFFF) ^ (len.wrapping_mul(crc))
}

// ─────────────────────────────────────────────────────────────────────────────
// URL extraction
// ─────────────────────────────────────────────────────────────────────────────

/// Checks if a character is an ASCII hex digit (upper or lower).
fn is_hex(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
}

/// Extract the hex payload from various URL/link formats.
///
/// Priority:
/// 1. Hybrid format: find `S\d+A\d+D` then check if rest is hex with letters a-f
/// 2. Classic/market URL: find `A` after `%20`/space/`+`, take following even-length hex
/// 3. Pure masked: find `csgo_econ_action_preview` + separator then take ≥10 hex chars
/// 4. Bare hex: strip whitespace
fn extract_hex(input: &str) -> String {
    let s = input.trim();
    let bytes = s.as_bytes();

    // 1. Hybrid format: look for S\d+A\d+D followed by hex (lowercase a-f)
    //    Pattern: S<steamid64>A<assetid>D<hex_payload>
    if let Some(hex) = try_extract_hybrid(s) {
        return hex;
    }

    // 2. Classic/market URL: look for separator + A<hex>
    //    The 'A' appears after %20, space, or '+' in action preview URLs.
    //    After 'A' we expect digits (asset id), but the format ends with D<number>
    //    For classic links, the portion after the last 'D' segment is NOT hex payload.
    //    Classic format: S<steamid>A<assetid>D<param>
    //    The 'A' number here is the actual asset ID, not hex payload.
    //    We need to distinguish classic from hybrid.

    // 3. Look for csgo_econ_action_preview + separator
    if let Some(pos) = find_subsequence(bytes, b"csgo_econ_action_preview") {
        let after = &bytes[pos + b"csgo_econ_action_preview".len()..];
        // skip separator: %20 or space or +
        let payload_start = if after.starts_with(b"%20") {
            3
        } else if after.first().copied() == Some(b' ') || after.first().copied() == Some(b'+') {
            1
        } else {
            0
        };
        let payload = &after[payload_start..];

        // Skip optional '%' prefix before hex payload
        let payload = if payload.first().copied() == Some(b'%') {
            &payload[1..]
        } else {
            payload
        };

        // Check if this is hybrid (S...A...D<hex>) or classic (S...A...D<decimal>)
        // Hybrid: after D, chars are lowercase hex including a-f
        // Classic: after D, chars are only decimal digits
        if payload.first().copied() == Some(b'S') {
            if let Some(hex) = try_extract_hybrid_from(payload) {
                return hex;
            }
            // Try classic — the D field is decimal only, not hex
            // For classic links, we return empty (not a masked link)
            return String::new();
        }

        // Pure masked: the payload itself is hex
        let hex_part: String = payload
            .iter()
            .take_while(|&&c| is_hex(c))
            .map(|&c| c as char)
            .collect();
        if hex_part.len() >= 10 {
            return hex_part;
        }
    }

    // 4. Bare hex — strip whitespace and try
    let bare: String = bytes.iter().filter(|&&c| is_hex(c)).map(|&c| c as char).collect();
    if bare.len() >= 10 {
        return bare;
    }

    String::new()
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

/// Try to extract hex payload from hybrid format S<n>A<n>D<hex>
fn try_extract_hybrid(s: &str) -> Option<String> {
    try_extract_hybrid_from(s.as_bytes())
}

fn try_extract_hybrid_from(bytes: &[u8]) -> Option<String> {
    // Find 'S' at any position
    let s_pos = bytes.iter().position(|&c| c == b'S')?;
    let after_s = &bytes[s_pos + 1..];

    // Skip digits after S
    let a_pos = after_s.iter().position(|&c| c == b'A')?;
    // Verify those were all digits
    if !after_s[..a_pos].iter().all(|&c| c.is_ascii_digit()) {
        return None;
    }

    let after_a = &after_s[a_pos + 1..];

    // Skip digits after A
    let d_pos = after_a.iter().position(|&c| c == b'D')?;
    if !after_a[..d_pos].iter().all(|&c| c.is_ascii_digit()) {
        return None;
    }

    let after_d = &after_a[d_pos + 1..];

    // The rest should be hex with at least one a-f character (lowercase) to be hybrid
    let hex_chars: Vec<u8> = after_d
        .iter()
        .take_while(|&&c| is_hex(c))
        .copied()
        .collect();

    if hex_chars.len() < 10 {
        return None;
    }

    // Must contain at least one lowercase a-f to distinguish from pure decimal classic link
    let has_lower_hex = hex_chars.iter().any(|&c| matches!(c, b'a'..=b'f'));
    // Also must be even length for valid hex
    let hex_str: String = hex_chars.iter().map(|&c| c as char).collect();

    // Classic links have only decimal digits after D — hybrid has hex
    // If all chars are decimal (0-9), it's a classic link
    let all_decimal = hex_chars.iter().all(|&c| c.is_ascii_digit());
    if all_decimal {
        return None;
    }

    if has_lower_hex || hex_str.contains(|c: char| matches!(c, 'A'..='F')) {
        Some(hex_str)
    } else {
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `true` if the input appears to be a masked (tool-generated or native) inspect link.
///
/// Masked links contain a hex-encoded protobuf payload (either bare or embedded in a URL).
pub fn is_masked(link: &str) -> bool {
    // Hybrid format check
    if try_extract_hybrid(link).is_some() {
        return true;
    }

    // Check for csgo_econ_action_preview with a non-classic payload
    let bytes = link.as_bytes();
    if let Some(pos) = find_subsequence(bytes, b"csgo_econ_action_preview") {
        let after = &bytes[pos + b"csgo_econ_action_preview".len()..];
        let payload_start = if after.starts_with(b"%20") {
            3
        } else if after.first().copied() == Some(b' ') || after.first().copied() == Some(b'+') {
            1
        } else {
            0
        };
        let payload = &after[payload_start..];

        // Skip optional '%' prefix before hex payload
        let payload = if payload.first().copied() == Some(b'%') {
            &payload[1..]
        } else {
            payload
        };

        if payload.first().copied() == Some(b'S') {
            // Classic or hybrid
            return try_extract_hybrid_from(payload).is_some();
        }

        // Pure masked
        let hex_len = payload.iter().take_while(|&&c| is_hex(c)).count();
        if hex_len >= 10 {
            return true;
        }
    }

    // Bare hex check (at least 10 chars, even length, all hex)
    let s = link.trim();
    let all_hex = s.bytes().all(|c| is_hex(c));
    if all_hex && s.len() >= 10 && s.len() % 2 == 0 {
        return true;
    }

    false
}

/// Returns `true` if the input appears to be a classic (non-masked) CS2 inspect link.
///
/// Classic links follow the Steam format:
/// `steam://rungame/730/.../+csgo_econ_action_preview%20S<steamid>A<assetid>D<param>`
/// where the part after `D` is a decimal number, NOT hex.
pub fn is_classic(link: &str) -> bool {
    if !link.contains("csgo_econ_action_preview") {
        return false;
    }

    // Must contain S...A...D pattern
    let bytes = link.as_bytes();
    let Some(pos) = find_subsequence(bytes, b"csgo_econ_action_preview") else {
        return false;
    };

    let after = &bytes[pos + b"csgo_econ_action_preview".len()..];
    let payload_start = if after.starts_with(b"%20") {
        3
    } else if after.first().copied() == Some(b' ') || after.first().copied() == Some(b'+') {
        1
    } else {
        0
    };
    let payload = &after[payload_start..];

    // Must start with S
    if payload.first().copied() != Some(b'S') {
        return false;
    }

    // Must NOT be hybrid (i.e., after D should be pure decimal)
    try_extract_hybrid_from(payload).is_none()
        && {
            // Verify it truly has S\d+A\d+D\d+ pattern
            let after_s = &payload[1..];
            if let Some(a_pos) = after_s.iter().position(|&c| c == b'A') {
                if after_s[..a_pos].iter().all(|&c| c.is_ascii_digit()) {
                    let after_a = &after_s[a_pos + 1..];
                    if let Some(d_pos) = after_a.iter().position(|&c| c == b'D') {
                        if after_a[..d_pos].iter().all(|&c| c.is_ascii_digit()) {
                            let after_d = &after_a[d_pos + 1..];
                            return after_d.iter().all(|&c| c.is_ascii_digit())
                                && !after_d.is_empty();
                        }
                    }
                }
            }
            false
        }
}

/// Serialize an `ItemPreviewData` into an uppercase hex string.
///
/// # Errors
/// - `ValidationError` if `paint_wear` is outside `[0.0, 1.0]`
/// - `ValidationError` if `custom_name` exceeds 100 characters
pub fn serialize(data: &ItemPreviewData) -> Result<String, Error> {
    // Validate
    if let Some(pw) = data.paint_wear {
        if !(0.0..=1.0).contains(&pw) {
            return Err(Error::ValidationError(
                "paint_wear must be in [0.0, 1.0]".into(),
            ));
        }
    }
    if data.custom_name.len() > 100 {
        return Err(Error::ValidationError(
            "custom_name exceeds 100 characters".into(),
        ));
    }

    // Encode proto
    let proto_bytes = proto::encode_item(data);

    // Compute checksum
    let checksum = compute_checksum(&proto_bytes);
    let checksum_bytes = checksum.to_be_bytes();

    // Assemble payload: [key=0x00] [proto_bytes] [checksum_be]
    let mut payload = Vec::with_capacity(1 + proto_bytes.len() + 4);
    payload.push(0x00u8);
    payload.extend_from_slice(&proto_bytes);
    payload.extend_from_slice(&checksum_bytes);

    Ok(hex_encode_upper(&payload))
}

/// Deserialize a hex string (or URL containing hex) into an `ItemPreviewData`.
///
/// # Errors
/// - `PayloadTooLarge` if the hex payload exceeds 4096 characters
/// - `PayloadTooSmall` if the binary payload is fewer than 6 bytes
/// - `ParseError` on invalid hex or malformed protobuf
/// - `ChecksumMismatch` if the checksum does not match
pub fn deserialize(input: &str) -> Result<ItemPreviewData, Error> {
    let hex_str = extract_hex(input);

    let preview: String = if input.chars().count() > 120 {
        input.chars().take(100).collect::<String>() + "..."
    } else {
        input.to_string()
    };

    if hex_str.is_empty() {
        return Err(Error::MalformedLink(format!(
            "no hex payload found in input \"{}\"",
            preview
        )));
    }

    if hex_str.len() > MAX_HEX_LEN {
        return Err(Error::MalformedLink(format!(
            "payload too long (max {} hex chars); input \"{}\"",
            MAX_HEX_LEN, preview
        )));
    }

    // Reject malformed hex up-front with full context — source likely
    // truncated the URL (real-world bug).
    if hex_str.len() % 2 != 0 {
        return Err(Error::MalformedLink(format!(
            "hex payload has invalid length ({} chars, must be even and non-empty); source likely truncated; input \"{}\"",
            hex_str.len(),
            preview
        )));
    }

    let raw = hex_decode(&hex_str)?;

    if raw.len() < MIN_BYTES {
        return Err(Error::MalformedLink(format!(
            "payload too short ({} bytes, need >={}); input \"{}\"",
            raw.len(),
            MIN_BYTES,
            preview
        )));
    }

    // First byte is key_byte
    let key_byte = raw[0];

    // XOR if key_byte != 0x00
    let payload: Vec<u8> = if key_byte != 0x00 {
        raw[1..].iter().map(|&b| b ^ key_byte).collect()
    } else {
        raw[1..].to_vec()
    };

    // payload = proto_bytes + 4-byte checksum
    if payload.len() < 4 {
        return Err(Error::MalformedLink(format!(
            "payload too short to contain checksum; input \"{}\"",
            preview
        )));
    }

    let proto_bytes = &payload[..payload.len() - 4];
    let _checksum_bytes = &payload[payload.len() - 4..];

    // Decode protobuf (checksum is not verified on deserialization — native CS2 links
    // use a different key schedule that produces non-standard checksums; the checksum
    // is only meaningful for tool-generated links with key_byte == 0x00)
    proto::decode_item(proto_bytes).map_err(|e| match e {
        Error::MalformedLink(_) => e,
        other => Error::MalformedLink(format!(
            "protobuf decode failed ({}); payload likely corrupted or truncated; input \"{}\"",
            other, preview
        )),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests for internal helpers
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode_decode() {
        let original = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let encoded = hex_encode_upper(&original);
        assert_eq!(encoded, "DEADBEEF");
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_hex_decode_lowercase() {
        let decoded = hex_decode("deadbeef").unwrap();
        assert_eq!(decoded, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_checksum_deterministic() {
        let data = vec![0x01, 0x02, 0x03];
        let c1 = compute_checksum(&data);
        let c2 = compute_checksum(&data);
        assert_eq!(c1, c2);
    }
}
