use anproto::*;
use ed25519_dalek::SigningKey;
use base64::{Engine as _, engine::general_purpose};

fn main() {
    let m = "Hello World";
    let h = hash(m);
    // Use fixed secret for deterministic output
    let secret = [0u8; 32];
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    let pub_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());
    let keypair_b64 = general_purpose::STANDARD.encode(signing_key.to_keypair_bytes());
    let k = pub_b64 + &keypair_b64;
    let s = sign(&h, &k).unwrap();
    let o = open(&s).unwrap();
    println!("{}", k);
    println!("{}", h);
    println!("{}", s);
    println!("{}", o);
}
