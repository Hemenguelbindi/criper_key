/// This is crypto
const BLOCK_SIZE: usize = 16;
const KEY_SIZE: usize = 32;
const ROUNDS:usize = 10;


type Block = [u8; BLOCK_SIZE];
type Key = [u8; KEY_SIZE];


pub fn xor_blocks(a: &mut [i32; 2], b: [i32; 2]) {
    for i in 0..a.len(){
        a[i] ^= b[i];
    }
}

pub struct Kuznechik{
    round_keys: Vec<Block>
}

#[derive(Debug)]
enum KuznechikError{
    InvalidKeyLength,
    InvalidBlockLength,
}

impl Kuznechik{
    pub fn new(key: &[u8]) -> Result<Self, KuznechikError> {
        if key.len() != KEY_SIZE {
            return Err(KuznechikError::InvalidKeyLength);
        }

        Ok(Kuznechik {
            round_keys: Self::generate_round_keys(<&Key>::try_from(key).unwrap()),
        })
    }

    fn generate_round_keys(key: &Key) -> Vec<Block> {
        let mut round_keys = Vec::with_capacity(ROUNDS);
        for i in 0..ROUNDS {
            let mut round_key = [0u8; BLOCK_SIZE];
            for j in 0..BLOCK_SIZE {
                round_key[j] = key[(i * BLOCK_SIZE + j) % KEY_SIZE];
            }
            round_keys.push(round_key);
        }
        round_keys
    }
    fn round_function(&self, block: &mut Block, round_key: &Block){
        for i in 0..BLOCK_SIZE {
            block[i] = round_key[i]
        }
    }
    pub fn encrypt(&self, block: &mut Block) {
        for round_key in &self.round_keys {
            self.round_function(block, round_key);
        }
    }


    pub fn decrypt(&self, block: &mut Block) {
        for round_key in self.round_keys.iter().rev() {
            self.round_function(block, round_key);
        }
    }

}