use spec::types::Block;

#[derive(Debug, Clone, Default)]
pub struct BlockDatabase {
    blocks: Vec<Block>,
}

impl BlockDatabase {
    pub fn get_all_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn get_tip_block(&self) -> Option<Block> {
        self.blocks.last().cloned()
    }

    pub fn append_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
