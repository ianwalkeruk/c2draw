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
