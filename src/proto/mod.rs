pub mod reader;
pub mod writer;

use crate::error::Error;
use crate::models::{ItemPreviewData, Sticker};
use reader::{ProtoReader, MAX_FIELDS};
use writer::ProtoWriter;

/// Encode a Sticker into protobuf bytes.
pub fn encode_sticker(sticker: &Sticker) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.write_uint32(1, sticker.slot);
    w.write_uint32(2, sticker.sticker_id);
    w.write_fixed32_opt(3, sticker.wear);
    w.write_fixed32_opt(4, sticker.scale);
    w.write_fixed32_opt(5, sticker.rotation);
    w.write_uint32(6, sticker.tint_id);
    w.write_fixed32_opt(7, sticker.offset_x);
    w.write_fixed32_opt(8, sticker.offset_y);
    w.write_fixed32_opt(9, sticker.offset_z);
    w.write_uint32(10, sticker.pattern);
    w.write_uint32_opt(11, sticker.highlight_reel);
    w.finish()
}

/// Encode an ItemPreviewData into protobuf bytes.
pub fn encode_item(data: &ItemPreviewData) -> Vec<u8> {
    let mut w = ProtoWriter::new();
    w.write_uint32(1, data.account_id);
    w.write_uint64(2, data.item_id);
    w.write_uint32(3, data.def_index);
    w.write_uint32(4, data.paint_index);
    w.write_uint32(5, data.rarity);
    w.write_uint32(6, data.quality);
    w.write_f32_as_varint_opt(7, data.paint_wear);
    w.write_uint32(8, data.paint_seed);
    w.write_uint32(9, data.kill_eater_score_type);
    w.write_uint32(10, data.kill_eater_value);
    w.write_string(11, &data.custom_name);
    for sticker in &data.stickers {
        let sticker_bytes = encode_sticker(sticker);
        w.write_bytes(12, &sticker_bytes);
    }
    w.write_uint32(13, data.inventory);
    w.write_uint32(14, data.origin);
    w.write_uint32(15, data.quest_id);
    w.write_uint32(16, data.drop_reason);
    w.write_uint32(17, data.music_index);
    w.write_int32(18, data.ent_index);
    w.write_uint32(19, data.pet_index);
    for keychain in &data.keychains {
        let kc_bytes = encode_sticker(keychain);
        w.write_bytes(20, &kc_bytes);
    }
    w.finish()
}

/// Decode a Sticker from protobuf bytes.
pub fn decode_sticker(data: &[u8]) -> Result<Sticker, Error> {
    let mut r = ProtoReader::new(data);
    let mut sticker = Sticker::default();
    let mut field_count = 0;
    while !r.is_empty() {
        field_count += 1;
        if field_count > MAX_FIELDS {
            return Err(Error::ParseError("too many fields in sticker".into()));
        }
        let (field_number, wire_type) = r.read_tag()?;
        match (field_number, wire_type) {
            (1, 0) => sticker.slot = r.read_uint32()?,
            (2, 0) => sticker.sticker_id = r.read_uint32()?,
            (3, 5) => sticker.wear = Some(r.read_fixed32_f32()?),
            (4, 5) => sticker.scale = Some(r.read_fixed32_f32()?),
            (5, 5) => sticker.rotation = Some(r.read_fixed32_f32()?),
            (6, 0) => sticker.tint_id = r.read_uint32()?,
            (7, 5) => sticker.offset_x = Some(r.read_fixed32_f32()?),
            (8, 5) => sticker.offset_y = Some(r.read_fixed32_f32()?),
            (9, 5) => sticker.offset_z = Some(r.read_fixed32_f32()?),
            (10, 0) => sticker.pattern = r.read_uint32()?,
            (11, 0) => sticker.highlight_reel = Some(r.read_uint32()?),
            _ => r.skip_field(wire_type)?,
        }
    }
    Ok(sticker)
}

/// Decode an ItemPreviewData from protobuf bytes.
pub fn decode_item(data: &[u8]) -> Result<ItemPreviewData, Error> {
    let mut r = ProtoReader::new(data);
    let mut item = ItemPreviewData::default();
    let mut field_count = 0;
    while !r.is_empty() {
        field_count += 1;
        if field_count > MAX_FIELDS {
            return Err(Error::ParseError("too many fields in item".into()));
        }
        let (field_number, wire_type) = r.read_tag()?;
        match (field_number, wire_type) {
            (1, 0) => item.account_id = r.read_uint32()?,
            (2, 0) => item.item_id = r.read_uint64()?,
            (3, 0) => item.def_index = r.read_uint32()?,
            (4, 0) => item.paint_index = r.read_uint32()?,
            (5, 0) => item.rarity = r.read_uint32()?,
            (6, 0) => item.quality = r.read_uint32()?,
            (7, 0) => {
                let bits = r.read_uint32()?;
                item.paint_wear = Some(f32::from_bits(bits));
            }
            (8, 0) => item.paint_seed = r.read_uint32()?,
            (9, 0) => item.kill_eater_score_type = r.read_uint32()?,
            (10, 0) => item.kill_eater_value = r.read_uint32()?,
            (11, 2) => item.custom_name = r.read_string()?,
            (12, 2) => {
                let bytes = r.read_bytes()?;
                let sticker = decode_sticker(bytes)?;
                item.stickers.push(sticker);
            }
            (13, 0) => item.inventory = r.read_uint32()?,
            (14, 0) => item.origin = r.read_uint32()?,
            (15, 0) => item.quest_id = r.read_uint32()?,
            (16, 0) => item.drop_reason = r.read_uint32()?,
            (17, 0) => item.music_index = r.read_uint32()?,
            (18, 0) => item.ent_index = r.read_int32()?,
            (19, 0) => item.pet_index = r.read_uint32()?,
            (20, 2) => {
                let bytes = r.read_bytes()?;
                let keychain = decode_sticker(bytes)?;
                item.keychains.push(keychain);
            }
            _ => r.skip_field(wire_type)?,
        }
    }
    Ok(item)
}
