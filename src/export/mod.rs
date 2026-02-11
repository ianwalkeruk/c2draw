pub mod mermaid;
pub mod plantuml;

pub use mermaid::MermaidExporter;
pub use plantuml::PlantUmlExporter;

use crate::model::Diagram;

/// Trait for diagram exporters
pub trait DiagramExporter {
    /// Export a diagram to string format
    fn export(&self, diagram: &Diagram) -> String;

    /// Get the file extension for this format
    fn file_extension(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Diagram, DiagramType, Element, ElementType, Position};

    /// Test helper struct implementing DiagramExporter
    struct TestExporter;

    impl TestExporter {
        fn new() -> Self {
            Self
        }
    }

    impl DiagramExporter for TestExporter {
        fn export(&self, diagram: &Diagram) -> String {
            format!("Test export of: {}", diagram.name)
        }

        fn file_extension(&self) -> &'static str {
            "test"
        }
    }

    mod trait_contract_tests {
        use super::*;

        /// Verifies DiagramExporter trait can be implemented and export method works
        #[test]
        fn diagram_exporter_export_method() {
            let exporter = TestExporter::new();
            let diagram = Diagram::new("My Diagram", "", DiagramType::SystemContext);

            let result = exporter.export(&diagram);
            assert_eq!(result, "Test export of: My Diagram");
        }

        /// Verifies DiagramExporter trait file_extension method works
        #[test]
        fn diagram_exporter_file_extension_method() {
            let exporter = TestExporter::new();
            assert_eq!(exporter.file_extension(), "test");
        }

        /// Verifies real exporters implement the trait correctly
        #[test]
        fn plantuml_exporter_implements_trait() {
            let exporter = PlantUmlExporter::new();
            let diagram = Diagram::new("Test", "", DiagramType::SystemContext);

            // Should be able to call trait methods
            let output = exporter.export(&diagram);
            assert!(!output.is_empty());
            assert_eq!(exporter.file_extension(), "puml");
        }

        /// Verifies MermaidExporter implements the trait correctly
        #[test]
        fn mermaid_exporter_implements_trait() {
            let exporter = MermaidExporter::new();
            let diagram = Diagram::new("Test", "", DiagramType::SystemContext);

            // Should be able to call trait methods
            let output = exporter.export(&diagram);
            assert!(!output.is_empty());
            assert_eq!(exporter.file_extension(), "mmd");
        }

        /// Verifies export produces non-empty output for diagrams with elements
        #[test]
        fn export_produces_output_with_elements() {
            let exporter = TestExporter::new();
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(0.0, 0.0),
            );
            diagram.add_element(element);

            let result = exporter.export(&diagram);
            assert!(!result.is_empty());
        }

    }
}
