use super::{ElementId, Position, Positioned, Size};
use serde::{Deserialize, Serialize};

/// A visual element on the diagram canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: ElementId,
    pub element_type: ElementType,
    pub position: Position,
    pub size: Size,
}

impl Element {
    pub fn new(element_type: ElementType, position: Position) -> Self {
        let size = element_type.default_size();
        Self {
            id: ElementId::new_v4(),
            element_type,
            position,
            size,
        }
    }

    pub fn name(&self) -> &str {
        match &self.element_type {
            ElementType::Person(data) => &data.name,
            ElementType::SoftwareSystem(data) => &data.name,
            ElementType::Container(data) => &data.name,
        }
    }

    pub fn description(&self) -> &str {
        match &self.element_type {
            ElementType::Person(data) => &data.description,
            ElementType::SoftwareSystem(data) => &data.description,
            ElementType::Container(data) => &data.description,
        }
    }

    pub fn is_external(&self) -> bool {
        match &self.element_type {
            ElementType::Person(data) => data.is_external,
            ElementType::SoftwareSystem(data) => data.is_external,
            ElementType::Container(_) => false,
        }
    }

    pub fn set_name(&mut self, name: String) {
        match &mut self.element_type {
            ElementType::Person(data) => data.name = name,
            ElementType::SoftwareSystem(data) => data.name = name,
            ElementType::Container(data) => data.name = name,
        }
    }

    pub fn set_description(&mut self, description: String) {
        match &mut self.element_type {
            ElementType::Person(data) => data.description = description,
            ElementType::SoftwareSystem(data) => data.description = description,
            ElementType::Container(data) => data.description = description,
        }
    }
}

impl Positioned for Element {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn size(&self) -> Size {
        self.size
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}

/// Types of elements in C4 diagrams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Person(PersonData),
    SoftwareSystem(SystemData),
    Container(ContainerData),
}

impl ElementType {
    /// Get the display name for this element type
    pub fn type_name(&self) -> &'static str {
        match self {
            ElementType::Person(_) => "Person",
            ElementType::SoftwareSystem(_) => "Software System",
            ElementType::Container(_) => "Container",
        }
    }

    /// Get the default size for this element type
    pub fn default_size(&self) -> Size {
        match self {
            ElementType::Person(_) => Size::new(120.0, 80.0),
            ElementType::SoftwareSystem(_) => Size::new(160.0, 100.0),
            ElementType::Container(_) => Size::new(160.0, 100.0),
        }
    }

    /// Create a new person element
    pub fn person(name: impl Into<String>, description: impl Into<String>) -> Self {
        ElementType::Person(PersonData {
            name: name.into(),
            description: description.into(),
            is_external: false,
        })
    }

    /// Create a new external person element
    pub fn external_person(name: impl Into<String>, description: impl Into<String>) -> Self {
        ElementType::Person(PersonData {
            name: name.into(),
            description: description.into(),
            is_external: true,
        })
    }

    /// Create a new software system element
    pub fn system(name: impl Into<String>, description: impl Into<String>) -> Self {
        ElementType::SoftwareSystem(SystemData {
            name: name.into(),
            description: description.into(),
            is_external: false,
        })
    }

    /// Create a new external software system element
    pub fn external_system(name: impl Into<String>, description: impl Into<String>) -> Self {
        ElementType::SoftwareSystem(SystemData {
            name: name.into(),
            description: description.into(),
            is_external: true,
        })
    }

    /// Create a new container element
    pub fn container(
        name: impl Into<String>,
        description: impl Into<String>,
        container_type: ContainerType,
        technology: impl Into<String>,
    ) -> Self {
        ElementType::Container(ContainerData {
            name: name.into(),
            description: description.into(),
            container_type,
            technology: technology.into(),
        })
    }
}

/// C1: Person/Actor element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonData {
    pub name: String,
    pub description: String,
    pub is_external: bool,
}

/// C1: Software System element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemData {
    pub name: String,
    pub description: String,
    pub is_external: bool,
}

/// C2: Container element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerData {
    pub name: String,
    pub description: String,
    pub container_type: ContainerType,
    pub technology: String,
}

/// Types of containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerType {
    WebApplication,
    MobileApp,
    Database,
    Microservice,
    Queue,
    Other(String),
}

impl ContainerType {
    pub fn as_str(&self) -> &str {
        match self {
            ContainerType::WebApplication => "Web Application",
            ContainerType::MobileApp => "Mobile App",
            ContainerType::Database => "Database",
            ContainerType::Microservice => "Microservice",
            ContainerType::Queue => "Message Queue",
            ContainerType::Other(s) => s.as_str(),
        }
    }
}
