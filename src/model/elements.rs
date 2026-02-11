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

#[cfg(test)]
mod tests {
    use super::*;

    mod element_creation_tests {
        use super::*;

        /// Verifies Element::new auto-generates a UUID
        #[test]
        fn element_new_generates_uuid() {
            let element = Element::new(
                ElementType::person("Test", "Description"),
                Position::new(0.0, 0.0),
            );

            // UUID should be non-nil
            assert_ne!(element.id, uuid::Uuid::nil());
        }

        /// Verifies Element::new sets correct element type
        #[test]
        fn element_new_sets_element_type() {
            let element = Element::new(
                ElementType::person("User", "A user"),
                Position::new(10.0, 20.0),
            );

            assert_eq!(element.name(), "User");
            assert_eq!(element.position.x, 10.0);
            assert_eq!(element.position.y, 20.0);
        }

        /// Verifies Element::new sets default size based on element type
        #[test]
        fn element_new_sets_default_size() {
            let person = Element::new(
                ElementType::person("User", "A user"),
                Position::new(0.0, 0.0),
            );
            let system = Element::new(
                ElementType::system("System", "A system"),
                Position::new(0.0, 0.0),
            );

            // Person has smaller default size
            assert_eq!(person.size.width, 120.0);
            assert_eq!(person.size.height, 80.0);

            // System has larger default size
            assert_eq!(system.size.width, 160.0);
            assert_eq!(system.size.height, 100.0);
        }
    }

    mod element_getter_tests {
        use super::*;

        /// Verifies name() returns correct name for all element types
        #[test]
        fn name_returns_correct_value() {
            let person = Element::new(
                ElementType::person("John", "A person"),
                Position::new(0.0, 0.0),
            );
            let system = Element::new(
                ElementType::system("MySystem", "A system"),
                Position::new(0.0, 0.0),
            );
            let container = Element::new(
                ElementType::container("WebApp", "A web app", ContainerType::WebApplication, "React"),
                Position::new(0.0, 0.0),
            );

            assert_eq!(person.name(), "John");
            assert_eq!(system.name(), "MySystem");
            assert_eq!(container.name(), "WebApp");
        }

        /// Verifies description() returns correct description for all element types
        #[test]
        fn description_returns_correct_value() {
            let person = Element::new(
                ElementType::person("John", "Person description"),
                Position::new(0.0, 0.0),
            );
            let system = Element::new(
                ElementType::system("MySystem", "System description"),
                Position::new(0.0, 0.0),
            );

            assert_eq!(person.description(), "Person description");
            assert_eq!(system.description(), "System description");
        }

        /// Verifies is_external() returns correct value for external elements
        #[test]
        fn is_external_returns_true_for_external_elements() {
            let external_person = Element::new(
                ElementType::external_person("External User", "External"),
                Position::new(0.0, 0.0),
            );
            let external_system = Element::new(
                ElementType::external_system("External System", "External"),
                Position::new(0.0, 0.0),
            );
            let internal_person = Element::new(
                ElementType::person("Internal User", "Internal"),
                Position::new(0.0, 0.0),
            );

            assert!(external_person.is_external());
            assert!(external_system.is_external());
            assert!(!internal_person.is_external());
        }

        /// Verifies is_external() returns false for containers
        #[test]
        fn is_external_returns_false_for_containers() {
            let container = Element::new(
                ElementType::container("WebApp", "A web app", ContainerType::WebApplication, "React"),
                Position::new(0.0, 0.0),
            );

            assert!(!container.is_external());
        }
    }

    mod element_setter_tests {
        use super::*;

        /// Verifies set_name updates the name for all element types
        #[test]
        fn set_name_updates_name() {
            let mut person = Element::new(
                ElementType::person("Old Name", "Description"),
                Position::new(0.0, 0.0),
            );
            person.set_name("New Name".to_string());

            assert_eq!(person.name(), "New Name");
        }

        /// Verifies set_description updates the description for all element types
        #[test]
        fn set_description_updates_description() {
            let mut system = Element::new(
                ElementType::system("System", "Old Description"),
                Position::new(0.0, 0.0),
            );
            system.set_description("New Description".to_string());

            assert_eq!(system.description(), "New Description");
        }
    }

    mod element_type_factory_tests {
        use super::*;

