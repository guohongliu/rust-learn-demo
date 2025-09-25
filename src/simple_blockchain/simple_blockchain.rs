use chrono::Utc;
use sha2::Digest;

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, timestamp: i64, data: String, prev_hash: String) -> Self {
        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    fn hash(index: &u32, timestamp: i64, data: &String, prev_hash: &String, nonce: u32) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
        let mut hasher = sha2::Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine(index: u32, timestamp: i64, data: String, prev_hash: String, difficulty: usize) -> Self {
        let mut nonce = 0;
        let mut hash = String::new();
        let target = "0".repeat(difficulty);

        println!("⛏️  开始挖矿... 目标: {}", target);

        let start = std::time::Instant::now();

        loop {
            hash = Self::hash(&index, timestamp, &data, &prev_hash, nonce);
            if hash.starts_with(&target) {
                break;
            }
            nonce += 1;

            // 每100万次输出一次进度（可选）
            if nonce % 1_000_000 == 0 {
                print!("\r⛏️  尝试了 {} 次...", nonce);
            }
        }

        let duration = start.elapsed();
        println!("\n✅ 挖矿成功！Nonce: {}, 耗时: {:?}", nonce, duration);
        println!("🔗 Hash: {}", hash);

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            nonce,
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut bc = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        bc.genesis_block();
        bc
    }

    fn genesis_block(&mut self) {
        let block = Block::new(
            0,
            Utc::now().timestamp(),
            "Genesis block".to_string(),
            "0".to_string(),
        );
        self.chain.push(block);
    }

    fn lastest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn add_block(&mut self, data: String) {
        let block = Block::mine(
            self.lastest_block().index + 1,
            Utc::now().timestamp(),
            data,
            self.lastest_block().hash.clone(),
            self.difficulty,
        );
        self.chain.push(block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != Block::hash(
                &current.index,
                current.timestamp,
                &current.data,
                &current.prev_hash,
                current.nonce)
            { return false; }

            if current.prev_hash != previous.hash {return false;}
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        // 创建区块链，难度为4（要求哈希前4位是0）
        let mut bc = Blockchain::new(4);

        // 添加一些区块
        bc.add_block("转账 1 BTC 给 Alice".to_string());
        bc.add_block("转账 2 BTC 给 Bob".to_string());
        bc.add_block("转账 3 BTC 给 Carol".to_string());

        // 打印区块链
        println!("\n🧱 区块链信息:");
        for block in &bc.chain {
            println!("{:?}", block);
        }

        // 验证区块链
        println!("\n🔍 区块链是否有效? {}", bc.is_valid());
    }
}
