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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ContainerType, Diagram, DiagramType, Element, ElementId, ElementType, Position, Relationship};

    mod escape_string_tests {
        use super::*;

        /// Verifies escape_string escapes double quotes
        #[test]
        fn escape_string_escapes_quotes() {
            let exporter = PlantUmlExporter::new();
            let input = r#"This has "quotes" in it"#;
            let result = exporter.escape_string(input);
            assert_eq!(result, r#"This has \"quotes\" in it"#);
        }

        /// Verifies escape_string replaces newlines with spaces
        #[test]
        fn escape_string_replaces_newlines() {
            let exporter = PlantUmlExporter::new();
            let input = "Line1\nLine2\nLine3";
            let result = exporter.escape_string(input);
            assert_eq!(result, "Line1 Line2 Line3");
        }

        /// Verifies escape_string handles combined special characters
        #[test]
        fn escape_string_handles_combined_special_chars() {
            let exporter = PlantUmlExporter::new();
            let input = "Description with \"quotes\" and\nnewlines";
            let result = exporter.escape_string(input);
            assert_eq!(result, "Description with \\\"quotes\\\" and newlines");
        }

        /// Verifies escape_string leaves normal text unchanged
        #[test]
        fn escape_string_leaves_normal_text() {
            let exporter = PlantUmlExporter::new();
            let input = "Normal text without special characters";
            let result = exporter.escape_string(input);
            assert_eq!(result, "Normal text without special characters");
        }
    }

    mod generate_element_tests {
        use super::*;

        /// Verifies generate_element creates correct output for internal person
        #[test]
        fn generate_element_internal_person() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let id = format!("elem_{}", element.id.simple());

            let result = exporter.generate_element(&element);
            assert!(result.contains("Person"));
            assert!(result.contains(&id));
            assert!(result.contains("User"));
            assert!(result.contains("A user"));
            assert!(!result.contains("Person_Ext"));
        }

        /// Verifies generate_element creates correct output for external person
        #[test]
        fn generate_element_external_person() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::external_person("External User", "External"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("Person_Ext"));
            assert!(result.contains("External User"));
        }

        /// Verifies generate_element creates correct output for internal system
        #[test]
        fn generate_element_internal_system() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::system("MySystem", "A system"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("System("));
            assert!(!result.contains("System_Ext"));
            assert!(result.contains("MySystem"));
        }

        /// Verifies generate_element creates correct output for external system
        #[test]
        fn generate_element_external_system() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::external_system("External System", "External"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("System_Ext"));
        }

        /// Verifies generate_element creates correct output for container
        #[test]
        fn generate_element_container() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::container("WebApp", "A web app", ContainerType::WebApplication, "React"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("Container("));
            assert!(result.contains("WebApp"));
            assert!(result.contains("A web app"));
            assert!(result.contains("React"));
        }

        /// Verifies generate_element creates ContainerDb for database containers
        #[test]
        fn generate_element_database_container() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::container("Database", "Stores data", ContainerType::Database, "PostgreSQL"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("ContainerDb"));
        }

        /// Verifies generate_element creates ContainerQueue for queue containers
        #[test]
        fn generate_element_queue_container() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::container("Queue", "Message queue", ContainerType::Queue, "RabbitMQ"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("ContainerQueue"));
        }

        /// Verifies generate_element handles empty technology
        #[test]
        fn generate_element_empty_technology() {
            let exporter = PlantUmlExporter::new();
            let element = Element::new(
                ElementType::container("App", "An app", ContainerType::Microservice, ""),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            // Should not have technology parameter when empty
            assert!(result.contains("Container("));
            assert!(!result.contains("\"\""));
        }
    }

    mod generate_relationship_tests {
        use super::*;

        /// Verifies generate_relationship creates correct output without technology
        #[test]
        fn generate_relationship_without_technology() {
            let exporter = PlantUmlExporter::new();
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let rel = Relationship::new(source_id, target_id, "uses");

            let result = exporter.generate_relationship(&rel);
            assert!(result.contains("Rel("));
            assert!(result.contains("uses"));
            assert!(!result.contains("\", \""));
        }

        /// Verifies generate_relationship creates correct output with technology
        #[test]
        fn generate_relationship_with_technology() {
            let exporter = PlantUmlExporter::new();
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let rel = Relationship::with_technology(source_id, target_id, "uses", "HTTPS");

            let result = exporter.generate_relationship(&rel);
            assert!(result.contains("Rel("));
            assert!(result.contains("uses"));
            assert!(result.contains("HTTPS"));
        }
    }

    mod export_tests {
        use super::*;

        /// Verifies export produces valid PlantUML output
        #[test]
        fn export_produces_valid_plantuml() {
            let exporter = PlantUmlExporter::new();
            let mut diagram = Diagram::new("Test Diagram", "Test Description", DiagramType::SystemContext);
            
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            diagram.add_element(element);

            let result = exporter.export(&diagram);
            
            // Check for PlantUML markers
            assert!(result.starts_with("@startuml"));
            assert!(result.ends_with("@enduml\n"));
            assert!(result.contains("!include"));
            assert!(result.contains("C4_Context.puml"));
            assert!(result.contains("title Test Diagram"));
            assert!(result.contains("' Test Description"));
            assert!(result.contains("Person"));
        }

        /// Verifies export uses correct include for Container diagrams
        #[test]
        fn export_uses_correct_include_for_container() {
            let exporter = PlantUmlExporter::new();
            let diagram = Diagram::new("Test", "", DiagramType::Container);

            let result = exporter.export(&diagram);
            assert!(result.contains("C4_Container.puml"));
            assert!(!result.contains("C4_Context.puml"));
        }

        /// Verifies export handles empty diagrams
        #[test]
        fn export_handles_empty_diagram() {
            let exporter = PlantUmlExporter::new();
            let diagram = Diagram::new("Empty", "", DiagramType::SystemContext);

            let result = exporter.export(&diagram);
            assert!(result.starts_with("@startuml"));
            assert!(result.ends_with("@enduml\n"));
        }

        /// Verifies export includes relationships
        #[test]
        fn export_includes_relationships() {
            let exporter = PlantUmlExporter::new();
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            
            let source = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let target = Element::new(
                ElementType::system("System", "A system"),
                Position::new(100.0, 0.0),
            );
            let source_id = source.id;
            let target_id = target.id;
            
            diagram.add_element(source);
            diagram.add_element(target);
            diagram.add_relationship(Relationship::new(source_id, target_id, "uses"));

            let result = exporter.export(&diagram);
            assert!(result.contains("Rel("));
            assert!(result.contains("uses"));
        }
    }

}
