/// CRC32 implementation using IEEE 802.3 polynomial 0xEDB88320 (reversed/reflected).
/// This is the standard CRC32 used in zlib, PNG, Ethernet, etc.

const POLYNOMIAL: u32 = 0xEDB88320;

/// Build the CRC32 lookup table at runtime.
fn make_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0u32..256 {
        let mut crc = i;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
        table[i as usize] = crc;
    }
    table
}

/// Compute CRC32 checksum of the given bytes.
pub fn crc32(data: &[u8]) -> u32 {
    let table = make_table();
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[idx];
    }
    crc ^ 0xFFFF_FFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_known_value() {
        // CRC32 of "123456789" = 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn test_crc32_empty() {
        // CRC32 of empty = 0x00000000
        assert_eq!(crc32(b""), 0x00000000);
    }
}
