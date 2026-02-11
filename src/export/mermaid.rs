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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ContainerType, Diagram, DiagramType, Element, ElementId, ElementType, Position, Relationship};

    mod escape_string_tests {
        use super::*;

        /// Verifies escape_string escapes double quotes
        #[test]
        fn escape_string_escapes_quotes() {
            let exporter = MermaidExporter::new();
            let input = r#"This has "quotes" in it"#;
            let result = exporter.escape_string(input);
            assert_eq!(result, r#"This has \"quotes\" in it"#);
        }

        /// Verifies escape_string replaces newlines with spaces
        #[test]
        fn escape_string_replaces_newlines() {
            let exporter = MermaidExporter::new();
            let input = "Line1\nLine2\nLine3";
            let result = exporter.escape_string(input);
            assert_eq!(result, "Line1 Line2 Line3");
        }

        /// Verifies escape_string handles combined special characters
        #[test]
        fn escape_string_handles_combined_special_chars() {
            let exporter = MermaidExporter::new();
            let input = "Description with \"quotes\" and\nnewlines";
            let result = exporter.escape_string(input);
            assert_eq!(result, "Description with \\\"quotes\\\" and newlines");
        }

        /// Verifies escape_string leaves normal text unchanged
        #[test]
        fn escape_string_leaves_normal_text() {
            let exporter = MermaidExporter::new();
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
            let exporter = MermaidExporter::new();
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let id = format!("elem_{}", element.id.simple());

            let result = exporter.generate_element(&element);
            assert!(result.contains("Person("));
            assert!(result.contains(&id));
            assert!(result.contains("User"));
            assert!(result.contains("A user"));
            assert!(!result.contains("Person_Ext"));
        }

        /// Verifies generate_element creates correct output for external person
        #[test]
        fn generate_element_external_person() {
            let exporter = MermaidExporter::new();
            let element = Element::new(
                ElementType::external_person("External User", "External"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("Person_Ext("));
            assert!(result.contains("External User"));
        }

        /// Verifies generate_element creates correct output for internal system
        #[test]
        fn generate_element_internal_system() {
            let exporter = MermaidExporter::new();
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
            let exporter = MermaidExporter::new();
            let element = Element::new(
                ElementType::external_system("External System", "External"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.contains("System_Ext("));
        }

        /// Verifies generate_element creates correct output for container
        #[test]
        fn generate_element_container() {
            let exporter = MermaidExporter::new();
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

        /// Verifies generate_element handles empty technology
        #[test]
        fn generate_element_empty_technology() {
            let exporter = MermaidExporter::new();
            let element = Element::new(
                ElementType::container("App", "An app", ContainerType::Microservice, ""),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            // Should not have technology parameter when empty
            assert!(result.contains("Container("));
            // Should have exactly 3 parameters (4 values including id)
            let comma_count = result.matches(',').count();
            assert_eq!(comma_count, 2);
        }

        /// Verifies generate_element uses proper indentation
        #[test]
        fn generate_element_uses_proper_indentation() {
            let exporter = MermaidExporter::new();
            let element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(0.0, 0.0),
            );

            let result = exporter.generate_element(&element);
            assert!(result.starts_with("    ")); // 4 spaces indent
        }
    }

    mod generate_relationship_tests {
        use super::*;

        /// Verifies generate_relationship creates correct output without technology
        #[test]
        fn generate_relationship_without_technology() {
            let exporter = MermaidExporter::new();
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let rel = Relationship::new(source_id, target_id, "uses");

            let result = exporter.generate_relationship(&rel);
            assert!(result.contains("BiRel("));
            assert!(result.contains("uses"));
            assert!(!result.contains("\", \""));
        }

        /// Verifies generate_relationship creates correct output with technology
        #[test]
        fn generate_relationship_with_technology() {
            let exporter = MermaidExporter::new();
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let rel = Relationship::with_technology(source_id, target_id, "uses", "HTTPS");

            let result = exporter.generate_relationship(&rel);
            assert!(result.contains("BiRel("));
            assert!(result.contains("uses"));
            assert!(result.contains("HTTPS"));
        }

        /// Verifies generate_relationship uses proper indentation
        #[test]
        fn generate_relationship_uses_proper_indentation() {
            let exporter = MermaidExporter::new();
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let rel = Relationship::new(source_id, target_id, "uses");

            let result = exporter.generate_relationship(&rel);
            assert!(result.starts_with("    ")); // 4 spaces indent
        }
    }

    mod export_tests {
        use super::*;

        /// Verifies export produces valid Mermaid output
        #[test]
        fn export_produces_valid_mermaid() {
            let exporter = MermaidExporter::new();
            let mut diagram = Diagram::new("Test Diagram", "Test Description", DiagramType::SystemContext);
            
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            diagram.add_element(element);

            let result = exporter.export(&diagram);
            
            // Check for Mermaid markers
            assert!(result.starts_with("C4Context"));
            assert!(result.contains("title Test Diagram"));
            assert!(result.contains("%% Test Description"));
            assert!(result.contains("Person("));
        }

        /// Verifies export uses correct diagram keyword for Container diagrams
        #[test]
        fn export_uses_correct_keyword_for_container() {
            let exporter = MermaidExporter::new();
            let diagram = Diagram::new("Test", "", DiagramType::Container);

            let result = exporter.export(&diagram);
            assert!(result.starts_with("C4Container"));
            assert!(!result.contains("C4Context"));
        }

        /// Verifies export handles empty diagrams
        #[test]
        fn export_handles_empty_diagram() {
            let exporter = MermaidExporter::new();
            let diagram = Diagram::new("Empty", "", DiagramType::SystemContext);

            let result = exporter.export(&diagram);
            assert!(result.starts_with("C4Context"));
        }

        /// Verifies export includes relationships
        #[test]
        fn export_includes_relationships() {
            let exporter = MermaidExporter::new();
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
            assert!(result.contains("BiRel("));
            assert!(result.contains("uses"));
        }

        /// Verifies export omits title when empty
        #[test]
        fn export_omits_empty_title() {
            let exporter = MermaidExporter::new();
            let mut diagram = Diagram::new("", "Description", DiagramType::SystemContext);
            
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            diagram.add_element(element);

            let result = exporter.export(&diagram);
            assert!(!result.contains("title"));
        }

        /// Verifies export includes description as comment
        #[test]
        fn export_includes_description_as_comment() {
            let exporter = MermaidExporter::new();
            let diagram = Diagram::new("Test", "A description", DiagramType::SystemContext);

            let result = exporter.export(&diagram);
            assert!(result.contains("%% A description"));
        }
    }

}
