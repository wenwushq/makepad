use crate::fish_param_storage::*;
use crate::fish_ports::*;
use crate::makepad_micro_serde::*;

#[derive(Clone, Debug, SerRon, DeRon, Default)]
pub struct FishBlock{
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub block_type: String,
    pub parameters: Vec<FishParamStorage>,
    pub input_ports: Vec<FishInputPort>,
    pub output_ports: Vec<FishOutputPort>
}

impl FishBlock{

    pub fn create_test_block(id: i32) -> FishBlock
    {
        let mut block = FishBlock::default();
        block.block_type = String::from(format!("BlockType {:?}", id));
        block.id = id;
    
        block
    }
}