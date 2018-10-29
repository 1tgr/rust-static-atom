# static-atom
[The benchmarks](src/lib.rs) decode any of of the following strings to `Option<NonZeroU8>`:
- `"BTC-EUR"` produces `Some(1)`
- `"ETH-EUR"` produces `Some(2)`
- `"ETH-BTC"` produces `Some(3)`
- Anything else produces `None`

# match_keyword
Standard Rust `match` keyword.

```
match_keyword           time:   [13.740 ns 13.803 ns 13.887 ns]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
```

# phf
Lookup in a hash table generated at compile time by the [`phf` crate](https://github.com/sfackler/rust-phf).

```
phf                     time:   [74.116 ns 74.361 ns 74.702 ns]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe
```

# static_map
Lookup in a hash table generated at compile time by the [`static_map` crate](https://github.com/cbreeden/static-map).

```
static_map              time:   [13.698 ns 13.743 ns 13.797 ns]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
```

# trie_u8
Consume one byte at a time to narrow down valid suffixes for the rest of the string.

```
trie_u8                 time:   [3.6510 ns 3.6596 ns 3.6711 ns]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
```

# trie_u8_u32
Like trie_u8, but after consuming the `u8` prefix, check the next six bytes as a `u32` then a `u16`.

```
trie_u8_u32             time:   [3.8726 ns 3.8954 ns 3.9192 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

# trie_u32_u8
Like trie_u8_u32, but consume the first four bytes as a `u32`. Read the next three bytes as ` u16` then a `u8`.

```
trie_u32_u8             time:   [3.8007 ns 3.8181 ns 3.8372 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```