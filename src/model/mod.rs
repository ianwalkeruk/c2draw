pub mod diagram;
pub mod elements;
pub mod relationship;

pub use diagram::{Diagram, DiagramType};
pub use elements::{ContainerType, Element, ElementType};
pub use relationship::Relationship;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Version of the diagram file format
pub const FILE_FORMAT_VERSION: &str = "1.0";

/// Unique identifier for diagram elements
pub type ElementId = Uuid;

/// Position on the canvas (x, y)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_pos2(&self) -> egui::Pos2 {
        egui::Pos2::new(self.x, self.y)
    }

    pub fn from_pos2(pos: egui::Pos2) -> Self {
        Self::new(pos.x, pos.y)
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Add<egui::Vec2> for Position {
    type Output = Self;

    fn add(self, rhs: egui::Vec2) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Position {
    type Output = egui::Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        egui::Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// Size of an element on the canvas (width, height)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn to_vec2(&self) -> egui::Vec2 {
        egui::Vec2::new(self.width, self.height)
    }

    pub fn from_vec2(vec: egui::Vec2) -> Self {
        Self::new(vec.x, vec.y)
    }
}

impl std::ops::Mul<f32> for Size {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

/// Trait for elements that can be positioned on the canvas
pub trait Positioned {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn size(&self) -> Size;
    fn set_size(&mut self, size: Size);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper struct for Positioned trait
    struct TestPositionable {
        pos: Position,
        sz: Size,
    }

    impl TestPositionable {
        fn new(pos: Position, sz: Size) -> Self {
            Self { pos, sz }
        }
    }

    impl Positioned for TestPositionable {
        fn position(&self) -> Position {
            self.pos
        }

        fn set_position(&mut self, position: Position) {
            self.pos = position;
        }

        fn size(&self) -> Size {
            self.sz
        }

        fn set_size(&mut self, size: Size) {
            self.sz = size;
        }
    }

    mod position_tests {
        use super::*;

        /// Verifies Position::new creates a position with correct coordinates
        #[test]
        fn position_new_creates_correct_position() {
            let pos = Position::new(10.0, 20.0);
            assert_eq!(pos.x, 10.0);
            assert_eq!(pos.y, 20.0);
        }

        /// Verifies Position can be converted to egui::Pos2 and back
        #[test]
        fn position_to_pos2_roundtrip() {
            let pos = Position::new(15.5, 25.5);
            let pos2 = pos.to_pos2();
            assert_eq!(pos2.x, 15.5);
            assert_eq!(pos2.y, 25.5);

            let back = Position::from_pos2(pos2);
            assert_eq!(back.x, 15.5);
            assert_eq!(back.y, 25.5);
        }

        /// Verifies Position addition works correctly
        #[test]
        fn position_addition() {
            let pos1 = Position::new(10.0, 20.0);
            let pos2 = Position::new(5.0, 8.0);
            let result = pos1 + pos2;
            assert_eq!(result.x, 15.0);
            assert_eq!(result.y, 28.0);
        }

        /// Verifies Position addition with egui::Vec2 works correctly
        #[test]
        fn position_add_vec2() {
            let pos = Position::new(10.0, 20.0);
            let vec = egui::Vec2::new(5.0, 8.0);
            let result = pos + vec;
            assert_eq!(result.x, 15.0);
            assert_eq!(result.y, 28.0);
        }

        /// Verifies Position subtraction returns correct egui::Vec2
        #[test]
        fn position_subtraction() {
            let pos1 = Position::new(10.0, 20.0);
            let pos2 = Position::new(3.0, 5.0);
            let result = pos1 - pos2;
            assert_eq!(result.x, 7.0);
            assert_eq!(result.y, 15.0);
        }

        /// Verifies Position scalar multiplication works correctly
        #[test]
        fn position_scalar_multiplication() {
            let pos = Position::new(10.0, 20.0);
            let result = pos * 2.0;
            assert_eq!(result.x, 20.0);
            assert_eq!(result.y, 40.0);
        }

    }

    mod size_tests {
        use super::*;

        /// Verifies Size::new creates a size with correct dimensions
        #[test]
        fn size_new_creates_correct_size() {
            let size = Size::new(100.0, 200.0);
            assert_eq!(size.width, 100.0);
            assert_eq!(size.height, 200.0);
        }

        /// Verifies Size can be converted to egui::Vec2 and back
        #[test]
        fn size_to_vec2_roundtrip() {
            let size = Size::new(150.5, 250.5);
            let vec2 = size.to_vec2();
            assert_eq!(vec2.x, 150.5);
            assert_eq!(vec2.y, 250.5);

            let back = Size::from_vec2(vec2);
            assert_eq!(back.width, 150.5);
            assert_eq!(back.height, 250.5);
        }

        /// Verifies Size scalar multiplication scales both dimensions
        #[test]
        fn size_scalar_multiplication() {
            let size = Size::new(100.0, 200.0);
            let result = size * 0.5;
            assert_eq!(result.width, 50.0);
            assert_eq!(result.height, 100.0);
        }

    }

    mod positioned_trait_tests {
        use super::*;

        /// Verifies Positioned trait getters work correctly
        #[test]
        fn positioned_getters() {
            let test = TestPositionable::new(
                Position::new(10.0, 20.0),
                Size::new(100.0, 200.0),
            );

            assert_eq!(test.position().x, 10.0);
            assert_eq!(test.position().y, 20.0);
            assert_eq!(test.size().width, 100.0);
            assert_eq!(test.size().height, 200.0);
        }

        /// Verifies Positioned trait setters work correctly
        #[test]
        fn positioned_setters() {
            let mut test = TestPositionable::new(
                Position::new(0.0, 0.0),
                Size::new(50.0, 50.0),
            );

            test.set_position(Position::new(30.0, 40.0));
            test.set_size(Size::new(150.0, 250.0));

            assert_eq!(test.position().x, 30.0);
            assert_eq!(test.position().y, 40.0);
            assert_eq!(test.size().width, 150.0);
            assert_eq!(test.size().height, 250.0);
        }
    }
}
