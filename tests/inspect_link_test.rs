use cs2_masked_inspect::{deserialize, is_classic, is_masked, serialize, Error, ItemPreviewData, Sticker};

// ─────────────────────────────────────────────────────────────────────────────
// Test vectors
// ─────────────────────────────────────────────────────────────────────────────

const NATIVE_HEX: &str = "E3F3367440334DE2FBE4C345E0CBE0D3E7DB6943400AE0A379E481ECEBE2F36F\
                           D9DE2BDB515EA6E30D74D981ECEBE3F37BCBDE640D475DA6E35EFCD881ECEBE3\
                           F359D5DE37E9D75DA6436DD3DD81ECEBE3F366DCDE3F8F9BDDA69B43B6DE81EC\
                           EBE3F33BC8DEBB1CA3DFA623F7DDDF8B71E293EBFD43382B";
// ItemId=46876117973, DefIndex=7, PaintIndex=422, PaintSeed=922, PaintWear≈0.04121, Rarity=3, Quality=4
// Stickers: [7436, 5144, 6970, 8069, 5592]

const TOOL_HEX: &str = "00183C20B803280538E9A3C5DD0340E102C246A0D1";
// DefIndex=60, PaintIndex=440, PaintSeed=353, PaintWear≈0.005411, Rarity=5

const HYBRID_URL: &str = "steam://rungame/730/76561202255233023/+csgo_econ_action_preview%20S76561199323320483A50075495125D00183C20B803280538E9A3C5DD0340E102C246A0D1";

const CLASSIC_URL: &str = "steam://rungame/730/76561202255233023/+csgo_econ_action_preview%20S76561199842063946A49749521570D2751293026650298712";

const CSFLOAT_A: &str = "00180720DA03280638FBEE88F90340B2026BC03C96";
// DefIndex=7, PaintIndex=474, PaintSeed=306, Rarity=6, PaintWear≈0.6337

const CSFLOAT_B: &str = "00180720C80A280638A4E1F5FB03409A0562040800104C62040801104C62040802104C62040803104C6D4F5E30";
// PaintIndex=1352, PaintWear≈0.99, 4 stickers each sticker_id=76

const CSFLOAT_C: &str = "A2B2A2BA69A882A28AA192AECAA2D2B700A3A5AAA2B286FA7BA0D684BE72";
// DefIndex=1355, Rarity=3, Quality=12, 1 keychain with highlight_reel=345, paint_wear=None

// Sticker slab test vectors (defIndex=1355, quality=8)
// keychains[0].sticker_id=37 (placeholder), keychains[0].paint_kit=variant ID
const SLAB_A: &str = "918191895A9BB191B994A199F991E191339096999181B4F149A98D5C0889";
// defIndex=1355, rarity=5, quality=8, keychains[0].sticker_id=37, keychains[0].paint_kit=7256
const SLAB_B: &str = "CBDBCBD300C1EBCBE3C8FBC3A3CBBBCB69CACCC3CBDBEEAB58C9B8B67C83";
// defIndex=1355, rarity=3, quality=8, keychains[0].sticker_id=37, keychains[0].paint_kit=275

// ─────────────────────────────────────────────────────────────────────────────
// 1. Deserialize NATIVE_HEX
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_deserialize_native_hex_item_id() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.item_id, 46876117973);
}

#[test]
fn test_deserialize_native_hex_def_index() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.def_index, 7);
}

#[test]
fn test_deserialize_native_hex_paint_index() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.paint_index, 422);
}

#[test]
fn test_deserialize_native_hex_paint_seed() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.paint_seed, 922);
}

#[test]
fn test_deserialize_native_hex_paint_wear() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    let pw = item.paint_wear.expect("paint_wear should be Some");
    assert!((pw - 0.04121).abs() < 0.001, "paint_wear={}", pw);
}

#[test]
fn test_deserialize_native_hex_rarity() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.rarity, 3);
}

#[test]
fn test_deserialize_native_hex_quality() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.quality, 4);
}

