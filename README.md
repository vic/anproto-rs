# anproto-rs

Rust implementation of [ANProto](https://github.com/evbogue/ANProto): the Authenticated and Non-networked protocol or ANother protocol.

## Description

ANProto is a protocol for authenticated, non-networked messages using ed25519 signatures, timestamps, and SHA-256 hashes, all base64-encoded.

## Usage

Add to your `Cargo.toml`:

## API

- `gen() -> Result<String, String>`: Generates an ed25519 keypair, returns base64(public + secret).
- `hash(data: &str) -> String`: Computes SHA-256 of data, returns base64 hash.
- `sign(hash: &str, key: &str) -> Result<String, String>`: Signs (timestamp + hash) with the secret key, returns base64(public + signature + timestamp + hash).
- `open(signed_message: &str) -> Result<String, String>`: Verifies the signature and returns (timestamp + hash) if valid.

## Example

Run the example:

```bash
cargo run --example example
```

This matches the output of the JavaScript implementation in `ANProto/node_ex.js`.

## Testing

Run tests:

```bash
cargo test
```

## License

Apache 2.0
