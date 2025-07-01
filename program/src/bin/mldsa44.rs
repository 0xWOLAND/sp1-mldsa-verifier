#![no_main]
sp1_zkvm::entrypoint!(main);

use dilithium::ml_dsa_44;

pub fn main() {
    let public_key_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let signature_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let message = sp1_zkvm::io::read::<Vec<u8>>();

    let public_key = ml_dsa_44::PublicKey::from_bytes(&public_key_bytes);

    let is_valid = public_key.verify(&message, &signature_bytes, None);

    sp1_zkvm::io::commit(&is_valid);
}
