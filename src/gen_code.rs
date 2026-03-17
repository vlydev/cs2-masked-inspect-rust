//! Gen code utilities for CS2 inspect links.
//!
//! Gen codes are space-separated command strings used on community servers:
//! ```text
//! !gen {defindex} {paintindex} {paintseed} {paintwear}
//! !gen ... {s0_id} {s0_wear} {s1_id} {s1_wear} ... {s4_id} {s4_wear} [{kc_id} {kc_wear} ...]
//! ```
//!
//! Stickers are always padded to 5 slot pairs. Keychains follow without padding.

use crate::error::Error;
use crate::inspect_link::serialize;
use crate::models::{ItemPreviewData, Sticker};

/// Steam inspect URL prefix.
pub const INSPECT_BASE: &str =
    "steam://rungame/730/76561202255233023/+csgo_econ_action_preview%20";

/// Format a float32, stripping trailing zeros (max 8 decimal places).
fn format_float(v: f32) -> String {
    let s = format!("{:.8}", v);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    if s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}

fn serialize_sticker_pairs(stickers: &[Sticker], pad_to: Option<usize>) -> Vec<String> {
    let mut result = Vec::new();
    let filtered: Vec<&Sticker> = stickers.iter().filter(|s| s.sticker_id != 0).collect();

    if let Some(n) = pad_to {
        use std::collections::HashMap;
        let slot_map: HashMap<u32, &Sticker> = filtered.iter().map(|s| (s.slot, *s)).collect();
        for slot in 0..n {
            if let Some(s) = slot_map.get(&(slot as u32)) {
                let wear = s.wear.unwrap_or(0.0);
                result.push(s.sticker_id.to_string());
                result.push(format_float(wear));
            } else {
                result.push("0".to_string());
                result.push("0".to_string());
            }
        }
    } else {
        let mut sorted = filtered.clone();
        sorted.sort_by_key(|s| s.slot);
        for s in sorted {
            let wear = s.wear.unwrap_or(0.0);
            result.push(s.sticker_id.to_string());
            result.push(format_float(wear));
        }
    }

    result
}

/// Convert an [`ItemPreviewData`] to a gen code string.
///
/// The prefix is typically `"!gen"` or `"!g"`. Pass `""` for no prefix.
///
/// # Example
/// ```
/// use cs2_masked_inspect::{ItemPreviewData, to_gen_code};
///
/// let mut item = ItemPreviewData::default();
/// item.def_index = 7;
/// item.paint_index = 474;
/// item.paint_seed = 306;
/// item.paint_wear = Some(0.22540508);
///
/// let code = to_gen_code(&item, "!gen");
/// assert_eq!(code, "!gen 7 474 306 0.22540508");
/// ```
pub fn to_gen_code(item: &ItemPreviewData, prefix: &str) -> String {
    let wear_str = item
        .paint_wear
        .map(format_float)
        .unwrap_or_else(|| "0".to_string());
    let mut parts = vec![
        item.def_index.to_string(),
        item.paint_index.to_string(),
        item.paint_seed.to_string(),
        wear_str,
    ];

    let has_stickers = item.stickers.iter().any(|s| s.sticker_id != 0);
    let has_keychains = item.keychains.iter().any(|s| s.sticker_id != 0);
    if has_stickers || has_keychains {
        parts.extend(serialize_sticker_pairs(&item.stickers, Some(5)));
        parts.extend(serialize_sticker_pairs(&item.keychains, None));
    }

    let payload = parts.join(" ");
    if prefix.is_empty() {
        payload
    } else {
        format!("{} {}", prefix, payload)
    }
}

/// Options for [`generate`].
#[derive(Debug, Default)]
pub struct GenerateOptions {
    pub rarity: u32,
    pub quality: u32,
    pub stickers: Vec<Sticker>,
    pub keychains: Vec<Sticker>,
}

