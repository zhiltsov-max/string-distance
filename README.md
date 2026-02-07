# String distance

A library to compute string edit distance
([Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)), written in Rust.

## Installation

```bash
cargo add --git "https://github.com/zhiltsov-max/string-distance.git"
```

## Usage

The library provides the following functions:

```rust
is_ascii(s: &String) -> bool
count_char_diff(a: &String, b: &String) -> i32
levenshtein(a: &String, b: &String) -> i32
```

`is_ascii(s: &String) -> bool`
- Checks if the passed string is a 1 byte string

`count_char_diff(a: &String, b: &String) -> i32`
- Counts the sum of different characters in 2 strings (position-independent), e.g.
  `count_char_diff("abcd", "bfa") = 3` (a and b match; c, d, and f don't).
  The main purpose  of this function is to be a quick early-exit check before running
  the more expensive `levenshtein()` function. The returned value is the minimum distance estimate,
  i.e. `count_char_diff(a, b) <= 2 * levenshtein(a, b)`.

`levenshtein(a: &String, b: &String) -> i32`
- Computes the Levenshtein distance for 2 input strings.

There are also specializations for different string contents - ASCII (1 byte per character)
and Unicode (multibyte). They can be useful if you want to be specific about the function being
used. These functions are automatically called by the higher-level ones mentioned earlier,
depending on the results of `is_ascii(arg)` check for both arguments. Typically, ASCII-based
functions work faster than Unicode-based ones.

```rust
count_char_diff_ascii(a: &String, b: &String) -> i32
count_char_diff_utf(a: &String, b: &String) -> i32
levenshtein_ascii(a: &String, b: &String) -> i32
levenshtein_utf(a: &String, b: &String) -> i32
```

## Testing

```bash
cargo test
```

## Performance

**levenshtein()**:

| Charset | Length | Time (µs) |
| - | - | - |
| ASCII | 16 | 0.859 ±0.003 |
| ASCII | 64 | 7.7590 ±0.03 |
| ASCII | 128 | 26.730 ±0.1 |
| ASCII | 256 | 98.987 ±0.3 |
| ASCII | 512 | 383.92 ±0.6 |
| ASCII | 1000 | 1414.7 ±3 |
| UTF8 | 16 | 2.2007 ±0.01 |
| UTF8 | 64 | 23.510 ±0.1 |
| UTF8 | 128 | 84.166 ±0.2 |
| UTF8 | 256 | 320.23 ±2 |
| UTF8 | 512 | 1.6507 ±3 |
| UTF8 | 1000 | 31650 ±300 |

* CPU: AMD Ryzen 7 6800H

## Benchmarking

```bash
cargo bench
```
