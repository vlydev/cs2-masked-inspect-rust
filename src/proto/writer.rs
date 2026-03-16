/// Protobuf binary writer (proto3 semantics).
/// Omits fields with zero/default values.
pub struct ProtoWriter {
    buf: Vec<u8>,
}

impl ProtoWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    /// Write a varint (base-128 encoding).
    pub fn write_varint(&mut self, mut value: u64) {
        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;
            if value == 0 {
                self.buf.push(byte);
                break;
            } else {
                self.buf.push(byte | 0x80);
            }
        }
    }

    /// Write the tag: (field_number << 3) | wire_type
    pub fn write_tag(&mut self, field_number: u32, wire_type: u8) {
        self.write_varint(((field_number as u64) << 3) | wire_type as u64);
    }

    /// Write a uint32 field (wire type 0, varint). Omit if value == 0.
    pub fn write_uint32(&mut self, field_number: u32, value: u32) {
        if value == 0 {
            return;
        }
        self.write_tag(field_number, 0);
        self.write_varint(value as u64);
    }

    /// Write a uint64 field (wire type 0, varint). Omit if value == 0.
    pub fn write_uint64(&mut self, field_number: u32, value: u64) {
        if value == 0 {
            return;
        }
        self.write_tag(field_number, 0);
        self.write_varint(value);
    }

    /// Write an int32 field (wire type 0, varint, zigzag not used in proto3 for int32).
    /// Uses standard varint encoding (cast to i64 then u64 for negative numbers).
    /// Omit if value == 0.
    pub fn write_int32(&mut self, field_number: u32, value: i32) {
        if value == 0 {
            return;
        }
        self.write_tag(field_number, 0);
        // Negative i32 → 10 bytes as u64 (sign-extended)
        self.write_varint(value as i64 as u64);
    }

    /// Write a string field (wire type 2, length-delimited). Omit if empty.
    pub fn write_string(&mut self, field_number: u32, value: &str) {
        if value.is_empty() {
            return;
        }
        let bytes = value.as_bytes();
        self.write_tag(field_number, 2);
        self.write_varint(bytes.len() as u64);
        self.buf.extend_from_slice(bytes);
    }

    /// Write a bytes/embedded message field (wire type 2). Omit if empty.
    pub fn write_bytes(&mut self, field_number: u32, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        self.write_tag(field_number, 2);
        self.write_varint(data.len() as u64);
        self.buf.extend_from_slice(data);
    }

    /// Write a fixed32 LE field (wire type 5). Omit if None.
    pub fn write_fixed32_opt(&mut self, field_number: u32, value: Option<f32>) {
        if let Some(v) = value {
            self.write_tag(field_number, 5);
            let bits = v.to_bits();
            self.buf.push((bits & 0xFF) as u8);
            self.buf.push(((bits >> 8) & 0xFF) as u8);
            self.buf.push(((bits >> 16) & 0xFF) as u8);
            self.buf.push(((bits >> 24) & 0xFF) as u8);
        }
    }

    /// Write a varint-encoded f32 (bits as u32 varint). Omit if None.
    /// Used for paint_wear field in ItemPreviewData.
    pub fn write_f32_as_varint_opt(&mut self, field_number: u32, value: Option<f32>) {
        if let Some(v) = value {
            let bits = v.to_bits();
            if bits == 0 {
                // zero f32 — still write it to preserve the field
                // (caller should handle omission logic)
                return;
            }
            self.write_tag(field_number, 0);
            self.write_varint(bits as u64);
        }
    }

    /// Write a varint-encoded optional u32. Omit if None.
    pub fn write_uint32_opt(&mut self, field_number: u32, value: Option<u32>) {
        if let Some(v) = value {
            if v == 0 {
                return;
            }
            self.write_tag(field_number, 0);
            self.write_varint(v as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_encoding() {
        let mut w = ProtoWriter::new();
        w.write_varint(1);
        assert_eq!(w.finish(), vec![0x01]);

        let mut w = ProtoWriter::new();
        w.write_varint(300);
        // 300 = 0b100101100, varint = 0b10101100 0b00000010 = AC 02
        assert_eq!(w.finish(), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_uint32_omits_zero() {
        let mut w = ProtoWriter::new();
        w.write_uint32(1, 0);
        assert_eq!(w.finish(), vec![]);
    }
}
