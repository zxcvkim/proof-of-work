use std::time::Instant;

use proof_of_work::{Block, BlockChain};

fn main() {
    let chain = BlockChain::new();

    let number_of_blocks = 100;
    let difficulty = 4;

    for i in 1..=number_of_blocks {
        let start_time = Instant::now();

        let prev_hash = chain.latest_block().unwrap().hash();
        let content = format!("Block #{}", i);
        let mut new_block = Block::new(prev_hash, content.clone(), difficulty, 0);

        if new_block.mine(u64::MAX) {
            let duration = start_time.elapsed();
            let message = format!(
                "{} mined successfully! (Nonce: {}, Time: {:.3}s)",
                content,
                new_block.nonce(),
                duration.as_secs_f32()
            );
            println!("{}", message);
        }
    }
}
