# static-atom
[The benchmarks](src/lib.rs) decode any of of the following strings to `Option<NonZeroU8>`:
- `"BTC-EUR"` produces `Some(1)`
- `"ETH-EUR"` produces `Some(2)`
- `"ETH-BTC"` produces `Some(3)`
- Anything else produces `None`

| Benchmark | Timing | Description |
| --- | ---: | --- |
| match_keyword | 15 ns/iter (+/- 1) | Standard Rust `match` keyword. |
| phf | 123 ns/iter (+/- 19) | Lookup in a hash table generated at compile time by the [`phf` crate](https://github.com/sfackler/rust-phf). |
| trie_u8 | 7 ns/iter (+/- 0) | Consume one byte at a time to narrow down valid suffixes for the rest of the string. |
| trie_u8_u32 | 3 ns/iter (+/- 0) | Like trie_u8, but after consuming the `u8` prefix, check the next six bytes as a `u32` then a `u16`. |
| trie_u32_u8 | 3 ns/iter (+/- 0) | Like trie_u8_u32, but consume the first four bytes as a `u32`. Read the next three bytes as ` u16` then a `u8`. |
