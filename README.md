bc (An arbitrary precision calculator language)
====================

[![CI](https://github.com/magiclen/bc/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/bc/actions/workflows/ci.yml)

Use `bc` in the Rust Programming Language.

## Examples

```rust
let result = bc::bc!("2 + 6");

assert_eq!("8", result.unwrap());
```

```rust
let result = bc::bc!("2.5 + 6");

assert_eq!("8.5", result.unwrap());
```

```rust
let result = bc::bc_timeout!("99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```

```rust
let result = bc::bc_timeout!(20, "99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```

## Crates.io

https://crates.io/crates/bc

## Documentation

https://docs.rs/bc

## License

[MIT](LICENSE)