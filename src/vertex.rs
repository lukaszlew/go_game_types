pub const MAX_GOBAN_SIZE: usize = 19;
pub const LEFT_SENTINELS: usize = 1;
pub const RIGHT_SENTINELS: usize = 1;
pub const TOP_SENTINELS: usize = 1;
pub const BOTTOM_SENTINELS: usize = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vertex(u32);

impl Vertex {
    pub const ROW_SIZE: usize = MAX_GOBAN_SIZE + LEFT_SENTINELS + RIGHT_SENTINELS;
    pub const COUNT_ON_BOARD: usize = Self::ROW_SIZE * (MAX_GOBAN_SIZE + TOP_SENTINELS + BOTTOM_SENTINELS);
    pub const COUNT: usize = Self::COUNT_ON_BOARD + 2;

    pub fn from_coords(row: isize, column: isize) -> Self {
        // Add sentinels: actual board position is (row + TOP_SENTINELS, column + LEFT_SENTINELS)
        let actual_row = row + TOP_SENTINELS as isize;
        let actual_column = column + LEFT_SENTINELS as isize;
        let index = actual_row * Self::ROW_SIZE as isize + actual_column;
        assert!(
            index >= 0 && index < Self::COUNT_ON_BOARD as isize,
            "Coordinates ({}, {}) produce invalid index",
            row,
            column
        );
        Vertex(index as u32)
    }

    pub fn row(self) -> isize {
        let actual_row = (self.0 as isize) / Self::ROW_SIZE as isize;
        // Subtract sentinel to get board coordinate
        actual_row - TOP_SENTINELS as isize
    }

    pub fn column(self) -> isize {
        let actual_column = (self.0 as isize) % Self::ROW_SIZE as isize;
        // Subtract sentinel to get board coordinate
        actual_column - LEFT_SENTINELS as isize
    }

    pub fn pass() -> Self {
        Vertex(Self::COUNT_ON_BOARD as u32)
    }

    pub fn none() -> Self {
        Vertex((Self::COUNT_ON_BOARD + 1) as u32)
    }

    pub fn up(self) -> Self {
        Vertex(self.0 - Self::ROW_SIZE as u32)
    }

    pub fn down(self) -> Self {
        Vertex(self.0 + Self::ROW_SIZE as u32)
    }

    pub fn left(self) -> Self {
        Vertex(self.0 - 1)
    }

    pub fn right(self) -> Self {
        Vertex(self.0 + 1)
    }
}

impl From<u32> for Vertex {
    fn from(value: u32) -> Self {
        assert!(
            value < Vertex::COUNT as u32,
            "Vertex value {} exceeds maximum {}",
            value,
            Vertex::COUNT as u32 - 1
        );
        Vertex(value)
    }
}

impl From<Vertex> for u32 {
    fn from(vertex: Vertex) -> Self {
        vertex.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_inverse() {
        // Test valid board positions
        for row in 0..MAX_GOBAN_SIZE as isize {
            for col in 0..MAX_GOBAN_SIZE as isize {
                let v = Vertex::from_coords(row, col);
                assert_eq!(v.row(), row, "Row mismatch at ({}, {})", row, col);
                assert_eq!(v.column(), col, "Column mismatch at ({}, {})", row, col);
            }
        }
        
        // Test sentinel positions
        let left_sentinel = -(LEFT_SENTINELS as isize);
        let top_sentinel = -(TOP_SENTINELS as isize);
        let right_edge = MAX_GOBAN_SIZE as isize;
        let bottom_edge = MAX_GOBAN_SIZE as isize;
        
        let sentinel_coords = [
            // Top-left corner
            (top_sentinel, left_sentinel),
            // Top edge
            (top_sentinel, 0), (top_sentinel, 5), (top_sentinel, MAX_GOBAN_SIZE as isize - 1), (top_sentinel, right_edge),
            // Left edge
            (0, left_sentinel), (5, left_sentinel), (MAX_GOBAN_SIZE as isize - 1, left_sentinel),
            // Right edge
            (0, right_edge), (5, right_edge), (MAX_GOBAN_SIZE as isize - 1, right_edge),
            // Bottom edge
            (bottom_edge, left_sentinel), (bottom_edge, 0), (bottom_edge, 5), (bottom_edge, MAX_GOBAN_SIZE as isize - 1),
            // Bottom-right corner
            (bottom_edge, right_edge),
        ];
        
        for &(row, col) in &sentinel_coords {
            let v = Vertex::from_coords(row, col);
            assert_eq!(v.row(), row, "Row mismatch at sentinel ({}, {})", row, col);
            assert_eq!(v.column(), col, "Column mismatch at sentinel ({}, {})", row, col);
        }
    }
    
    #[test]
    fn test_special_vertices() {
        // Pass and none should have consistent coordinates
        let pass = Vertex::pass();
        let pass_row = pass.row();
        let pass_col = pass.column();
        
        let none = Vertex::none();
        let none_row = none.row();
        let none_col = none.column();
        
        // These special vertices are beyond the normal board
        // Just verify they don't panic and have reasonable values
        assert!(pass_row >= 0 || pass_col >= MAX_GOBAN_SIZE as isize);
        assert!(none_row >= 0 || none_col >= MAX_GOBAN_SIZE as isize);
    }
}