#[test]
fn test_deserialize_native_hex_sticker_count() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    assert_eq!(item.stickers.len(), 5);
}

#[test]
fn test_deserialize_native_hex_sticker_ids() {
    let item = deserialize(NATIVE_HEX).expect("should deserialize NATIVE_HEX");
    let ids: Vec<u32> = item.stickers.iter().map(|s| s.sticker_id).collect();
    // Expected sticker IDs in order of slots
    let expected_ids = vec![7436u32, 5144, 6970, 8069, 5592];
    // Sort both since order might vary by slot
    let mut ids_sorted = ids.clone();
    ids_sorted.sort();
    let mut expected_sorted = expected_ids.clone();
    expected_sorted.sort();
    assert_eq!(ids_sorted, expected_sorted, "sticker ids: {:?}", ids);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Deserialize TOOL_HEX
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_deserialize_tool_hex_def_index() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    assert_eq!(item.def_index, 60);
}

#[test]
fn test_deserialize_tool_hex_paint_index() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    assert_eq!(item.paint_index, 440);
}

#[test]
fn test_deserialize_tool_hex_paint_seed() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    assert_eq!(item.paint_seed, 353);
}

#[test]
fn test_deserialize_tool_hex_paint_wear() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    let pw = item.paint_wear.expect("paint_wear should be Some");
    assert!((pw - 0.005411).abs() < 0.0001, "paint_wear={}", pw);
}

#[test]
fn test_deserialize_tool_hex_rarity() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    assert_eq!(item.rarity, 5);
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Serialize
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_serialize_produces_tool_hex() {
    // First deserialize to get the data, then serialize back and compare
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    let serialized = serialize(&item).expect("should serialize");
    assert_eq!(
        serialized.to_uppercase(),
        TOOL_HEX.to_uppercase(),
        "serialized output mismatch"
    );
}

#[test]
fn test_serialize_output_uppercase() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    let serialized = serialize(&item).expect("should serialize");
    assert_eq!(serialized, serialized.to_uppercase(), "output should be uppercase");
}

#[test]
fn test_serialize_starts_with_00() {
    let item = deserialize(TOOL_HEX).expect("should deserialize TOOL_HEX");
    let serialized = serialize(&item).expect("should serialize");
    assert!(serialized.starts_with("00"), "tool-generated link should start with 00");
}

