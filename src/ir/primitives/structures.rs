use crate::ir::constructs::port::Port;

pub enum GateType {
    And,
    Or,
    Xor,
    Xnor,
    Nand,
}

pub struct Gate {
    width: u8,
    operation: GateType,
    inputs: Vec<Port>,
    output: Port,
}

pub struct Constant {
    width: u8,
    value: u8,
    output: Port,
}

pub struct Register {
    width: u8,
    clk: Port,
    input: Port,
    output: Port,
}

pub enum Primitive {
    Register(Register),
    Constant(Constant),
    Gate(Gate),
}
