use crate::ir::constructs::port::DirectedPort;

pub enum GateType {
    And,
    Or,
    Xor,
    Xnor,
    Nand,
    Not,
}

pub struct Gate {
    width: u8,
    operation: GateType,
    inputs: Vec<DirectedPort>,
    output: DirectedPort,
}

impl Gate {
    pub fn new(
        width: u8,
        operation: GateType,
        inputs: Vec<DirectedPort>,
        output: DirectedPort,
    ) -> Self {
        Self {
            width,
            operation,
            inputs,
            output,
        }
    }
}

pub struct Constant {
    width: u8,
    value: u8,
    output: DirectedPort,
}

pub struct Register {
    width: u8,
    clk: DirectedPort,
    input: DirectedPort,
    output: DirectedPort,
}

pub enum PrimitiveType {
    Register(Register),
    Constant(Constant),
    Gate(Gate),
}

pub struct Primitive {
    id: usize,
    primitive_type: PrimitiveType,
}
