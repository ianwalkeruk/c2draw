use crate::model::{ContainerType, Diagram, DiagramType, ElementType};
use super::DiagramExporter;

/// Exports diagrams to C4-PlantUML format
pub struct PlantUmlExporter;

impl PlantUmlExporter {
    pub fn new() -> Self {
        Self
    }

    fn get_include(&self, diagram_type: DiagramType) -> &'static str {
        match diagram_type {
            DiagramType::SystemContext => "C4_Context.puml",
            DiagramType::Container => "C4_Container.puml",
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
                        "Person_Ext({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                } else {
                    format!(
                        "Person({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                }
            }
            ElementType::SoftwareSystem(data) => {
                if data.is_external {
                    format!(
                        "System_Ext({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                } else {
                    format!(
                        "System({}, \"{}\", \"{}\")",
                        id, name, description
                    )
                }
            }
            ElementType::Container(data) => {
                let container_type = match &data.container_type {
                    ContainerType::Database => "ContainerDb",
                    ContainerType::Queue => "ContainerQueue",
                    _ => "Container",
                };
                let technology = self.escape_string(&data.technology);
                if technology.is_empty() {
                    format!(
                        "{}({}, \"{}\", \"{}\")",
                        container_type, id, name, description
                    )
                } else {
                    format!(
                        "{}({}, \"{}\", \"{}\", \"{}\")",
                        container_type, id, name, description, technology
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
                "Rel({}, {}, \"{}\", \"{}\")",
                source_id, target_id, description, technology
            )
        } else {
            format!(
                "Rel({}, {}, \"{}\")",
                source_id, target_id, description
            )
        }
    }
}

impl Default for PlantUmlExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagramExporter for PlantUmlExporter {
    fn export(&self, diagram: &Diagram) -> String {
        let include = self.get_include(diagram.diagram_type);
        let mut output = String::new();

        // Header
        output.push_str("@startuml\n");
        output.push_str(&format!(
            "!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/{}\n\n",
            include
        ));

        // Title
        output.push_str(&format!("title {}\n\n", self.escape_string(&diagram.name)));

        // Description (as comment)
        if !diagram.description.is_empty() {
            output.push_str(&format!(
                "' {}\n\n",
                self.escape_string(&diagram.description)
            ));
        }

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

        // Footer
        output.push_str("\n@enduml\n");

        output
    }

    fn file_extension(&self) -> &'static str {
        "puml"
    }
}
