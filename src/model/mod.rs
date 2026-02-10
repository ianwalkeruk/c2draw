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
