use crate::error::Error;

/// Maximum number of fields to read per message (prevents infinite loops on malformed input).
pub const MAX_FIELDS: usize = 100;

/// Protobuf binary reader.
pub struct ProtoReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ProtoReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Read a single byte.
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= self.data.len() {
            return Err(Error::ParseError("unexpected end of data".into()));
        }
        let byte = self.data[self.pos];
        self.pos += 1;
        Ok(byte)
    }

    /// Read a varint (base-128 encoding), returning u64.
    pub fn read_varint(&mut self) -> Result<u64, Error> {
        let mut result: u64 = 0;
        let mut shift = 0u32;
        loop {
            if shift >= 64 {
                return Err(Error::ParseError("varint too long".into()));
            }
            let byte = self.read_byte()?;
            result |= ((byte & 0x7F) as u64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Read a tag and return (field_number, wire_type).
    pub fn read_tag(&mut self) -> Result<(u32, u8), Error> {
        let tag = self.read_varint()? as u32;
        let field_number = tag >> 3;
        let wire_type = (tag & 0x07) as u8;
        Ok((field_number, wire_type))
    }

    /// Read a uint32 (varint).
    pub fn read_uint32(&mut self) -> Result<u32, Error> {
        Ok(self.read_varint()? as u32)
    }

    /// Read a uint64 (varint).
    pub fn read_uint64(&mut self) -> Result<u64, Error> {
        self.read_varint()
    }

    /// Read an int32 (varint, sign-extended from 64-bit).
    pub fn read_int32(&mut self) -> Result<i32, Error> {
        Ok(self.read_varint()? as i32)
    }

    /// Read a length-delimited field (wire type 2) and return the bytes.
    pub fn read_bytes(&mut self) -> Result<&'a [u8], Error> {
        let len = self.read_varint()? as usize;
        if self.pos + len > self.data.len() {
            return Err(Error::ParseError("length-delimited field exceeds data".into()));
        }
        let bytes = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(bytes)
    }

    /// Read a length-delimited string (wire type 2).
    pub fn read_string(&mut self) -> Result<String, Error> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| Error::ParseError("invalid UTF-8 in string field".into()))
    }

    /// Read a fixed32 LE (wire type 5), returning bits as u32.
    pub fn read_fixed32(&mut self) -> Result<u32, Error> {
        if self.pos + 4 > self.data.len() {
            return Err(Error::ParseError("not enough bytes for fixed32".into()));
        }
        let b0 = self.data[self.pos] as u32;
        let b1 = self.data[self.pos + 1] as u32;
        let b2 = self.data[self.pos + 2] as u32;
        let b3 = self.data[self.pos + 3] as u32;
        self.pos += 4;
        Ok(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    /// Read a fixed32 LE as f32 (wire type 5).
    pub fn read_fixed32_f32(&mut self) -> Result<f32, Error> {
        let bits = self.read_fixed32()?;
        Ok(f32::from_bits(bits))
    }

    /// Skip a field based on wire type.
    pub fn skip_field(&mut self, wire_type: u8) -> Result<(), Error> {
        match wire_type {
            0 => {
                self.read_varint()?;
            }
            1 => {
                // 64-bit
                if self.pos + 8 > self.data.len() {
                    return Err(Error::ParseError("not enough bytes to skip 64-bit field".into()));
                }
                self.pos += 8;
            }
            2 => {
                let len = self.read_varint()? as usize;
                if self.pos + len > self.data.len() {
                    return Err(Error::ParseError("not enough bytes to skip length-delimited field".into()));
                }
                self.pos += len;
            }
            5 => {
                // 32-bit
                if self.pos + 4 > self.data.len() {
                    return Err(Error::ParseError("not enough bytes to skip 32-bit field".into()));
                }
                self.pos += 4;
            }
            _ => {
                return Err(Error::ParseError(format!("unknown wire type: {}", wire_type)));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_varint() {
        let data = vec![0x01];
        let mut r = ProtoReader::new(&data);
        assert_eq!(r.read_varint().unwrap(), 1);

        let data = vec![0xAC, 0x02];
        let mut r = ProtoReader::new(&data);
        assert_eq!(r.read_varint().unwrap(), 300);
    }

    #[test]
    fn test_read_tag() {
        // Field 1, wire type 0 → tag = (1 << 3) | 0 = 0x08
        let data = vec![0x08];
        let mut r = ProtoReader::new(&data);
        let (field, wire) = r.read_tag().unwrap();
        assert_eq!(field, 1);
        assert_eq!(wire, 0);
    }
}
