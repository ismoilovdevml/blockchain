use chrono::Utc;
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
}