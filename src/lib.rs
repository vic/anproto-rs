use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn gen() -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let secret: [u8; 32] = rng.gen();
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    let pub_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());
    let keypair_b64 = general_purpose::STANDARD.encode(signing_key.to_keypair_bytes());
    Ok(pub_b64 + &keypair_b64)
}

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}

pub fn sign(h: &str, k: &str) -> Result<String, String> {
    if k.len() < 44 + 88 {
        return Err("invalid key length".to_string());
    }
    let pub_b64 = &k[..44];
    let sec_b64 = &k[44..];
    let sec_bytes = general_purpose::STANDARD.decode(sec_b64).map_err(|_| "invalid secret key base64".to_string())?;
    let signing_key = SigningKey::from_keypair_bytes(&sec_bytes.try_into().map_err(|_| "invalid secret key length".to_string())?).map_err(|_| "invalid secret key".to_string())?;
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let msg = format!("{}{}", ts, h);
    let sig = signing_key.sign(msg.as_bytes());
    let signed = sig.to_bytes().iter().chain(msg.as_bytes()).cloned().collect::<Vec<_>>();
    let signed_b64 = general_purpose::STANDARD.encode(signed);
    Ok(pub_b64.to_string() + &signed_b64)
}

pub fn open(m: &str) -> Result<String, String> {
    if m.len() < 44 {
        return Err("invalid message length".to_string());
    }
    let pub_b64 = &m[..44];
    let signed_b64 = &m[44..];
    let signed = general_purpose::STANDARD.decode(signed_b64).map_err(|_| "invalid signed message base64".to_string())?;
    if signed.len() < 64 {
        return Err("signed message too short".to_string());
    }
    let sig_bytes: [u8; 64] = signed[..64].try_into().unwrap();
    let sig = Signature::from_bytes(&sig_bytes);
    let msg = &signed[64..];
    let pub_bytes = general_purpose::STANDARD.decode(pub_b64).map_err(|_| "invalid public key base64".to_string())?;
    let verifying_key = VerifyingKey::from_bytes(&pub_bytes.try_into().map_err(|_| "invalid public key length".to_string())?).map_err(|_| "invalid public key".to_string())?;
    verifying_key.verify(msg, &sig).map_err(|_| "signature verification failed".to_string())?;
    String::from_utf8(msg.to_vec()).map_err(|_| "invalid utf8 in message".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let k = gen().unwrap();
        assert_eq!(k.len(), 132);
    }

    #[test]
    fn test_hash() {
        let h = hash("hello");
        assert_eq!(h.len(), 44); // base64 of 32 bytes
    }

    #[test]
    fn test_sign_open() {
        let k = gen().unwrap();
        let h = hash("hello");
        let signed = sign(&h, &k).unwrap();
        assert!(signed.starts_with(&k[..44]));
        let opened = open(&signed).unwrap();
        assert!(opened.ends_with(&h));
    }

    #[test]
    fn test_example() {
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
}
