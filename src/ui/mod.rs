pub mod canvas;

use crate::model::{ContainerType, Element, ElementType, Position};
use egui::{Color32, Rect, Response, StrokeKind, Ui};

/// Get default position for new elements
pub fn default_element_position(index: usize) -> Position {
    let col = index % 3;
    let row = index / 3;
    Position::new(50.0 + col as f32 * 200.0, 50.0 + row as f32 * 150.0)
}

/// Get colors for an element based on its type and selection state
pub fn element_colors(element: &Element, is_selected: bool) -> (Color32, Color32) {
    let border = if is_selected {
        Color32::from_rgb(0, 120, 215)
    } else {
        Color32::from_gray(150)
    };

    let bg = match &element.element_type {
        ElementType::Person(data) => {
            if data.is_external {
                Color32::from_rgb(255, 240, 220)
            } else {
                Color32::from_rgb(255, 220, 180)
            }
        }
        ElementType::SoftwareSystem(data) => {
            if data.is_external {
                Color32::from_rgb(230, 230, 230)
            } else {
                Color32::from_rgb(200, 220, 255)
            }
        }
        ElementType::Container(data) => {
            match data.container_type {
                ContainerType::Database => Color32::from_rgb(200, 255, 200),
                ContainerType::Queue => Color32::from_rgb(255, 255, 200),
                _ => Color32::from_rgb(220, 240, 255),
            }
        }
    };

    (bg, border)
}

/// Get icon for element type
pub fn get_element_icon(element: &Element) -> &'static str {
    match &element.element_type {
        ElementType::Person(_) => "👤",
        ElementType::SoftwareSystem(_) => "🖥️",
        ElementType::Container(data) => match data.container_type {
            ContainerType::Database => "🗄️",
            ContainerType::MobileApp => "📱",
            ContainerType::Queue => "📨",
            _ => "📦",
        },
    }
}
