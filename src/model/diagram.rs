use super::{Element, ElementId, Relationship, FILE_FORMAT_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The complete diagram containing all elements and relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagram {
    #[serde(default = "default_version")]
    pub version: String,
    pub name: String,
    pub description: String,
    pub diagram_type: DiagramType,
    pub elements: HashMap<ElementId, Element>,
    pub relationships: Vec<Relationship>,
}

fn default_version() -> String {
    FILE_FORMAT_VERSION.to_string()
}

impl Default for Diagram {
    fn default() -> Self {
        Self::new("Untitled Diagram", "", DiagramType::SystemContext)
    }
}

impl Diagram {
    pub fn new(name: impl Into<String>, description: impl Into<String>, diagram_type: DiagramType) -> Self {
        Self {
            version: FILE_FORMAT_VERSION.to_string(),
            name: name.into(),
            description: description.into(),
            diagram_type,
            elements: HashMap::new(),
            relationships: Vec::new(),
        }
    }

    /// Add an element to the diagram
    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.id, element);
    }

    /// Remove an element and all its relationships
    pub fn remove_element(&mut self, id: ElementId) {
        self.elements.remove(&id);
        self.relationships
            .retain(|r| r.source_id != id && r.target_id != id);
    }

    /// Get an element by ID
    pub fn get_element(&self, id: ElementId) -> Option<&Element> {
        self.elements.get(&id)
    }

    /// Get a mutable reference to an element
    pub fn get_element_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.get_mut(&id)
    }

    /// Add a relationship between two elements
    pub fn add_relationship(&mut self, relationship: Relationship) {
        // Only add if both elements exist
        if self.elements.contains_key(&relationship.source_id)
            && self.elements.contains_key(&relationship.target_id)
        {
            self.relationships.push(relationship);
        }
    }

    /// Remove a relationship by ID
    pub fn remove_relationship(&mut self, id: uuid::Uuid) {
        self.relationships.retain(|r| r.id != id);
    }

    /// Get all relationships from a specific element
    pub fn relationships_from(&self, element_id: ElementId) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_id == element_id)
            .collect()
    }

    /// Get all relationships to a specific element
    pub fn relationships_to(&self, element_id: ElementId) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.target_id == element_id)
            .collect()
    }

    /// Get all relationships connected to an element (both from and to)
    pub fn relationships_connected_to(&self, element_id: ElementId) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_id == element_id || r.target_id == element_id)
            .collect()
    }

    /// Save the diagram to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load a diagram from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Type of C4 diagram
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagramType {
    /// C1: System Context diagram
    #[serde(rename = "SystemContext")]
    SystemContext,
    /// C2: Container diagram
    #[serde(rename = "Container")]
    Container,
}

