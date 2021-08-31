// TODO: WIP
use crate::util::random;
use aes::cipher::{
	generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher,
};
use aes::{Aes256, Block, ParBlocks};

// let key = GenericArray::from_slice(&[0u8; 16]);
// let mut block = Block::default();
// let mut block8 = ParBlocks::default();

// // Initialize cipher
// let cipher = Aes256::new(&key);

// let block_copy = block.clone();

// // Encrypt block in-place
// cipher.encrypt_block(&mut block);

// // And decrypt it back
// cipher.decrypt_block(&mut block);
// assert_eq!(block, block_copy);

pub fn generate_symmetric_key() {
	let slice = random::generate_rand_slice();
	let key = GenericArray::from_slice(&slice);
	let cipher = Aes256::new(&key);
}
