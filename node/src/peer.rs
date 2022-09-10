use crate::{
    database::Database,
    util::{
        execution::{sleep_millis, Runnable},
        Config,
    },
};
use anyhow::Result;
use isahc::{ReadResponseExt, Request};
use spec::types::Block;
use std::panic;

pub struct Peer {
    peer_addresses: Vec<String>,
    peer_sync_ms: u64,
    database: Database,
}

impl Runnable for Peer {
    fn run(&self) -> Result<()> {
        self.start()
    }
}

impl Peer {
    pub fn new(config: &Config, database: &Database) -> Peer {
        Peer {
            peer_addresses: config.peers.clone(),
            peer_sync_ms: config.peer_sync_ms,
            database: database.clone(),
        }
    }

    pub fn start(&self) -> Result<()> {
        if self.peer_addresses.is_empty() {
            info!("No peers configured, exiting peer sync system");
            return Ok(());
        }

        info!(
            "start peer system with peers: {}",
            self.peer_addresses.join(", ")
        );

        // At regular intervals of time, we try to sync new blocks from our peers
        let mut last_sent_block_index = self.get_last_block_index();
        loop {
            self.try_receive_new_blocks();
            self.try_send_new_blocks(last_sent_block_index);
            last_sent_block_index = self.get_last_block_index();
            sleep_millis(self.peer_sync_ms);
        }
    }

    fn get_last_block_index(&self) -> usize {
        self.database.get_last_block().index as usize
    }

    // Retrieve new blocks from all peers and add them to the blockchain
    fn try_receive_new_blocks(&self) {
        for address in self.peer_addresses.iter() {
            let new_blocks = self.get_new_blocks_from_peer(address);

            if !new_blocks.is_empty() {
                self.add_new_blocks(&new_blocks);
            }
        }
    }

    // Try to add a bunch of new blocks to our blockchain
    fn add_new_blocks(&self, new_blocks: &[Block]) {
        for block in new_blocks.iter() {
            let result = self.database.add_block(block.clone());

            // if a block is invalid, no point in trying to add the next ones
            if result.is_err() {
                error!("Could not add peer block {} to the blockchain", block.index);
                return;
            }

            info!("Added new peer block {} to the blockchain", block.index);
        }
    }

    // Retrieve only the new blocks from a peer
    fn get_new_blocks_from_peer(&self, address: &str) -> Vec<Block> {
        // we need to know the last block index in our blockchain
        // FIXME: we should use a u64 range to avoid overflows
        let our_last_index = self.database.get_last_block().index as usize;

        // we retrieve all the blocks from the peer
        let peer_blocks = self.get_blocks_from_peer(address);
        let peer_last_index = match peer_blocks.last() {
            Some(block) => block.index as usize,
            None => 0,
        };

        // The peer do have new blocks, and we return ONLY the new ones
        let first_new = our_last_index + 1;
        let last_new = peer_last_index;
        let new_blocks_range = first_new..=last_new;
        peer_blocks
            .get(new_blocks_range)
            .unwrap_or_default()
            .to_vec()
    }

    // Retrieve ALL blocks from a peer
    // if the peer is not responsive, we ignore it and return and empty vector
    fn get_blocks_from_peer(&self, address: &str) -> Vec<Block> {
        let uri = format!("{}/blocks", address);
        let default_value = vec![];

        let mut response = match isahc::get(uri) {
            Ok(value) => value,
            Err(_) => return default_value,
        };

        // check that the response is sucessful
        if response.status().as_u16() != 200 {
            return default_value;
        }

        // parse and return the list of blocks from the response body
        let raw_body = response.text().unwrap_or_default();
        serde_json::from_str(&raw_body).unwrap_or_default()
    }

    // Try to broadcast all new blocks to peers since last time we broadcasted
    fn try_send_new_blocks(&self, last_send_block_index: usize) {
        let new_blocks = self.get_new_blocks_since(last_send_block_index);

        for block in new_blocks.iter() {
            for address in self.peer_addresses.iter() {
                // we don't want to panic if one peer is down or not working properly
                let result = panic::catch_unwind(|| {
                    Peer::send_block_to_peer(address, block);
                });

                if result.is_err() {
                    error!("Could not send block {} to peer {}", block.index, address);
                    return;
                }

                info!("Sended new block {} to peer {}", block.index, address);
            }
        }
    }

    // Return all new blocks added to the blockchain since the one with the indicated index
    fn get_new_blocks_since(&self, start_index: usize) -> Vec<Block> {
        let last_block_index = self.get_last_block_index();
        let new_blocks_range = start_index + 1..=last_block_index;
        self.database
            .get_all_blocks()
            .get(new_blocks_range)
            .unwrap()
            .to_vec()
    }

    // Send a block to a peer using the REST API of the peer
    fn send_block_to_peer(address: &str, block: &Block) {
        let uri = format!("{}/blocks", address);
        let body = serde_json::to_string(&block).unwrap();

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        // Ignore unresponsive peers
        let _response = isahc::send(request);
    }
}