#[test]
fn test_serialize_minimum_length() {
    let item = ItemPreviewData {
        def_index: 1,
        ..Default::default()
    };
    let serialized = serialize(&item).expect("should serialize");
    // At minimum: 1 byte key + some proto bytes + 4 byte checksum = at least 6 bytes = 12 hex chars
    assert!(serialized.len() >= 12, "serialized length={}", serialized.len());
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Round-trip tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_roundtrip_basic_fields() {
    let original = ItemPreviewData {
        def_index: 7,
        paint_index: 422,
        paint_seed: 922,
        paint_wear: Some(0.04121_f32),
        rarity: 3,
        quality: 4,
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.def_index, original.def_index);
    assert_eq!(decoded.paint_index, original.paint_index);
    assert_eq!(decoded.paint_seed, original.paint_seed);
    assert_eq!(decoded.rarity, original.rarity);
    assert_eq!(decoded.quality, original.quality);
}

#[test]
fn test_roundtrip_paint_wear_f32_precision() {
    let pw = 0.123456789_f32;
    let original = ItemPreviewData {
        paint_wear: Some(pw),
        def_index: 1,
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    let recovered = decoded.paint_wear.expect("paint_wear should be Some");
    // f32 round-trip should be exact bit-for-bit
    assert_eq!(recovered.to_bits(), pw.to_bits(), "paint_wear bits mismatch");
}

#[test]
fn test_roundtrip_large_item_id() {
    let original = ItemPreviewData {
        item_id: 46876117973u64,
        def_index: 7,
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.item_id, original.item_id);
}

#[test]
fn test_roundtrip_stickers() {
    let original = ItemPreviewData {
        def_index: 7,
        stickers: vec![
            Sticker {
                slot: 0,
                sticker_id: 7436,
                wear: Some(0.1_f32),
                ..Default::default()
            },
            Sticker {
                slot: 1,
                sticker_id: 5144,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.stickers.len(), 2);
    assert_eq!(decoded.stickers[0].sticker_id, 7436);
    assert_eq!(decoded.stickers[1].sticker_id, 5144);
    let wear = decoded.stickers[0].wear.expect("wear should be Some");
    assert_eq!(wear.to_bits(), 0.1_f32.to_bits());
}

#[test]
fn test_roundtrip_keychains() {
    let original = ItemPreviewData {
        def_index: 1355,
        keychains: vec![Sticker {
            slot: 0,
            sticker_id: 999,
            highlight_reel: Some(345),
            ..Default::default()
        }],
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.keychains.len(), 1);
    assert_eq!(decoded.keychains[0].sticker_id, 999);
    assert_eq!(decoded.keychains[0].highlight_reel, Some(345));
}

#[test]
fn test_roundtrip_custom_name() {
    let original = ItemPreviewData {
        def_index: 7,
        custom_name: "My Knife".to_string(),
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.custom_name, "My Knife");
}

#[test]
fn test_roundtrip_none_paint_wear() {
    let original = ItemPreviewData {
        def_index: 1355,
        paint_wear: None,
        rarity: 3,
        quality: 12,
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.paint_wear, None);
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. is_masked / is_classic
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_masked_bare_tool_hex() {
    assert!(is_masked(TOOL_HEX));
}

#[test]
fn test_is_masked_bare_native_hex() {
    assert!(is_masked(NATIVE_HEX));
}

#[test]
fn test_is_masked_hybrid_url() {
    assert!(is_masked(HYBRID_URL));
}

#[test]
fn test_is_classic_classic_url() {
    assert!(is_classic(CLASSIC_URL));
}

#[test]
fn test_is_masked_classic_url_false() {
    assert!(!is_masked(CLASSIC_URL));
}

#[test]
fn test_is_classic_tool_hex_false() {
    assert!(!is_classic(TOOL_HEX));
}

#[test]
fn test_is_classic_hybrid_url_false() {
    assert!(!is_classic(HYBRID_URL));
}

#[test]
fn test_is_masked_csfloat_a() {
    assert!(is_masked(CSFLOAT_A));
}

#[test]
fn test_is_masked_csfloat_b() {
    assert!(is_masked(CSFLOAT_B));
}

#[test]
fn test_is_classic_random_string_false() {
    assert!(!is_classic("not a link at all"));
    assert!(!is_masked("not a link at all"));
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. Defensive / error cases
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_deserialize_payload_too_large() {
    // > 4096 hex chars
    let huge = "AB".repeat(2049); // 4098 chars
    let result = deserialize(&huge);
    assert_eq!(result, Err(Error::PayloadTooLarge));
}

#[test]
fn test_deserialize_payload_too_small() {
    // < 6 bytes = < 12 hex chars
    // 4 bytes = 8 hex chars
    let small = "00AABBCC"; // 4 bytes, after key byte + checksum = 0 proto bytes
    let result = deserialize(small);
    // This should fail — either too small or checksum mismatch
    assert!(result.is_err(), "expected error for small payload");
}

#[test]
fn test_deserialize_truly_too_small() {
    // 5 bytes total (1 key + 4 checksum = 0 proto bytes) is borderline
    // Let's try 3 bytes = 6 hex chars
    let tiny = "00AABB";
    let result = deserialize(tiny);
    assert!(result.is_err());
}

#[test]
fn test_serialize_paint_wear_out_of_range_negative() {
    let data = ItemPreviewData {
        paint_wear: Some(-0.1),
        ..Default::default()
    };
    let result = serialize(&data);
    assert_eq!(result, Err(Error::ValidationError("paint_wear must be in [0.0, 1.0]".into())));
}

#[test]
fn test_serialize_paint_wear_out_of_range_above_one() {
    let data = ItemPreviewData {
        paint_wear: Some(1.1),
        ..Default::default()
    };
    let result = serialize(&data);
    assert!(matches!(result, Err(Error::ValidationError(_))));
}

#[test]
fn test_serialize_custom_name_too_long() {
    let data = ItemPreviewData {
        custom_name: "a".repeat(101),
        ..Default::default()
    };
    let result = serialize(&data);
    assert!(matches!(result, Err(Error::ValidationError(_))));
}

#[test]
fn test_serialize_custom_name_exactly_100_ok() {
    let data = ItemPreviewData {
        custom_name: "a".repeat(100),
        def_index: 1,
        ..Default::default()
    };
    let result = serialize(&data);
    assert!(result.is_ok());
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. CSFloat vectors
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_csfloat_a_def_index() {
    let item = deserialize(CSFLOAT_A).expect("should deserialize CSFLOAT_A");
    assert_eq!(item.def_index, 7);
}

#[test]
fn test_csfloat_a_paint_index() {
    let item = deserialize(CSFLOAT_A).expect("should deserialize CSFLOAT_A");
    assert_eq!(item.paint_index, 474);
}

#[test]
fn test_csfloat_a_paint_seed() {
    let item = deserialize(CSFLOAT_A).expect("should deserialize CSFLOAT_A");
    assert_eq!(item.paint_seed, 306);
}

#[test]
fn test_csfloat_a_rarity() {
    let item = deserialize(CSFLOAT_A).expect("should deserialize CSFLOAT_A");
    assert_eq!(item.rarity, 6);
}

#[test]
fn test_csfloat_a_paint_wear() {
    let item = deserialize(CSFLOAT_A).expect("should deserialize CSFLOAT_A");
    let pw = item.paint_wear.expect("paint_wear should be Some");
    assert!((pw - 0.6337).abs() < 0.001, "paint_wear={}", pw);
}

#[test]
fn test_csfloat_b_paint_index() {
    let item = deserialize(CSFLOAT_B).expect("should deserialize CSFLOAT_B");
    assert_eq!(item.paint_index, 1352);
}

#[test]
fn test_csfloat_b_paint_wear() {
    let item = deserialize(CSFLOAT_B).expect("should deserialize CSFLOAT_B");
    let pw = item.paint_wear.expect("paint_wear should be Some");
    assert!((pw - 0.99).abs() < 0.01, "paint_wear={}", pw);
}

#[test]
fn test_csfloat_b_sticker_count() {
    let item = deserialize(CSFLOAT_B).expect("should deserialize CSFLOAT_B");
    assert_eq!(item.stickers.len(), 4);
}

#[test]
fn test_csfloat_b_sticker_ids() {
    let item = deserialize(CSFLOAT_B).expect("should deserialize CSFLOAT_B");
    for sticker in &item.stickers {
        assert_eq!(sticker.sticker_id, 76, "sticker_id should be 76, got {}", sticker.sticker_id);
    }
}

#[test]
fn test_csfloat_c_def_index() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.def_index, 1355);
}

#[test]
fn test_csfloat_c_rarity() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.rarity, 3);
}

#[test]
fn test_csfloat_c_quality() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.quality, 12);
}

#[test]
fn test_csfloat_c_paint_wear_none() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.paint_wear, None, "paint_wear should be None for CSFLOAT_C");
}

