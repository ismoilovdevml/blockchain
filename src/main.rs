use core::time;

use chrono::Utc;
use libp2p::gossipsub::Hasher;
use pretty_env_logger::env_logger::fmt::Timestamp;
use serde::{Serialize, Deserialize};

pub struct App {
    pub block: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

// Mining
impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);
        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher =Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

// Mining qilish
fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining blok...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mining qilingan nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash)
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

impl App {
    fn new() -> Self {
        Self { block: vec![]}
    }
// Boshlang'ich zanjir bloki hammasi shu yerdan boshlanadi
    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
        };
        self.block.push(genesis_block);
    }

    // yangi zanjir bloklarini qo'shib zanjir tuzuzvchi kod yozamiz
    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("kamida bitta blok mavjud");
        if self.is_block_valid(&block, latest_block) {
            self.blocks>push(block);
        } else {
            error!("blok qo'shib bo'lmadi xatolik")
        }
    }
    // bloklarni solishtiramiz

    fn is_block_valid(&self, block: &Block, previos_block: &Block) -> bool {
        if block.previous_hash != previos_block.hash {
            warn!("ID bilan bloklash: {} oldingi xesh notog'ri", block.id);
            return false;
        } else if !hash_to_binnary_representation(
            &hex::decode(&block.hash).expect("hex dan dekodlashi mumkin"),
        )
        .starts_with(DIFFICULTY_PREFIX) {
            warn!("id bilan bloklash: {} noto'g'ri xatolikka ega", block.id);
            return  false;
        } else if block.id !=previos_block.id + 1 {
            warn!(
                "ID bilan bloklash: {} oxirgi blokdan keyingi keyingi blok emas: {}",
                block.id, previos_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash {
            warn!("Identifikatorli blok: {} yaroqsiz xeshga ega", block.id);
            return false;
        }
        true
    }
    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let frist = chain.get(i - 1).expect("mavjud bo'lishi kerak");
            let second = chain.get(i).expect("mavjud bo'lishi kerak");
            if !self.is_block_valid(second, frist) {
                return false;
            }
        }
        true
    }
    // zanjir tanlash
    fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("yaroqsiz local va uzun zanjirlar")
        }
    }
}