impl DiagramType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagramType::SystemContext => "System Context",
            DiagramType::Container => "Container",
        }
    }

    pub fn supports_containers(&self) -> bool {
        matches!(self, DiagramType::Container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Element, ElementType, Position, Relationship};

    mod diagram_creation_tests {
        use super::*;

        /// Verifies Diagram::new creates an empty diagram with correct properties
        #[test]
        fn diagram_new_creates_empty_diagram() {
            let diagram = Diagram::new("Test Diagram", "A test description", DiagramType::SystemContext);

            assert_eq!(diagram.name, "Test Diagram");
            assert_eq!(diagram.description, "A test description");
            assert_eq!(diagram.diagram_type, DiagramType::SystemContext);
            assert_eq!(diagram.version, FILE_FORMAT_VERSION);
            assert!(diagram.elements.is_empty());
            assert!(diagram.relationships.is_empty());
        }

    }

    mod element_management_tests {
        use super::*;

        /// Verifies add_element adds elements to the diagram
        #[test]
        fn add_element_adds_to_diagram() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let id = element.id;

            diagram.add_element(element);

            assert_eq!(diagram.elements.len(), 1);
            assert!(diagram.elements.contains_key(&id));
        }

        /// Verifies get_element returns the correct element
        #[test]
        fn get_element_returns_element() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let element = Element::new(
                ElementType::system("System", "A system"),
                Position::new(10.0, 20.0),
            );
            let id = element.id;

            diagram.add_element(element);

            let retrieved = diagram.get_element(id);
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().name(), "System");
        }

        /// Verifies get_element returns None for non-existent element
        #[test]
        fn get_element_returns_none_for_invalid_id() {
            let diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let fake_id = ElementId::new_v4();

            let retrieved = diagram.get_element(fake_id);
            assert!(retrieved.is_none());
        }

        /// Verifies get_element_mut allows modifying the element
        #[test]
        fn get_element_mut_allows_modification() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let element = Element::new(
                ElementType::system("System", "A system"),
                Position::new(0.0, 0.0),
            );
            let id = element.id;

            diagram.add_element(element);

            if let Some(elem) = diagram.get_element_mut(id) {
                elem.set_name("Modified System".to_string());
            }

            assert_eq!(diagram.get_element(id).unwrap().name(), "Modified System");
        }

        /// Verifies remove_element removes the element
        #[test]
        fn remove_element_removes_from_diagram() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let id = element.id;

            diagram.add_element(element);
            diagram.remove_element(id);

            assert!(diagram.elements.is_empty());
        }
    }

    mod relationship_tests {
        use super::*;

        fn create_test_diagram_with_elements() -> (Diagram, ElementId, ElementId) {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let source = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let target = Element::new(
                ElementType::system("System", "A system"),
                Position::new(100.0, 100.0),
            );
            let source_id = source.id;
            let target_id = target.id;

            diagram.add_element(source);
            diagram.add_element(target);

            (diagram, source_id, target_id)
        }

        /// Verifies add_relationship adds a relationship between elements
        #[test]
        fn add_relationship_adds_connection() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel = Relationship::new(source_id, target_id, "uses");
            diagram.add_relationship(rel);

            assert_eq!(diagram.relationships.len(), 1);
        }

        /// Verifies add_relationship does not add if source element doesn't exist
        #[test]
        fn add_relationship_requires_existing_source() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let target = Element::new(
                ElementType::system("System", "A system"),
                Position::new(100.0, 100.0),
            );
            let target_id = target.id;
            diagram.add_element(target);

            let fake_source_id = ElementId::new_v4();
            let rel = Relationship::new(fake_source_id, target_id, "uses");
            diagram.add_relationship(rel);

            assert!(diagram.relationships.is_empty());
        }

        /// Verifies add_relationship does not add if target element doesn't exist
        #[test]
        fn add_relationship_requires_existing_target() {
            let mut diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let source = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let source_id = source.id;
            diagram.add_element(source);

            let fake_target_id = ElementId::new_v4();
            let rel = Relationship::new(source_id, fake_target_id, "uses");
            diagram.add_relationship(rel);

            assert!(diagram.relationships.is_empty());
        }

        /// Verifies remove_relationship removes by id
        #[test]
        fn remove_relationship_removes_by_id() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel = Relationship::new(source_id, target_id, "uses");
            let rel_id = rel.id;
            diagram.add_relationship(rel);

            diagram.remove_relationship(rel_id);

            assert!(diagram.relationships.is_empty());
        }

        /// Verifies remove_element also removes associated relationships
        #[test]
        fn remove_element_removes_associated_relationships() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel = Relationship::new(source_id, target_id, "uses");
            diagram.add_relationship(rel);

            diagram.remove_element(source_id);

            assert!(diagram.relationships.is_empty());
        }

        /// Verifies relationships_from returns only relationships from the specified element
        #[test]
        fn relationships_from_filters_correctly() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel1 = Relationship::new(source_id, target_id, "uses");
            diagram.add_relationship(rel1);

            let from_source = diagram.relationships_from(source_id);
            assert_eq!(from_source.len(), 1);
            assert_eq!(from_source[0].description, "uses");

            let from_target = diagram.relationships_from(target_id);
            assert!(from_target.is_empty());
        }

        /// Verifies relationships_to returns only relationships to the specified element
        #[test]
        fn relationships_to_filters_correctly() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel1 = Relationship::new(source_id, target_id, "uses");
            diagram.add_relationship(rel1);

            let to_target = diagram.relationships_to(target_id);
            assert_eq!(to_target.len(), 1);

            let to_source = diagram.relationships_to(source_id);
            assert!(to_source.is_empty());
        }

        /// Verifies relationships_connected_to returns all connected relationships
        #[test]
        fn relationships_connected_to_returns_all() {
            let (mut diagram, source_id, target_id) = create_test_diagram_with_elements();

            let rel1 = Relationship::new(source_id, target_id, "uses");
            diagram.add_relationship(rel1);

            let connected_to_source = diagram.relationships_connected_to(source_id);
            assert_eq!(connected_to_source.len(), 1);

            let connected_to_target = diagram.relationships_connected_to(target_id);
            assert_eq!(connected_to_target.len(), 1);
        }
    }

    mod serialization_tests {
        use super::*;

        /// Verifies to_json produces valid JSON and from_json can parse it back
        #[test]
        fn json_roundtrip_preserves_data() {
            let mut diagram = Diagram::new("Test Diagram", "Test Description", DiagramType::Container);
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(10.0, 20.0),
            );
            diagram.add_element(element);

            let json = diagram.to_json().expect("Failed to serialize");
            let restored = Diagram::from_json(&json).expect("Failed to deserialize");

            assert_eq!(restored.name, diagram.name);
            assert_eq!(restored.description, diagram.description);
            assert_eq!(restored.diagram_type, diagram.diagram_type);
            assert_eq!(restored.elements.len(), diagram.elements.len());
        }

        /// Verifies JSON serialization includes version field
        #[test]
        fn json_includes_version() {
            let diagram = Diagram::new("Test", "", DiagramType::SystemContext);
            let json = diagram.to_json().expect("Failed to serialize");
            
            assert!(json.contains("version"));
            assert!(json.contains(FILE_FORMAT_VERSION));
        }
    }

    mod diagram_type_tests {
        use super::*;

        /// Verifies DiagramType::as_str returns correct display strings
        #[test]
        fn diagram_type_as_str() {
            assert_eq!(DiagramType::SystemContext.as_str(), "System Context");
            assert_eq!(DiagramType::Container.as_str(), "Container");
        }

        /// Verifies supports_containers returns correct values
        #[test]
        fn diagram_type_supports_containers() {
            assert!(!DiagramType::SystemContext.supports_containers());
            assert!(DiagramType::Container.supports_containers());
        }
    }
}