#[test]
fn test_csfloat_c_keychain_count() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.keychains.len(), 1);
}

#[test]
fn test_csfloat_c_keychain_highlight_reel() {
    let item = deserialize(CSFLOAT_C).expect("should deserialize CSFLOAT_C");
    assert_eq!(item.keychains.len(), 1);
    let kc = &item.keychains[0];
    assert_eq!(kc.highlight_reel, Some(345), "highlight_reel should be 345, got {:?}", kc.highlight_reel);
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. HighlightReel round-trip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_highlight_reel_roundtrip_present() {
    let original = ItemPreviewData {
        def_index: 1355,
        keychains: vec![Sticker {
            slot: 0,
            sticker_id: 42,
            highlight_reel: Some(345),
            ..Default::default()
        }],
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.keychains[0].highlight_reel, Some(345));
}

#[test]
fn test_highlight_reel_roundtrip_absent() {
    let original = ItemPreviewData {
        def_index: 1355,
        keychains: vec![Sticker {
            slot: 0,
            sticker_id: 42,
            highlight_reel: None,
            ..Default::default()
        }],
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.keychains[0].highlight_reel, None);
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. Hybrid URL deserialization
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_deserialize_hybrid_url() {
    let item = deserialize(HYBRID_URL).expect("should deserialize HYBRID_URL");
    // HYBRID_URL contains the same hex as TOOL_HEX
    assert_eq!(item.def_index, 60);
    assert_eq!(item.paint_index, 440);
    assert_eq!(item.paint_seed, 353);
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. Default trait
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_item_preview_data_default() {
    let d = ItemPreviewData::default();
    assert_eq!(d.def_index, 0);
    assert_eq!(d.item_id, 0);
    assert_eq!(d.paint_wear, None);
    assert!(d.stickers.is_empty());
    assert!(d.keychains.is_empty());
}

#[test]
fn test_sticker_default() {
    let s = Sticker::default();
    assert_eq!(s.slot, 0);
    assert_eq!(s.sticker_id, 0);
    assert_eq!(s.wear, None);
    assert_eq!(s.highlight_reel, None);
    assert_eq!(s.paint_kit, None);
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. Sticker slab test vectors (paint_kit field)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_slab_a_def_index() {
    let item = deserialize(SLAB_A).expect("should deserialize SLAB_A");
    assert_eq!(item.def_index, 1355);
}

#[test]
fn test_slab_a_rarity() {
    let item = deserialize(SLAB_A).expect("should deserialize SLAB_A");
    assert_eq!(item.rarity, 5);
}

#[test]
fn test_slab_a_quality() {
    let item = deserialize(SLAB_A).expect("should deserialize SLAB_A");
    assert_eq!(item.quality, 8);
}

#[test]
fn test_slab_a_keychain_sticker_id() {
    let item = deserialize(SLAB_A).expect("should deserialize SLAB_A");
    assert_eq!(item.keychains.len(), 1);
    assert_eq!(item.keychains[0].sticker_id, 37);
}

#[test]
fn test_slab_a_keychain_paint_kit() {
    let item = deserialize(SLAB_A).expect("should deserialize SLAB_A");
    assert_eq!(item.keychains[0].paint_kit, Some(7256));
}

#[test]
fn test_slab_b_def_index() {
    let item = deserialize(SLAB_B).expect("should deserialize SLAB_B");
    assert_eq!(item.def_index, 1355);
}

#[test]
fn test_slab_b_rarity() {
    let item = deserialize(SLAB_B).expect("should deserialize SLAB_B");
    assert_eq!(item.rarity, 3);
}

#[test]
fn test_slab_b_quality() {
    let item = deserialize(SLAB_B).expect("should deserialize SLAB_B");
    assert_eq!(item.quality, 8);
}

#[test]
fn test_slab_b_keychain_sticker_id() {
    let item = deserialize(SLAB_B).expect("should deserialize SLAB_B");
    assert_eq!(item.keychains.len(), 1);
    assert_eq!(item.keychains[0].sticker_id, 37);
}

#[test]
fn test_slab_b_keychain_paint_kit() {
    let item = deserialize(SLAB_B).expect("should deserialize SLAB_B");
    assert_eq!(item.keychains[0].paint_kit, Some(275));
}

#[test]
fn test_slab_roundtrip_paint_kit() {
    let original = ItemPreviewData {
        def_index: 1355,
        rarity: 5,
        quality: 8,
        keychains: vec![Sticker {
            slot: 0,
            sticker_id: 37,
            paint_kit: Some(7256),
            ..Default::default()
        }],
        ..Default::default()
    };
    let hex = serialize(&original).expect("serialize");
    let decoded = deserialize(&hex).expect("deserialize");
    assert_eq!(decoded.keychains[0].paint_kit, Some(7256));
}
