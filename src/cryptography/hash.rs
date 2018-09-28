use sha2::{ Sha256, Digest };

/**
 * Handles hashing and such. Project-wide changes to the hash 
 * function and processes can all be made in this file
 */

fn hash(input: &Vec<u8>) -> GenericArray<u8, U64> {
    let mut hasher = Sha256::new();
    hasher.input(input);

    hasher.result()
}