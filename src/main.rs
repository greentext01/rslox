mod chunk;
mod opcode;
mod values;
mod instructions;

fn main() {
    let mut chunk = chunk::Chunk::new();
    let constant_ref = chunk.add_constant(&(1.2));
    chunk.add_instruction(&(opcode::OpCode::OpConstant as u8), 1);
    // TODO: Replace this
    chunk.add_instruction(&((constant_ref % 255) as u8), 1);
    chunk.add_instruction(&(opcode::OpCode::OpReturn as u8), 1);
    chunk.disassemble("Test chunk");
}
