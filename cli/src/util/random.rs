use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn generate_rand() -> ChaCha20Rng {
	ChaCha20Rng::from_rng(rand::thread_rng()).unwrap()
}

pub fn generate_rand_slice() -> [u8; 16] {
	let mut dest: [u8; 16] = [0; 16];
	generate_rand().fill_bytes(&mut dest);
	dest
}
