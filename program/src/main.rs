#![no_main]
sp1_zkvm::entrypoint!(main);

use dilithium::{ml_dsa_44, ml_dsa_65, ml_dsa_87};

pub fn main() {
    let variant = sp1_zkvm::io::read::<u8>();
    let public_key_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let signature_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let message = sp1_zkvm::io::read::<Vec<u8>>();

    let is_valid = match variant {
        2 => {
            let public_key = ml_dsa_44::PublicKey::from_bytes(&public_key_bytes);
            public_key.verify(&message, &signature_bytes, None)
        }
        3 => {
            let public_key = ml_dsa_65::PublicKey::from_bytes(&public_key_bytes);
            public_key.verify(&message, &signature_bytes, None)
        }
        5 => {
            let public_key = ml_dsa_87::PublicKey::from_bytes(&public_key_bytes);
            public_key.verify(&message, &signature_bytes, None)
        }
        _ => panic!("Invalid Dilithium variant"),
    };

    sp1_zkvm::io::commit(&is_valid);
}
