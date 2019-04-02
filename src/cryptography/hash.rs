use sha3;
use blake2;
use digest::Digest;
use tiny_keccak::Keccak;
use cryptography::HASH_ALGORITHM;


/*---- ALLOWED HASH ALGORITHMS ----*/

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    Blake2b,
    Blake2s,
    Sha3_256,
    Sha3_512,
    Keccak256,
    Keccak512
}


/// Hash a completely available message
/// 
/// ### Arguments
///
/// * `message` - Message to sign
/// * `protocol` - Hash protocol to use
pub fn hash_message(message: &[u8]) -> Vec<u8> {
    let result = match HASH_ALGORITHM {
        HashAlgorithm::Blake2b => blake2::Blake2b::digest(message).to_vec(),
        HashAlgorithm::Blake2s => blake2::Blake2s::digest(message).to_vec(),
        HashAlgorithm::Sha3_256 => sha3::Sha3_256::digest(message).to_vec(),
        HashAlgorithm::Sha3_512 => sha3::Sha3_512::digest(message).to_vec(),
        HashAlgorithm::Keccak256 => {
            let mut keccak = Keccak::new_keccak256();
            let mut res: [u8; 32] = [0; 32];

            keccak.update(&message);
            keccak.finalize(&mut res);

            return res.to_vec();
        },
        HashAlgorithm::Keccak512 => {
            let mut keccak = Keccak::new_keccak512();
            let mut res: [u8; 64] = [0; 64];

            keccak.update(&message);
            keccak.finalize(&mut res);

            return res.to_vec();
        }
    };

    result
}