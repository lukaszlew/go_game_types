pub const MAX_GOBAN_SIZE: usize = 19;
pub const LEFT_SENTINELS: usize = 1;
pub const RIGHT_SENTINELS: usize = 1;
pub const TOP_SENTINELS: usize = 1;
pub const BOTTOM_SENTINELS: usize = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vertex(u32);

impl Vertex {
    pub const COUNT: usize = (MAX_GOBAN_SIZE + LEFT_SENTINELS + RIGHT_SENTINELS) * (MAX_GOBAN_SIZE + TOP_SENTINELS + BOTTOM_SENTINELS) + 2;
}

impl From<u32> for Vertex {
    fn from(value: u32) -> Self {
        assert!(value < Vertex::COUNT as u32, "Vertex value {} exceeds maximum {}", value, Vertex::COUNT as u32 - 1);
        Vertex(value)
    }
}

impl From<Vertex> for u32 {
    fn from(vertex: Vertex) -> Self {
        vertex.0
    }
}