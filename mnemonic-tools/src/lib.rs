mod u11;

#[allow(dead_code)]
pub mod mnemonic;

const BITS_PER_BYTE: usize = 8;

// const PBKDF2_ROUNDS: u32 = 2048;
//
// fn main() -> anyhow::Result<()> {
//     // let mut entropy = [0u8; ENTROPY_BYTES];
//     // rand::thread_rng().fill_bytes(&mut entropy);
//     // let indices = entropy_to_indices(&entropy);
//     // let words = indices_to_mnemonic_words(&indices);
//     let words = "satisfy spend denial mammal salon trade monster echo until stand say aunt"
//         .split(" ")
//         .collect::<Vec<_>>();
//     println!("mnemonic: {}", words.join(" "));
//
//     let passphrase = "";
//     let password = words.join(" ");
//     let salt = format!("mnemonic{passphrase}");
//     let mut seed = [0u8; 64];
//     pbkdf2_hmac::<Sha512>(
//         password.as_bytes(),
//         salt.as_bytes(),
//         PBKDF2_ROUNDS,
//         &mut seed,
//     );
//
//     // let mut master = Hmac::<Sha512>::new_from_slice(b"Bitcoin seed")?;
//     // master.update(&seed);
//     // let master = master.finalize().into_bytes();
//     // let master: [u8; 64] = master.as_slice().try_into()?;
//     //
//     // let secret_key = SecretKey::from_byte_array(&master[..32].try_into()?)?;
//     // let secp = Secp256k1::new();
//
//     Ok(())
// }
//
