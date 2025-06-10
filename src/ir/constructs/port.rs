use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    In,
    Out,
}

#[derive(Debug)]
pub struct DirectedPort {
    id: Option<usize>,
    width: u8,
    name: Option<&'static str>,
    direction: Direction,
}

impl DirectedPort {
    pub fn new(id: Option<usize>, width: u8, name: Option<&'static str>, out: bool) -> Self {
        Self {
            id,
            width,
            name,
            direction: if out { Direction::Out } else { Direction::In },
        }
    }

    pub fn id(&self) -> Option<usize> {
        self.id
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn name(&self) -> Option<&str> {
        self.name
    }
}

#[derive(Debug)]
pub struct IOPort {
    id: usize,
    name: &'static str,
    input: DirectedPort,
    output: DirectedPort,
}

impl IOPort {
    pub fn new(id: usize, name: &'static str, input: DirectedPort, output: DirectedPort) -> Self {
        Self {
            id,
            name,
            input,
            output,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn width(&self) -> u8 {
        self.input.width()
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

#[derive(Debug)]
pub enum Port {
    DirectedPort(DirectedPort),
    IOPort(IOPort),
}

impl Port {
    pub fn id(&self) -> usize {
        match self {
            Self::DirectedPort(dp) => dp.id.expect("cannot hash a directed port without id"),
            Self::IOPort(iop) => iop.id,
        }
    }
    pub fn width(&self) -> u8 {
        match self {
            Self::DirectedPort(dp) => dp.width(),
            Self::IOPort(iop) => iop.width(),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::DirectedPort(dp) => dp
                .name()
                .expect("called `name()` on directed port without name"),
            Self::IOPort(iop) => iop.name(),
        }
    }
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}

impl Eq for Port {}
