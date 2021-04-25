use crypto::curve25519::curve25519;
use crypto::chacha20poly1305::ChaCha20Poly1305;
use crypto::aead::AeadDecryptor;
use std::io::BufRead;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::convert::TryInto;

#[derive(Debug)]
enum DecryptError {
    Invalid,
    Malformed
}

impl Display for DecryptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DecryptError {}

pub fn decrypt(secret_key: &[u8; 32], message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if message.len() < 48 {
        return Err(Box::new(DecryptError::Malformed));
    }

    let ephemeral_public_key = &message[0..32];
    let tag = &message[32..48];
    let ciphertext = &message[48..];

    let mut plaintext = vec![0; ciphertext.len()];
    let symmetric_key = curve25519(secret_key, ephemeral_public_key);

    let mut decrypter = ChaCha20Poly1305::new(&symmetric_key[..], &[0u8; 8][..], &[]);
    if !decrypter.decrypt(ciphertext, &mut plaintext[..], tag) {
        return Err(Box::new(DecryptError::Invalid));
    }

    Ok(plaintext)
}

fn iteration() -> Result<(), Box<dyn Error>> {
    let stdin1 = std::io::stdin();
    let mut iter = stdin1.lock().lines();
    println!("Enter message to decrypt:");
    let msg = iter.next().unwrap()?.trim().to_owned();
    let decoded = base64::decode(msg)?;

    println!("Enter decode key:");
    let key = iter.next().unwrap()?.trim().to_owned();
    let key_decoded = base64::decode(key)?;

    let dec = decrypt(&key_decoded.try_into().unwrap(), &decoded)?;
    println!("---------------------");
    println!("   Decoded message   ");
    println!("---------------------");
    let msg = String::from_utf8_lossy(&dec);
    if matches!(msg.find("\n"), Some(_)) {
        print!("{}", msg)
    } else {
        println!("{}", msg);
    }
    println!("---------------------");
    Ok(())
}

fn main() {
    loop {
        if let Err(e) = iteration() {
            println!("Error while processing: {:?}", e);
        }
    }
}
