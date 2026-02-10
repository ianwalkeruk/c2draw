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
