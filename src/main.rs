pub mod ir;

use ir::{
    constructs::{
        component::Component,
        port::{DirectedPort, IOPort, Port},
    },
    primitives::structures::{Gate, GateType},
};

use std::collections::HashMap;

fn main() {
    let (in1, in2, out) = (
        Port::IOPort(IOPort::new(
            0,
            "equality_in1",
            DirectedPort::new(None, 32, None, false),
            DirectedPort::new(None, 32, None, true),
        )),
        Port::IOPort(IOPort::new(
            1,
            "equality_in2",
            DirectedPort::new(None, 32, None, false),
            DirectedPort::new(None, 32, None, true),
        )),
        Port::IOPort(IOPort::new(
            2,
            "equality_out",
            DirectedPort::new(None, 32, None, false),
            DirectedPort::new(None, 32, None, true),
        )),
    );

    let xnor_gate = Gate::new(
        32,
        GateType::Xnor,
        vec![
            DirectedPort::new(Some(3), 32, Some("xnor_in1"), false),
            DirectedPort::new(Some(4), 32, Some("xnor_in2"), false),
        ],
        DirectedPort::new(Some(5), 32, Some("xnor_out"), true),
    );

    let equality = Component::new(
        0,
        HashMap::from_iter(vec![]),
        HashMap::from_iter(vec![]),
        vec![],
        vec![],
    );

    println!("Hello, world!");
}
