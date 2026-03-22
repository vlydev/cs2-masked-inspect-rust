/// Represents a sticker or keychain applied to a CS2 item.
#[derive(Debug, Clone, PartialEq)]
pub struct Sticker {
    pub slot: u32,
    pub sticker_id: u32,
    /// Wear value, stored as fixed32 LE in proto (wire type 5)
    pub wear: Option<f32>,
    pub scale: Option<f32>,
    pub rotation: Option<f32>,
    pub tint_id: u32,
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
    pub offset_z: Option<f32>,
    pub pattern: u32,
    /// highlight_reel: omitted if None
    pub highlight_reel: Option<u32>,
    /// paint_kit (proto field 12, varint): actual variant ID for sticker slabs; omitted if None
    pub paint_kit: Option<u32>,
}

impl Default for Sticker {
    fn default() -> Self {
        Self {
            slot: 0,
            sticker_id: 0,
            wear: None,
            scale: None,
            rotation: None,
            tint_id: 0,
            offset_x: None,
            offset_y: None,
            offset_z: None,
            pattern: 0,
            highlight_reel: None,
            paint_kit: None,
        }
    }
}

/// CEconItemPreviewDataBlock — the primary data structure representing a CS2 item preview.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemPreviewData {
    pub account_id: u32,
    pub item_id: u64,
    pub def_index: u32,
    pub paint_index: u32,
    pub rarity: u32,
    pub quality: u32,
    /// PaintWear stored as u32 varint whose bits are IEEE 754 f32.
    /// None means field is omitted in proto.
    pub paint_wear: Option<f32>,
    pub paint_seed: u32,
    pub kill_eater_score_type: u32,
    pub kill_eater_value: u32,
    pub custom_name: String,
    pub stickers: Vec<Sticker>,
    pub inventory: u32,
    pub origin: u32,
    pub quest_id: u32,
    pub drop_reason: u32,
    pub music_index: u32,
    pub ent_index: i32,
    pub pet_index: u32,
    pub keychains: Vec<Sticker>,
}

impl Default for ItemPreviewData {
    fn default() -> Self {
        Self {
            account_id: 0,
            item_id: 0,
            def_index: 0,
            paint_index: 0,
            rarity: 0,
            quality: 0,
            paint_wear: None,
            paint_seed: 0,
            kill_eater_score_type: 0,
            kill_eater_value: 0,
            custom_name: String::new(),
            stickers: Vec::new(),
            inventory: 0,
            origin: 0,
            quest_id: 0,
            drop_reason: 0,
            music_index: 0,
            ent_index: 0,
            pet_index: 0,
            keychains: Vec::new(),
        }
    }
}
