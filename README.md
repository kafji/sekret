# Sekret

A container type for redacting its content from being printed.

```rust
let token = sekret::Secret("secret_token");
println!("Secret token is: `{token}`.");
```

```
Secret token is: `█████`.
```

## Install

```toml
sekret = { git = "https://github.com/kafji/sekret", tag = "v0.1.1", features = ["ext_serde"] }
```
