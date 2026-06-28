use crate::sha256::Hash;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    content: String,
    difficulty: usize,
    nonce: u64,
}

impl Block {
    pub fn new(prev_hash: Hash, content: String, difficulty: usize, nonce: u64) -> Self {
        Self {
            prev_hash,
            content,
            difficulty,
            nonce,
        }
    }

    pub fn prev_hash(&self) -> Hash {
        self.prev_hash
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn mine(&mut self, steps: u64) -> bool {
        let prefix = "0".repeat(self.difficulty);

        for _ in 0..steps {
            let data = format!("{}{}{}", self.prev_hash, self.content, self.nonce);
            let hash = Hash::hash(data.as_bytes());
            let hash_string = hash.to_string();

            if hash_string.starts_with(&prefix) {
                return true;
            }

            if let Some(new_nonce) = self.nonce.checked_add(1) {
                self.nonce = new_nonce;
            } else {
                self.nonce = 0;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_block = Block::new(Hash::zero(), "hello world!".to_string(), 3, 0);
        Self {
            blocks: vec![genesis_block],
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
