use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtEnum>,
}

impl Block {
    pub fn new(statements: Vec<StmtEnum>) -> Self {
        Self { statements }
    }
}

impl From<Block> for StmtEnum {
    fn from(value: Block) -> Self {
        StmtEnum::Block(value)
    }
}
