use num_derive::{ToPrimitive, FromPrimitive};

#[derive(FromPrimitive, ToPrimitive)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubstract,
    OpMultiply,
    OpDivide,
}
