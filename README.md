# static-atom
[The benchmarks](src/lib.rs) decode any of of the following strings to `Option<NonZeroU8>`:
- `"BTC-EUR"` produces `Some(1)`
- `"ETH-EUR"` produces `Some(2)`
- `"ETH-BTC"` produces `Some(3)`
- Anything else produces `None`

# match_keyword
Standard Rust `match` keyword.

```
match_keyword           time:   [13.931 ns 13.981 ns 14.040 ns]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

# phf
Lookup in a hash table generated at compile time by the [`phf` crate](https://github.com/sfackler/rust-phf).

```
phf                     time:   [77.568 ns 77.829 ns 78.129 ns]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe
```

# static_map
Lookup in a hash table generated at compile time by the [`static_map` crate](https://github.com/cbreeden/static-map).

```
static_map              time:   [54.846 ns 55.241 ns 55.696 ns]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```

# trie_u8
Consume one byte at a time to narrow down valid suffixes for the rest of the string.

```
trie_u8                 time:   [3.6513 ns 3.6571 ns 3.6636 ns]
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe
```

# trie_u8_u32
Like trie_u8, but after consuming the `u8` prefix, check the next six bytes as a `u32` then a `u16`.

```
trie_u8_u32             time:   [3.6532 ns 3.6602 ns 3.6686 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
```

# trie_u32_u8
Like trie_u8_u32, but consume the first four bytes as a `u32`. Read the next three bytes as ` u16` then a `u8`.

```
trie_u32_u8             time:   [3.6628 ns 3.6720 ns 3.6833 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```

# trie_generated_small
Like trie_u8_u32, but the code is generated automatically by a build script.

```
trie_generated_small    time:   [3.6487 ns 3.6542 ns 3.6605 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```

# trie_generated_big
Like trie_generated_small, but the generated code tests against 24 strings not 3.

```
trie_generated_big      time:   [14.847 ns 14.915 ns 14.999 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```