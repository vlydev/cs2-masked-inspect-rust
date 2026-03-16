# cs2-masked-inspect

Pure Rust library for encoding and decoding CS2 masked inspect links — no runtime dependencies.

[![Tests](https://github.com/vlydev/cs2-masked-inspect-rust/actions/workflows/tests.yml/badge.svg)](https://github.com/vlydev/cs2-masked-inspect-rust/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/cs2-masked-inspect)](https://crates.io/crates/cs2-masked-inspect)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Overview

CS2 inspect links encode a `CEconItemPreviewDataBlock` protobuf payload directly in the URL. This library provides offline encode/decode without Steam API access. It handles all three link variants:

- **Tool-generated** (`key_byte = 0x00`): no XOR obfuscation
- **Native CS2** (`key_byte != 0x00`): every byte XOR'd with the key
- **Hybrid** (`S<steamid>A<assetid>D<hex_payload>`): steamid/assetid prefix followed by hex proto
- **Classic** (`S<steamid>A<assetid>D<decimal>`): standard Steam inspect URL (not decodable offline)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cs2-masked-inspect = "0.1"
```

## Usage

### Deserialize an inspect link

```rust
use cs2_masked_inspect::deserialize;

// Bare hex payload
let item = deserialize("00183C20B803280538E9A3C5DD0340E102C246A0D1")?;
println!("def_index={} paint_index={} paint_seed={}", item.def_index, item.paint_index, item.paint_seed);

// Hybrid URL
let item = deserialize("steam://rungame/730/.../+csgo_econ_action_preview%20S76561199323320483A50075495125D00183C20B803280538...")?;

// Native CS2 link
let item = deserialize("E3F33674...")?;
```

### Serialize to inspect link hex

```rust
use cs2_masked_inspect::{serialize, ItemPreviewData, Sticker};

let item = ItemPreviewData {
    def_index: 7,
    paint_index: 422,
    paint_seed: 922,
    paint_wear: Some(0.04121),
    rarity: 3,
    quality: 4,
    stickers: vec![
        Sticker {
            slot: 0,
            sticker_id: 7436,
            ..Default::default()
        },
    ],
    ..Default::default()
};

let hex = serialize(&item)?; // uppercase hex, starts with "00"
```

### Detect link type

```rust
use cs2_masked_inspect::{is_masked, is_classic};

// Masked links contain a decodable protobuf payload
assert!(is_masked("00183C20B803..."));
assert!(is_masked("steam://...D00183C20B803..."));

// Classic links use decimal Steam parameters (cannot be decoded offline)
assert!(is_classic("steam://rungame/730/.../+csgo_econ_action_preview%20S76561199842063946A49749521570D2751293026650298712"));
```

## Validation rules

| Field | Rule |
|-------|------|
| `paint_wear` | Must be in `[0.0, 1.0]` or `None` |
| `custom_name` | Maximum 100 characters |
| Hex payload (input) | Maximum 4096 hex characters (2048 bytes) |
| Binary payload (input) | Minimum 6 bytes |

## Proto field reference

### ItemPreviewData

| Field | Number | Wire Type | Type |
|-------|--------|-----------|------|
| account_id | 1 | varint | u32 |
| item_id | 2 | varint | u64 |
| def_index | 3 | varint | u32 |
| paint_index | 4 | varint | u32 |
| rarity | 5 | varint | u32 |
| quality | 6 | varint | u32 |
| paint_wear | 7 | varint | f32 as u32 bits |
| paint_seed | 8 | varint | u32 |
| kill_eater_score_type | 9 | varint | u32 |
| kill_eater_value | 10 | varint | u32 |
| custom_name | 11 | length-delimited | String |
| stickers | 12 | length-delimited (repeated) | Sticker |
| inventory | 13 | varint | u32 |
| origin | 14 | varint | u32 |
| quest_id | 15 | varint | u32 |
| drop_reason | 16 | varint | u32 |
| music_index | 17 | varint | u32 |
| ent_index | 18 | varint | i32 |
| pet_index | 19 | varint | u32 |
| keychains | 20 | length-delimited (repeated) | Sticker |

### Sticker

| Field | Number | Wire Type | Type |
|-------|--------|-----------|------|
| slot | 1 | varint | u32 |
| sticker_id | 2 | varint | u32 |
| wear | 3 | fixed32 LE | f32 |
| scale | 4 | fixed32 LE | f32 |
| rotation | 5 | fixed32 LE | f32 |
| tint_id | 6 | varint | u32 |
| offset_x | 7 | fixed32 LE | f32 |
| offset_y | 8 | fixed32 LE | f32 |
| offset_z | 9 | fixed32 LE | f32 |
| pattern | 10 | varint | u32 |
| highlight_reel | 11 | varint | Option<u32> |

## Test vectors

### TOOL_HEX
```
00183C20B803280538E9A3C5DD0340E102C246A0D1
```
Expected: `def_index=60, paint_index=440, paint_seed=353, paint_wear≈0.005411, rarity=5`

### NATIVE_HEX (native CS2 link, key_byte=0xE3)
```
E3F3367440334DE2FBE4C345E0CBE0D3E7DB6943400AE0A379E481ECEBE2F36F...
```
Expected: `item_id=46876117973, def_index=7, paint_index=422, paint_seed=922, paint_wear≈0.04121, rarity=3, quality=4, stickers=[7436,5144,6970,8069,5592]`

### CSFLOAT_A
```
00180720DA03280638FBEE88F90340B2026BC03C96
```
Expected: `def_index=7, paint_index=474, paint_seed=306, rarity=6, paint_wear≈0.6337`

### CSFLOAT_B
```
00180720C80A280638A4E1F5FB03409A0562040800104C...
```
Expected: `paint_index=1352, paint_wear≈0.99, 4 stickers each sticker_id=76`

### CSFLOAT_C
```
A2B2A2BA69A882A28AA192AECAA2D2B700A3A5AAA2B286FA7BA0D684BE72
```
Expected: `def_index=1355, rarity=3, quality=12, 1 keychain with highlight_reel=345, paint_wear=None`

## Running tests

```bash
cargo test
```

## Contributing

Contributions are welcome. Please ensure all tests pass before submitting a pull request:

```bash
cargo test
cargo clippy
```

## License

MIT — see [LICENSE](LICENSE).
