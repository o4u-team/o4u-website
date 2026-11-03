use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use base64::{Engine, engine::general_purpose::STANDARD};
use hmac::{Hmac, Mac};
use o4u_domain::{Result, invalid};
use rand::RngCore;
use serde_json::{Value, json};
use sha2::Sha256;
/// Match PHP OpenSSL's AES-128-CBC key truncation / NUL padding and Laravel envelope.
pub fn encrypt(plain: &str, secret: &str) -> Result<String> {
    if secret.is_empty() {
        return Err(o4u_domain::Error::Unavailable(
            "O4U_SECRET_KEY is missing".into(),
        ));
    }
    let key = aes_key(secret);
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    let encrypted = cbc::Encryptor::<aes::Aes128>::new(&key.into(), &iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(plain.as_bytes());
    let iv = STANDARD.encode(iv);
    let value = STANDARD.encode(encrypted);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| o4u_domain::Error::Internal)?;
    mac.update(format!("{iv}{value}").as_bytes());
    let mac = mac
        .finalize()
        .into_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    Ok(STANDARD.encode(json!({"iv":iv,"value":value,"mac":mac,"tag":""}).to_string()))
}
fn aes_key(secret: &str) -> [u8; 16] {
    let mut key = [0u8; 16];
    let s = secret.as_bytes();
    let n = s.len().min(16);
    key[..n].copy_from_slice(&s[..n]);
    key
}
pub fn decrypt(payload: &str, secret: &str) -> Result<String> {
    let fail = || invalid("data_decrypt", "The payload or MAC is invalid");
    let bytes = STANDARD.decode(payload).map_err(|_| fail())?;
    let v: Value = serde_json::from_slice(&bytes).map_err(|_| fail())?;
    let iv = v["iv"].as_str().ok_or_else(fail)?;
    let value = v["value"].as_str().ok_or_else(fail)?;
    if v["tag"].as_str().is_some_and(|s| !s.is_empty()) {
        return Err(fail());
    }
    let hex = v["mac"].as_str().ok_or_else(fail)?;
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(fail());
    }
    let signature = (0..32)
        .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).map_err(|_| fail()))
        .collect::<Result<Vec<_>>>()?;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| fail())?;
    mac.update(format!("{iv}{value}").as_bytes());
    mac.verify_slice(&signature).map_err(|_| fail())?;
    let iv: [u8; 16] = STANDARD
        .decode(iv)
        .map_err(|_| fail())?
        .try_into()
        .map_err(|_| fail())?;
    let encrypted = STANDARD.decode(value).map_err(|_| fail())?;
    let plain = cbc::Decryptor::<aes::Aes128>::new(&aes_key(secret).into(), &iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(&encrypted)
        .map_err(|_| fail())?;
    String::from_utf8(plain).map_err(|_| fail())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decrypts_php_openssl_fixture() {
        let payload = "eyJpdiI6IllXRmhZV0ZoWVdGaFlXRmhZV0ZoWVE9PSIsInZhbHVlIjoiVDFPMG0xQUtvb3V5TG5vYmhRa2drdz09IiwibWFjIjoiMGYxNTlmNWM3Zjg3MDNlYWNjYTcwMGU2MDk1YTY0Y2Q4MDVhM2Y0MGJiY2ZjZGMxNDQ5YWU1YWQzYTA4ZTIyNiIsInRhZyI6IiJ9";
        assert_eq!(decrypt(payload, "legacy secret").unwrap(), "Xin chào O4U");
    }
    #[test]
    fn round_trip_and_wrong_key() {
        let s = encrypt("Xin chào", "legacy secret").unwrap();
        assert_eq!(decrypt(&s, "legacy secret").unwrap(), "Xin chào");
        assert!(decrypt(&s, "other secret").is_err());
    }
}
