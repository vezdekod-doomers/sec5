use crypto::curve25519::{curve25519_base, curve25519};
use crypto::chacha20poly1305::ChaCha20Poly1305;
use crypto::aead::AeadEncryptor;
use rand::{rngs::OsRng, RngCore};
use std::io::BufRead;
use crate::sender::{Sender, PrintSender, EmailSender};
use std::error::Error;

mod sender;
pub(crate) mod helper;

pub fn encrypt(public_key: &[u8; 32], message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ephemeral_secret_key = [0u8; 32];
    OsRng.fill_bytes(&mut ephemeral_secret_key[..]);

    let ephemeral_public_key: [u8; 32] = curve25519_base(&ephemeral_secret_key[..]);
    let symmetric_key = curve25519(&ephemeral_secret_key[..], &public_key[..]);

    let mut c = ChaCha20Poly1305::new(&symmetric_key, &[0u8; 8][..], &[]);

    let mut output = vec![0; 32 + 16 + message.len()];
    let mut tag = [0u8; 16];
    c.encrypt(message, &mut output[32+16..], &mut tag[..]);

    for (dest, src) in (&mut output[0..32]).iter_mut().zip( ephemeral_public_key.iter() ) {
        *dest = *src;
    }

    for (dest, src) in (&mut output[32..48]).iter_mut().zip( tag.iter() ) {
        *dest = *src;
    }

    Ok(output)
}

fn get_msg() -> String {
    let stdin1 = std::io::stdin();
    let mut iter = stdin1.lock().lines();
    println!("Enter message to encrypt: ");
    iter.next().unwrap().unwrap()
}

fn main() {
    let mut senders: Vec<Box<dyn Sender>> = vec![Box::new(PrintSender {}), Box::new(EmailSender::new())];
    for x in &mut senders {
        x.init()
    }
    let mut selected_sender: Option<&Box<dyn Sender>> = None;
    loop {
        let mut msg = get_msg();
        if msg.starts_with("\\") {
            let msg_multiline_result = helper::read_multiline();
            if let Err(e) = msg_multiline_result {
                println!("Failed to read multiline message: {}", e);
                continue
            } else {
                msg = msg_multiline_result.unwrap()
            }
        }

        let mut secret_key = [0u8; 32];
        OsRng.fill_bytes(&mut secret_key[..]);

        let public_key = curve25519_base(&secret_key[..]);

        let encrypted_result = encrypt(&public_key, &msg.as_bytes());
        if let Err(e) = encrypted_result {
            println!("Failed to encrypt message: {}", e);
            continue
        }
        let encoded = base64::encode(encrypted_result.unwrap());

        print!("Select sender from [{}]", senders.iter().map(|e| e.name()).collect::<Vec<&'static str>>().join(", "));
        if let Some(sender) = &selected_sender {
            print!("({})", sender.name())
        }
        println!();
        let mut selected = helper::read_line();
        if !selected.is_empty() || selected_sender.is_none() {
            selected_sender = senders.iter().filter(|e| e.name() == selected).next();
            while selected_sender.is_none() {
                println!("Sender not found. Please, select valid sender from list [{}]", senders.iter().map(|e| e.name()).collect::<Vec<&'static str>>().join(", "));
                selected = helper::read_line();
                selected_sender = senders.iter().filter(|e| e.name() == selected).next();
            }
        }

        if let Err(e) = selected_sender.unwrap().send(&encoded) {
            println!("Failed to send message: {}", e)
        } else {
            println!("Message sent! Decryption key is '{}'", base64::encode(secret_key));
        }
    }
}
