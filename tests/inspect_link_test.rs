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
    assert!(matches!(result, Err(Error::MalformedLink(_))));
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

// ───────────────────────────────────────────────────────────────────────────
// 7. Malformed URLs (regression: must reject cleanly with Error::MalformedLink)
// ───────────────────────────────────────────────────────────────────────────

const MALFORMED_URLS: &[(&str, &str)] = &[
    ("truncated mid-keychain (defindex=1, key=0xAD)",  "steam://run/730//+csgo_econ_action_preview%20ADBD1050393912ACB5AC8D45AC85A99DA9956A116D5FAEED21ACCFB4A5AFBD348EB0ADAD2D9280ADADDD6F90EDA37510E84D8BEE11CFB4A5ACBD348EB0ADAD2D9280ADAD5D6F906F2B4C13E84D93D591CFB9A5ADBD419EB0ADAD2D9290ADF22010E8B72FB213CFB4A5ADBD549EB0ADAD2D9280ADADED6C90CFD43F10E892DFE513CFB4A5ADBD549EB0ADAD2D9280ADAD85EE902F952210E82EB8A613C52E2D2D2DA1DDA90FACBBA5ADBD89902923AAECE83"),
    ("truncated mid-keychain (defindex=9, key=0xEE)",  "steam://run/730//+csgo_econ_action_preview%20EEFE3144332550EFF6E7CE28E8C6EADEEAD642323218EDAE4DEA8CFAE6ECFE35A7F302BFD6D1D3004FCB50ABAE5CF8528CF7E6EEFE0DA7F3394DDED1C3EEEE7EAFD39E25B3D3AB9EAE70D28CFAE6ECFE32A7F3EEEE6ED1D3595B17D3AB9E8E65538CFAE6ECFE64ADF3EEEE6ED1D3E597F3D3AB2EEA1AD58CF7E6EDFE0BCAF302BFD6D1C3EEEEEF2DD3AEF5F552ABEE31A855866D6E6E6EE29EE64CEFF8E6EEFED8D3B6CBCBACABFD70EED1A31F96E7AFBE5"),
    ("truncated mid-keychain (defindex=1, key=0x4A)",  "steam://run/730//+csgo_econ_action_preview%204A5A8EFCB1B9F44B524B6AA24B624E7A4E72BACFF6B8490AD449285E42485AAB75576316457577CA2422760F4A413E712853424A5AA679574A4ACA75674A4A8A8A770A7F85760FD04246F4285342495AB279574A4ACA75674A4A8A0A7799714A750F0A5140F7285342495AAD7277EB4547F40F0A00EB7122C9CACACA463A4EE84B5D424A5A4C776C02A10A0F34A5C17407F0145C0A1AA"),
    ("truncated mid-keychain (AK-47 1035, key=0xFA)",  "steam://run/730//+csgo_econ_action_preview%20FAEA5766387F45FBE2FDDA71F2D2FECAF3C2142C0C0EF9BA7CFFB2FAAAFA98EEF2F8EA3BFBD7FAFA3ABBC7EA2FB7C7BF9ACC47C698E3F2F9EA03C9E7FAFA7AC5D7FAFABA3AC780CA89C4BFFAAAD9C198E3F2F9EA03C9E7FAFA7AC5D7FAFACEB9C7B60177C4BFAAB82AC698EEF2F9EA03C9E7FAFA7AC5C7C11558C4BFFA43DBC198F5F2FBEA13DEC7759A1147BF9A16F64692797A7A7AF68AF258FBEFF2FAEADEC7DEAC32BBBF0BF9BAC4B760382CC5A2D24"),
    ("truncated mid-keychain (defindex=40, key=0x9F)", "steam://run/730//+csgo_econ_action_preview%209F8F4F504C7C219E87B7BF629EB79CAF9BA73F1D53419CDF0699FD8B979F8F5CCF82050686A0A2F4038821DA17F3FD22FD8B979F8F49D4825C6AB7A0A25F50B224DA7F0CF222FD8B979D8F5DCF82F9F9B9A0A2B7B25422DA6F6F1422FD8B979D8F5DCF822781DAA0A247B731A2DA7F92EF23FD86979F8F5DCF827EE5CBA0B29F9FDFDFA285AD4322DA9FFD8AA3F71C1F1F1F93EF873D9E88979F8FDDA243C05EDFDA4CFB44A0D202B77EDFCF3F339C3DF89"),
    ("truncated mid-keychain (M4A1-S 1130, key=0xFA)", "steam://run/730//+csgo_econ_action_preview%20FAEA5B24060844FBE2C6DA10F2D2FECAF3C2631A3308F9BA47F8B2FAAAFA98EEF2FEEA29B2E781EED4C5C7776B78C4BFFAFA21CD98EEF2FAEA09BCE7F02DD9C5C7FE6C57C7BF7A58ED4698EEF2FEEA6DBDE79C9CDCC5C7929F2F47BFFA461BC398E3F2FBEA15BDE7E57FD1C5D7FAFA8AB8C7C2CEDC46BF12973EC798EEF2FEEA24B8E781EED4C5C75696FCC5BF2AEBF2C792797A7A7AF68AF258FBEDF2FAEAD2C7B3683FBBBF065F8CC5B763A382BAAA0C5"),
    ("truncated mid-keychain (defindex=35, key=0x4D)", "steam://run/730//+csgo_econ_action_preview%204D5D9DF8C7D2F34C556E6DDC4C654E7D4975B2D2AEBB4E0DAB4B2F59454E5D9A70604D4DBD8C7002A356F308CD4603F62F5445495DEB745011C20F72604D4D798F70797F9FF3089D49ABF12F5945495DEB74502B2BAB737045B3FAF308FD4BE8F12F5945495DF868508081417270BFB9D2F308ADD283F12F5945495DF86850AC375972702FA3CBF3083D2EBFF125CECDCDCD413D5AEF4C5A454D5D567084E4F80C08254C547200CE3E1D0D1DF9D24C63938"),
    ("truncated mid-keychain (AK-47 1171, key=0xCF)",  "steam://run/730//+csgo_econ_action_preview%20CFDF6258412F71CED7C8EF5CC6E7C9FFCBF7465B3B38CC8F7ACEADD6C7CEDF3BF2D2F2C5D8F0E2CFCFE40CF27F72E1728AF786F5F2ADC0C7CCDF3BF2F241D88EF18A0F03F6F2ADDBC7CCDF3CF2E2CFCF0F8DF27B3FB7F18A6F5ACDF2ADD6C7CCDF3CF2D2C518ECF0E2CFCF8F0EF2D5C29AF18ACFCFD8F5ADDBC7CFDF3CF2E2CFCF6D8DF24DB0DA718A2FA6DBF3A74C4F4F4FC3BFC76DCED8C7CFDF87F27B950C8E8A37D0A3F182A2B8A48F9F4332CD2C2B6"),
    ("truncated mid-keychain (defindex=1 1050, key=0xCE)", "steam://run/730//+csgo_econ_action_preview%20CEDE51082D1C70CFD6CFEE54C6E6CAFEC7F631274538CD8E2DC886CE9ECEACDAC6CDDE0C8DD3CECE4EF1F382996B738B4E5E8D75ACD7C6CEDE0C8DD3CECE4EF1E3CECE8E0FF3A56603708BECDBE170ACD7C6CEDE0C8DD3CECE4EF1E3CECEDE0FF37682A5708B0650FB70ACD7C6CEDE0C8DD3CECE4EF1E3CECEDE0FF34E3CEC738BDAC6F870ACD7C6CDDE798AD3CECE4EF1E3CECE0E0EF3333BC5F18BAE8E9FF2A64D4E4E4EC2BECA6CCFD9C6CEDECFF37C6"),
    ("odd-length bare hex", "ABC"),
    ("empty string", ""),
    ("non-hex characters", "ZZZZZZZZZZZZ"),
];

#[test]
fn test_malformed_urls_all_rejected() {
    for (label, url) in MALFORMED_URLS {
        let result = deserialize(url);
        assert!(
            matches!(result, Err(Error::MalformedLink(_))),
            "expected MalformedLink for {label:?}, got {result:?}"
        );
    }
}

#[test]
fn test_malformed_link_message_mentions_length_for_odd_hex() {
    match deserialize("ABC") {
        Err(Error::MalformedLink(msg)) => {
            assert!(
                msg.contains("length") || msg.contains("even") || msg.contains("hex"),
                "expected length/even/hex hint in message: {msg}"
            );
        }
        other => panic!("expected MalformedLink, got {other:?}"),
    }
}
