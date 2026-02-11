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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ContainerType, Element, ElementType, Position};

    mod default_element_position_tests {
        use super::*;

        /// Verifies default_element_position places first element at correct position
        #[test]
        fn default_element_position_first_element() {
            let pos = default_element_position(0);
            assert_eq!(pos.x, 50.0);
            assert_eq!(pos.y, 50.0);
        }

        /// Verifies default_element_position places second element in same row
        #[test]
        fn default_element_position_second_element() {
            let pos = default_element_position(1);
            assert_eq!(pos.x, 250.0); // 50 + 200
            assert_eq!(pos.y, 50.0);
        }

        /// Verifies default_element_position places fourth element (index 3) in second row
        #[test]
        fn default_element_position_fourth_element() {
            // Index 3: col = 3 % 3 = 0, row = 3 / 3 = 1
            let pos = default_element_position(3);
            assert_eq!(pos.x, 50.0); // col 0
            assert_eq!(pos.y, 200.0); // row 1: 50 + 1*150 = 200
        }

        /// Verifies correct grid layout calculation
        #[test]
        fn default_element_position_row_wrap() {
            // Index 3 should be first element of second row
            let pos_3 = default_element_position(3);
            assert_eq!(pos_3.x, 50.0); // col = 3 % 3 = 0
            assert_eq!(pos_3.y, 200.0); // row = 3 / 3 = 1, so 50 + 1*150 = 200
        }

        /// Verifies correct grid layout calculation
        #[test]
        fn default_element_position_grid_layout() {
            // Row 0
            let pos_0 = default_element_position(0);
            assert_eq!(pos_0.x, 50.0);
            assert_eq!(pos_0.y, 50.0);

            let pos_1 = default_element_position(1);
            assert_eq!(pos_1.x, 250.0);
            assert_eq!(pos_1.y, 50.0);

            let pos_2 = default_element_position(2);
            assert_eq!(pos_2.x, 450.0);
            assert_eq!(pos_2.y, 50.0);

            // Row 1
            let pos_3 = default_element_position(3);
            assert_eq!(pos_3.x, 50.0);
            assert_eq!(pos_3.y, 200.0);

            let pos_4 = default_element_position(4);
            assert_eq!(pos_4.x, 250.0);
            assert_eq!(pos_4.y, 200.0);
        }
    }

    mod element_colors_tests {
        use super::*;

        /// Verifies element_colors returns correct colors for internal person
        #[test]
        fn element_colors_internal_person() {
            let element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(0.0, 0.0),
            );

            let (bg, border) = element_colors(&element, false);
            // Internal person should have peachy color
            assert_eq!(bg, Color32::from_rgb(255, 220, 180));
            assert_eq!(border, Color32::from_gray(150));
        }

        /// Verifies element_colors returns correct colors for external person
        #[test]
        fn element_colors_external_person() {
            let element = Element::new(
                ElementType::external_person("External", "Description"),
                Position::new(0.0, 0.0),
            );

            let (bg, border) = element_colors(&element, false);
            // External person should have lighter peach color
            assert_eq!(bg, Color32::from_rgb(255, 240, 220));
        }

        /// Verifies element_colors returns correct colors for internal system
        #[test]
        fn element_colors_internal_system() {
            let element = Element::new(
                ElementType::system("System", "Description"),
                Position::new(0.0, 0.0),
            );

            let (bg, border) = element_colors(&element, false);
            // Internal system should have light blue
            assert_eq!(bg, Color32::from_rgb(200, 220, 255));
        }

        /// Verifies element_colors returns correct colors for external system
        #[test]
        fn element_colors_external_system() {
            let element = Element::new(
                ElementType::external_system("External", "Description"),
                Position::new(0.0, 0.0),
            );

            let (bg, border) = element_colors(&element, false);
            // External system should have gray
            assert_eq!(bg, Color32::from_rgb(230, 230, 230));
        }

        /// Verifies element_colors returns correct colors for database container
        #[test]
        fn element_colors_database_container() {
            let element = Element::new(
                ElementType::container("DB", "Database", ContainerType::Database, "PostgreSQL"),
                Position::new(0.0, 0.0),
            );

            let (bg, _) = element_colors(&element, false);
            assert_eq!(bg, Color32::from_rgb(200, 255, 200)); // Light green
        }

        /// Verifies element_colors returns correct colors for queue container
        #[test]
        fn element_colors_queue_container() {
            let element = Element::new(
                ElementType::container("Queue", "Message Queue", ContainerType::Queue, "RabbitMQ"),
                Position::new(0.0, 0.0),
            );

            let (bg, _) = element_colors(&element, false);
            assert_eq!(bg, Color32::from_rgb(255, 255, 200)); // Light yellow
        }

        /// Verifies element_colors returns correct colors for web container
        #[test]
        fn element_colors_web_container() {
            let element = Element::new(
                ElementType::container("Web", "Web App", ContainerType::WebApplication, "React"),
                Position::new(0.0, 0.0),
            );

            let (bg, _) = element_colors(&element, false);
            assert_eq!(bg, Color32::from_rgb(220, 240, 255)); // Light blue-gray
        }

        /// Verifies element_colors returns selected border color when selected
        #[test]
        fn element_colors_selected() {
            let element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(0.0, 0.0),
            );

            let (_, border) = element_colors(&element, true);
            assert_eq!(border, Color32::from_rgb(0, 120, 215)); // Blue selection
        }
    }

    mod get_element_icon_tests {
        use super::*;

        /// Verifies get_element_icon returns correct icon for person
        #[test]
        fn get_element_icon_person() {
            let element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "👤");
        }

        /// Verifies get_element_icon returns correct icon for external person
        #[test]
        fn get_element_icon_external_person() {
            let element = Element::new(
                ElementType::external_person("External", "Description"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "👤");
        }

        /// Verifies get_element_icon returns correct icon for system
        #[test]
        fn get_element_icon_system() {
            let element = Element::new(
                ElementType::system("System", "Description"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "🖥️");
        }

        /// Verifies get_element_icon returns correct icon for external system
        #[test]
        fn get_element_icon_external_system() {
            let element = Element::new(
                ElementType::external_system("External", "Description"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "🖥️");
        }

        /// Verifies get_element_icon returns correct icon for database container
        #[test]
        fn get_element_icon_database() {
            let element = Element::new(
                ElementType::container("DB", "Database", ContainerType::Database, "PostgreSQL"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "🗄️");
        }

        /// Verifies get_element_icon returns correct icon for mobile app container
        #[test]
        fn get_element_icon_mobile_app() {
            let element = Element::new(
                ElementType::container("App", "Mobile App", ContainerType::MobileApp, "iOS"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "📱");
        }

        /// Verifies get_element_icon returns correct icon for queue container
        #[test]
        fn get_element_icon_queue() {
            let element = Element::new(
                ElementType::container("Queue", "Message Queue", ContainerType::Queue, "RabbitMQ"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "📨");
        }

        /// Verifies get_element_icon returns correct icon for web application container
        #[test]
        fn get_element_icon_web_application() {
            let element = Element::new(
                ElementType::container("Web", "Web App", ContainerType::WebApplication, "React"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "📦");
        }

        /// Verifies get_element_icon returns correct icon for microservice container
        #[test]
        fn get_element_icon_microservice() {
            let element = Element::new(
                ElementType::container("Service", "Microservice", ContainerType::Microservice, "Rust"),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "📦");
        }

        /// Verifies get_element_icon returns correct icon for other container type
        #[test]
        fn get_element_icon_other() {
            let element = Element::new(
                ElementType::container("Custom", "Custom Type", ContainerType::Other("Custom".to_string()), ""),
                Position::new(0.0, 0.0),
            );
            assert_eq!(get_element_icon(&element), "📦");
        }
    }
}
