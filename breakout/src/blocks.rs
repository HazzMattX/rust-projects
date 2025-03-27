use macroquad::prelude::*;
const BLOCK_SIZE: Vec2 = Vec2::new(120.0, 40.0);
#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath, // Block spawns a ball when removed
}
pub struct Block {
    pub rect: Rect,
    pub lives: u8,
    pub block_type: BlockType
}
impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
            Self {
                rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
                lives: 2,
                block_type,
            }
        }
    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular =>
                match self.lives {
                2 => RED,
                _ => ORANGE
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
pub fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5.0;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - total_block_size.x * width as f32) / 2.0 , 50.0);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), BlockType::Regular));
    }
    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}