/// Generate a full Steam inspect URL from item parameters.
///
/// Returns the full `steam://rungame/...` URL.
///
/// # Errors
/// Returns an error if serialization fails (e.g., `paint_wear` outside `[0.0, 1.0]`).
///
/// # Example
/// ```
/// use cs2_masked_inspect::generate;
///
/// let url = generate(7, 474, 306, 0.22540508, Default::default()).unwrap();
/// assert!(url.starts_with("steam://rungame/730"));
/// ```
pub fn generate(
    def_index: u32,
    paint_index: u32,
    paint_seed: u32,
    paint_wear: f32,
    opts: GenerateOptions,
) -> Result<String, Error> {
    let data = ItemPreviewData {
        def_index,
        paint_index,
        paint_seed,
        paint_wear: Some(paint_wear),
        rarity: opts.rarity,
        quality: opts.quality,
        stickers: opts.stickers,
        keychains: opts.keychains,
        ..Default::default()
    };
    let hex = serialize(&data)?;
    Ok(format!("{}{}", INSPECT_BASE, hex))
}

/// Generate a gen code string from an existing CS2 inspect link.
///
/// Deserializes the inspect link and converts the item data to gen code format.
///
/// # Example
/// ```
/// use cs2_masked_inspect::gen_code_from_link;
///
/// let code = gen_code_from_link("steam://rungame/730/76561202255233023/+csgo_econ_action_preview%20001A07A8", "!gen");
/// ```
pub fn gen_code_from_link(hex_or_url: &str, prefix: &str) -> Result<String, Error> {
    let item = crate::deserialize(hex_or_url)?;
    Ok(to_gen_code(&item, prefix))
}

