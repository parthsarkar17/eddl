use std::hash::{Hash, Hasher};
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    In,
    Out,
}

#[derive(Debug)]
pub struct Port {
    id: usize,
    width: u8,
    name: &'static str,
    direction: Direction,
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Port {
    pub fn new(id: usize, width: u8, name: &'static str, out: bool) -> Self {
        Self {
            id,
            width,
            name,
            direction: if out { Direction::Out } else { Direction::In },
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}
