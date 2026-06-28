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
        }

        false
    }
}