/// Parse a gen code string into an [`ItemPreviewData`].
///
/// Accepts codes like:
/// - `"!gen 7 474 306 0.22540508"`
/// - `"7 941 2 0.22540508 0 0 0 0 7203 0 0 0 0 0 36 0"`
///
/// # Errors
/// Returns an error if the code has fewer than 4 tokens.
///
/// # Example
/// ```
/// use cs2_masked_inspect::parse_gen_code;
///
/// let item = parse_gen_code("!gen 7 474 306 0.22540508").unwrap();
/// assert_eq!(item.def_index, 7);
/// ```
pub fn parse_gen_code(gen_code: &str) -> Result<ItemPreviewData, Error> {
    let mut tokens: Vec<&str> = gen_code.trim().split_whitespace().collect();

    if tokens.first().map(|t| t.starts_with('!')).unwrap_or(false) {
        tokens.remove(0);
    }

    if tokens.len() < 4 {
        return Err(Error::ParseError(format!(
            "gen code must have at least 4 tokens, got: {:?}",
            gen_code
        )));
    }

    let def_index: u32 = tokens[0]
        .parse()
        .map_err(|e| Error::ParseError(format!("invalid defindex: {}", e)))?;
    let paint_index: u32 = tokens[1]
        .parse()
        .map_err(|e| Error::ParseError(format!("invalid paintindex: {}", e)))?;
    let paint_seed: u32 = tokens[2]
        .parse()
        .map_err(|e| Error::ParseError(format!("invalid paintseed: {}", e)))?;
    let paint_wear: f32 = tokens[3]
        .parse()
        .map_err(|e| Error::ParseError(format!("invalid paintwear: {}", e)))?;

    let rest = &tokens[4..];
    let mut stickers = Vec::new();
    let mut keychains = Vec::new();

    if rest.len() >= 10 {
        let sticker_tokens = &rest[..10];
        for slot in 0..5usize {
            let sid: u32 = sticker_tokens[slot * 2].parse().unwrap_or(0);
            let wear: f32 = sticker_tokens[slot * 2 + 1].parse().unwrap_or(0.0);
            if sid != 0 {
                stickers.push(Sticker {
                    slot: slot as u32,
                    sticker_id: sid,
                    wear: Some(wear),
                    ..Default::default()
                });
            }
        }
        let rest = &rest[10..];
        let mut i = 0;
        while i + 1 < rest.len() {
            let sid: u32 = rest[i].parse().unwrap_or(0);
            let wear: f32 = rest[i + 1].parse().unwrap_or(0.0);
            if sid != 0 {
                keychains.push(Sticker {
                    slot: (i / 2) as u32,
                    sticker_id: sid,
                    wear: Some(wear),
                    ..Default::default()
                });
            }
            i += 2;
        }
    } else {
        let mut i = 0;
        while i + 1 < rest.len() {
            let sid: u32 = rest[i].parse().unwrap_or(0);
            let wear: f32 = rest[i + 1].parse().unwrap_or(0.0);
            if sid != 0 {
                keychains.push(Sticker {
                    slot: (i / 2) as u32,
                    sticker_id: sid,
                    wear: Some(wear),
                    ..Default::default()
                });
            }
            i += 2;
        }
    }

    Ok(ItemPreviewData {
        def_index,
        paint_index,
        paint_seed,
        paint_wear: Some(paint_wear),
        stickers,
        keychains,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Sticker;

    #[test]
    fn test_to_gen_code_basic() {
        let item = ItemPreviewData {
            def_index: 7,
            paint_index: 474,
            paint_seed: 306,
            paint_wear: Some(0.22540508),
            ..Default::default()
        };
        assert_eq!(to_gen_code(&item, "!gen"), "!gen 7 474 306 0.22540508");
    }

    #[test]
    fn test_to_gen_code_with_sticker_and_keychain() {
        let item = ItemPreviewData {
            def_index: 7,
            paint_index: 941,
            paint_seed: 2,
            paint_wear: Some(0.22540508),
            stickers: vec![Sticker {
                slot: 2,
                sticker_id: 7203,
                wear: Some(0.0),
                ..Default::default()
            }],
            keychains: vec![Sticker {
                slot: 0,
                sticker_id: 36,
                wear: Some(0.0),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(
            to_gen_code(&item, "!g"),
            "!g 7 941 2 0.22540508 0 0 0 0 7203 0 0 0 0 0 36 0"
        );
    }

    #[test]
    fn test_parse_gen_code_basic() {
        let item = parse_gen_code("!gen 7 474 306 0.22540508").unwrap();
        assert_eq!(item.def_index, 7);
        assert_eq!(item.paint_index, 474);
        assert_eq!(item.paint_seed, 306);
        assert!((item.paint_wear.unwrap() - 0.22540508f32).abs() < 1e-5);
    }

    #[test]
    fn test_parse_gen_code_with_sticker_and_keychain() {
        let item =
            parse_gen_code("!g 7 941 2 0.22540508 0 0 0 0 7203 0 0 0 0 0 36 0").unwrap();
        assert_eq!(item.stickers.len(), 1);
        assert_eq!(item.stickers[0].sticker_id, 7203);
        assert_eq!(item.keychains.len(), 1);
        assert_eq!(item.keychains[0].sticker_id, 36);
    }

    #[test]
    fn test_gen_code_from_link_from_hex() {
        let url = generate(7, 474, 306, 0.22540508, Default::default()).unwrap();
        let hex = url.trim_start_matches(INSPECT_BASE);
        let code = gen_code_from_link(hex, "!gen").unwrap();
        assert!(code.starts_with("!gen 7 474 306"), "got: {}", code);
    }

    #[test]
    fn test_gen_code_from_link_from_full_url() {
        let url = generate(7, 474, 306, 0.22540508, Default::default()).unwrap();
        let code = gen_code_from_link(&url, "!gen").unwrap();
        assert!(code.starts_with("!gen 7 474 306"), "got: {}", code);
    }

    #[test]
    fn test_generate_roundtrip() {
        let url = generate(7, 474, 306, 0.22540508, Default::default()).unwrap();
        assert!(url.starts_with(INSPECT_BASE));
        let hex = url.trim_start_matches(INSPECT_BASE);
        let item = crate::deserialize(hex).unwrap();
        assert_eq!(item.def_index, 7);
        assert_eq!(item.paint_index, 474);
    }
}
