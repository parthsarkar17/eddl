use crate::ir::{
    constructs::port::{IOPort, Port},
    primitives::structures::Primitive,
};
use std::collections::HashMap;

pub enum Block {
    Primitive(Primitive),
    Component(Component),
}

pub struct Component {
    id: usize,
    wires: HashMap<Port, Vec<Port>>,
    port2block: HashMap<Port, Block>,
    in_ports: Vec<IOPort>,
    out_ports: Vec<IOPort>,
}

impl Component {
    pub fn new(
        id: usize,
        wires: HashMap<Port, Vec<Port>>,
        port2block: HashMap<Port, Block>,
        in_ports: Vec<IOPort>,
        out_ports: Vec<IOPort>,
    ) -> Self {
        Self {
            id,
            wires,
            port2block,
            in_ports,
            out_ports,
        }
    }
}
