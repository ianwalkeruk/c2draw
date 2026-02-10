use crate::model::{Diagram, DiagramType, ElementType};
use super::DiagramExporter;

/// Exports diagrams to Mermaid C4 format
pub struct MermaidExporter;

impl MermaidExporter {
    pub fn new() -> Self {
        Self
    }

    fn get_diagram_keyword(&self, diagram_type: DiagramType) -> &'static str {
        match diagram_type {
            DiagramType::SystemContext => "C4Context",
            DiagramType::Container => "C4Container",
        }
    }

    fn escape_string(&self, s: &str) -> String {
        s.replace('"', "\\\"").replace('\n', " ")
    }

    fn generate_element(&self, element: &crate::model::Element) -> String {
        let name = self.escape_string(element.name());
        let description = self.escape_string(element.description());
        let id = format!("elem_{}", element.id.simple());

        match &element.element_type {
            ElementType::Person(data) => {
                if data.is_external {
                    format!(
                        "    Person_Ext({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                } else {
                    format!(
                        "    Person({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                }
            }
            ElementType::SoftwareSystem(data) => {
                if data.is_external {
                    format!(
                        "    System_Ext({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                } else {
                    format!(
                        "    System({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                }
            }
            ElementType::Container(data) => {
                let technology = self.escape_string(&data.technology);
                if technology.is_empty() {
                    format!(
                        "    Container({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                } else {
                    format!(
                        "    Container({}, \"{}\", \"{}\", \"{}\")",
                        id, name, description, technology
                    )
                }
            }
        }
    }

    fn generate_relationship(&self, rel: &crate::model::Relationship) -> String {
        let source_id = format!("elem_{}", rel.source_id.simple());
        let target_id = format!("elem_{}", rel.target_id.simple());
        let description = self.escape_string(&rel.description);

        if let Some(tech) = &rel.technology {
            let technology = self.escape_string(tech);
            format!(
                "    BiRel({}, {}, \"{}\", \"{}\")",
                source_id, target_id, description, technology
            )
        } else {
            format!(
                "    BiRel({}, {}, \"{}\")",
                source_id, target_id, description
            )
        }
    }
}

impl Default for MermaidExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagramExporter for MermaidExporter {
    fn export(&self, diagram: &Diagram) -> String {
        let diagram_keyword = self.get_diagram_keyword(diagram.diagram_type);
        let mut output = String::new();

        // Header
        output.push_str(&format!("{}\n", diagram_keyword));

        // Title/Note
        if !diagram.name.is_empty() {
            output.push_str(&format!(
                "    title {}\n",
                self.escape_string(&diagram.name)
            ));
        }

        // Description
        if !diagram.description.is_empty() {
            output.push_str(&format!(
                "    %% {}\n",
                self.escape_string(&diagram.description)
            ));
        }

        output.push('\n');

        // Elements
        for element in diagram.elements.values() {
            output.push_str(&self.generate_element(element));
            output.push('\n');
        }

        output.push('\n');

        // Relationships
        for rel in &diagram.relationships {
            output.push_str(&self.generate_relationship(rel));
            output.push('\n');
        }

        output
    }

    fn file_extension(&self) -> &'static str {
        "mmd"
    }
}