        /// Verifies ElementType::person creates internal person
        #[test]
        fn person_factory_creates_internal_person() {
            let et = ElementType::person("User", "A user");

            match et {
                ElementType::Person(data) => {
                    assert_eq!(data.name, "User");
                    assert_eq!(data.description, "A user");
                    assert!(!data.is_external);
                }
                _ => panic!("Expected Person variant"),
            }
        }

        /// Verifies ElementType::external_person creates external person
        #[test]
        fn external_person_factory_creates_external_person() {
            let et = ElementType::external_person("External", "An external user");

            match et {
                ElementType::Person(data) => {
                    assert_eq!(data.name, "External");
                    assert!(data.is_external);
                }
                _ => panic!("Expected Person variant"),
            }
        }

        /// Verifies ElementType::system creates internal system
        #[test]
        fn system_factory_creates_internal_system() {
            let et = ElementType::system("MySystem", "A system");

            match et {
                ElementType::SoftwareSystem(data) => {
                    assert_eq!(data.name, "MySystem");
                    assert!(!data.is_external);
                }
                _ => panic!("Expected SoftwareSystem variant"),
            }
        }

        /// Verifies ElementType::external_system creates external system
        #[test]
        fn external_system_factory_creates_external_system() {
            let et = ElementType::external_system("External", "An external system");

            match et {
                ElementType::SoftwareSystem(data) => {
                    assert!(data.is_external);
                }
                _ => panic!("Expected SoftwareSystem variant"),
            }
        }

        /// Verifies ElementType::container creates container with all properties
        #[test]
        fn container_factory_creates_container() {
            let et = ElementType::container("WebApp", "A web app", ContainerType::WebApplication, "React");

            match et {
                ElementType::Container(data) => {
                    assert_eq!(data.name, "WebApp");
                    assert_eq!(data.description, "A web app");
                    match data.container_type {
                        ContainerType::WebApplication => {}
                        _ => panic!("Expected WebApplication container type"),
                    }
                    assert_eq!(data.technology, "React");
                }
                _ => panic!("Expected Container variant"),
            }
        }
    }

    mod element_type_method_tests {
        use super::*;

        /// Verifies type_name returns correct display names
        #[test]
        fn type_name_returns_correct_value() {
            let person = ElementType::person("User", "Description");
            let system = ElementType::system("System", "Description");
            let container = ElementType::container("App", "Description", ContainerType::Microservice, "");

            assert_eq!(person.type_name(), "Person");
            assert_eq!(system.type_name(), "Software System");
            assert_eq!(container.type_name(), "Container");
        }

        /// Verifies default_size returns correct sizes for each type
        #[test]
        fn default_size_returns_correct_dimensions() {
            let person = ElementType::person("User", "Description");
            let system = ElementType::system("System", "Description");
            let container = ElementType::container("App", "Description", ContainerType::Database, "");

            assert_eq!(person.default_size(), Size::new(120.0, 80.0));
            assert_eq!(system.default_size(), Size::new(160.0, 100.0));
            assert_eq!(container.default_size(), Size::new(160.0, 100.0));
        }
    }

    mod container_type_tests {
        use super::*;

        /// Verifies ContainerType::as_str returns correct display strings
        #[test]
        fn container_type_as_str_returns_correct_strings() {
            assert_eq!(ContainerType::WebApplication.as_str(), "Web Application");
            assert_eq!(ContainerType::MobileApp.as_str(), "Mobile App");
            assert_eq!(ContainerType::Database.as_str(), "Database");
            assert_eq!(ContainerType::Microservice.as_str(), "Microservice");
            assert_eq!(ContainerType::Queue.as_str(), "Message Queue");
            assert_eq!(ContainerType::Other("Custom".to_string()).as_str(), "Custom");
        }
    }

    mod positioned_trait_tests {
        use super::*;

        /// Verifies Positioned trait is correctly implemented for Element
        #[test]
        fn element_implements_positioned() {
            let mut element = Element::new(
                ElementType::person("User", "Description"),
                Position::new(10.0, 20.0),
            );

            // Test getters
            assert_eq!(element.position().x, 10.0);
            assert_eq!(element.size().width, 120.0);

            // Test setters
            element.set_position(Position::new(30.0, 40.0));
            element.set_size(Size::new(200.0, 150.0));

            assert_eq!(element.position().x, 30.0);
            assert_eq!(element.size().width, 200.0);
        }
    }
}
