use std::{panic};

use crate::{model::{Block, Blockchain}, util::{Context, execution::{Runnable, sleep_millis}}};
use anyhow::Result;
use isahc::ReadResponseExt;

pub struct Peer {
    peer_addresses: Vec<String>,
    blockchain: Blockchain,
    peer_sync_ms: u64,
}

impl Runnable for Peer {
    fn run(&self) -> Result<()> {
        self.start()
    }
}

impl Peer {
    pub fn new(context: &Context) -> Peer {
        Peer {
            peer_addresses: context.config.peers.clone(),
            blockchain: context.blockchain.clone(),
            peer_sync_ms: context.config.peer_sync_ms,
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
        loop {
            self.sync_blocks();
            sleep_millis(self.peer_sync_ms);
        }
    }

    // Retrieve new blocks from all peers and add them to the blockchain
    fn sync_blocks(&self) {
        for address in self.peer_addresses.iter() {
            // we don't want to panic if one peer is down or not working properly
            let result = panic::catch_unwind(|| {
                let new_blocks = self.get_new_blocks_from_peer(address);

                if !new_blocks.is_empty() {
                    self.add_new_blocks(&new_blocks);
                    info!("Added {} new blocks from the peer at {}", new_blocks.len(), address);
                }
            });

            // if a peer is not working, we simply log it and ignore the error
            if result.is_err() {
                error!("Could not sync blocks from peer {}", address);
            }
        }
    }

    // Try to add a bunch of new blocks to our blockchain
    fn add_new_blocks(&self, new_blocks: &[Block]) {
        for block in new_blocks.iter() {
            let result = self.blockchain.add_block(block.clone());

            // if a block is invalid, no point in trying to add the next ones
            if result.is_err() {
                error!("Could not add peer block {} to the blockchain", block.index);
                return;
            }
        }
    }

    // Retrieve only the new blocks from a peer
    fn get_new_blocks_from_peer(&self, address: &str) -> Vec<Block> {
        // we need to know the last block index in our blockchain
        let our_last_index = self.blockchain.get_last_block().index as usize;

        // we retrieve all the blocks from the peer
        let peer_blocks = self.get_blocks_from_peer(address);
        let peer_last_index = peer_blocks.last().unwrap().index as usize;
                
        // Check if the peer has new blocks
        if peer_last_index <= our_last_index {
            return Vec::<Block>::new();
        }

        // The peer do have new blocks, and we return ONLY the new ones
        let first_new = our_last_index + 1;
        let last_new = peer_last_index;
        let new_blocks_range = first_new..=last_new;
        peer_blocks.get(new_blocks_range).unwrap().to_vec()
    }

    // Retrieve ALL blocks from a peer
    fn get_blocks_from_peer(&self, address: &str) -> Vec<Block> {
        let uri = format!("{}/blocks", address);
        let mut response = isahc::get(uri).unwrap();
    
        // check that the response is sucessful
        assert_eq!(response.status().as_u16(), 200);
    
        // parse and return the list of blocks from the response body
        let raw_body = response.text().unwrap();
        serde_json::from_str(&raw_body).unwrap()
    }
}