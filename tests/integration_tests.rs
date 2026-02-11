//! Integration tests for C2Draw
//!
//! These tests verify end-to-end workflows including:
//! - Full diagram creation
//! - Serialization/deserialization
//! - Export to various formats

use c2draw::export::{DiagramExporter, MermaidExporter, PlantUmlExporter};
use c2draw::model::{
    ContainerType, Diagram, DiagramType, Element, ElementId, ElementType, Position, Positioned,
    Relationship,
};

/// Creates a complete system context diagram for testing
fn create_system_context_diagram() -> Diagram {
    let mut diagram = Diagram::new(
        "System Context Diagram",
        "A simple system context diagram for testing",
        DiagramType::SystemContext,
    );

    // Add users
    let user = Element::new(
        ElementType::person("User", "An end user of the system"),
        Position::new(50.0, 100.0),
    );
    let user_id = user.id;
    diagram.add_element(user);

    // Add external system
    let external = Element::new(
        ElementType::external_system("External API", "Third-party service"),
        Position::new(300.0, 50.0),
    );
    let external_id = external.id;
    diagram.add_element(external);

    // Add main system
    let system = Element::new(
        ElementType::system("My Application", "The main application"),
        Position::new(300.0, 200.0),
    );
    let system_id = system.id;
    diagram.add_element(system);

    // Add relationships
    diagram.add_relationship(Relationship::new(user_id, system_id, "uses"));
    diagram.add_relationship(Relationship::with_technology(
        system_id,
        external_id,
        "fetches data from",
        "HTTPS/JSON",
    ));

    diagram
}

/// Creates a complete container diagram for testing
fn create_container_diagram() -> Diagram {
    let mut diagram = Diagram::new(
        "Container Diagram",
        "A container diagram for testing",
        DiagramType::Container,
    );

    // Add user
    let user = Element::new(
        ElementType::person("User", "An end user"),
        Position::new(50.0, 150.0),
    );
    let user_id = user.id;
    diagram.add_element(user);

    // Add web application
    let web_app = Element::new(
        ElementType::container(
            "Web Application",
            "Delivers the web UI",
            ContainerType::WebApplication,
            "React",
        ),
        Position::new(300.0, 100.0),
    );
    let web_app_id = web_app.id;
    diagram.add_element(web_app);

    // Add API
    let api = Element::new(
        ElementType::container(
            "API",
            "Handles business logic",
            ContainerType::Microservice,
            "Rust/Axum",
        ),
        Position::new(300.0, 250.0),
    );
    let api_id = api.id;
    diagram.add_element(api);

    // Add database
    let database = Element::new(
        ElementType::container(
            "Database",
            "Stores user data",
            ContainerType::Database,
            "PostgreSQL",
        ),
        Position::new(550.0, 250.0),
    );
    let database_id = database.id;
    diagram.add_element(database);

    // Add message queue
    let queue = Element::new(
        ElementType::container(
            "Message Queue",
            "Async processing",
            ContainerType::Queue,
            "RabbitMQ",
        ),
        Position::new(550.0, 100.0),
    );
    let queue_id = queue.id;
    diagram.add_element(queue);

    // Add relationships
    diagram.add_relationship(Relationship::new(user_id, web_app_id, "visits"));
    diagram.add_relationship(Relationship::with_technology(
        web_app_id,
        api_id,
        "calls",
        "REST API",
    ));
    diagram.add_relationship(Relationship::with_technology(
        api_id,
        database_id,
        "reads/writes",
        "SQL",
    ));
    diagram.add_relationship(Relationship::with_technology(
        api_id,
        queue_id,
        "publishes to",
        "AMQP",
    ));

    diagram
}

mod diagram_workflow_tests {
    use super::*;

    /// Verifies a complete system context diagram can be created
    #[test]
    fn create_system_context_diagram_workflow() {
        let diagram = create_system_context_diagram();

        assert_eq!(diagram.name, "System Context Diagram");
        assert_eq!(diagram.diagram_type, DiagramType::SystemContext);
        assert_eq!(diagram.elements.len(), 3);
        assert_eq!(diagram.relationships.len(), 2);
    }

    /// Verifies a complete container diagram can be created
    #[test]
    fn create_container_diagram_workflow() {
        let diagram = create_container_diagram();

        assert_eq!(diagram.name, "Container Diagram");
        assert_eq!(diagram.diagram_type, DiagramType::Container);
        assert_eq!(diagram.elements.len(), 5);
        assert_eq!(diagram.relationships.len(), 4);
    }

    /// Verifies elements can be modified after being added to diagram
    #[test]
    fn modify_elements_in_diagram() {
        let mut diagram = create_system_context_diagram();

        // Find and modify an element
        let element_id = diagram.elements.values().next().unwrap().id;
        if let Some(element) = diagram.get_element_mut(element_id) {
            Element::set_name(element, "Modified Name".to_string());
            element.set_position(Position::new(999.0, 888.0));
        }

        // Verify modification
        let element = diagram.get_element(element_id).unwrap();
        assert_eq!(element.name(), "Modified Name");
        assert_eq!(element.position.x, 999.0);
        assert_eq!(element.position.y, 888.0);
    }

    /// Verifies removing an element cascades to relationships
    #[test]
    fn remove_element_cascades_to_relationships() {
        let mut diagram = create_system_context_diagram();
        let initial_rel_count = diagram.relationships.len();
        assert_eq!(initial_rel_count, 2);

        // Find the user element (has 1 outgoing relationship)
        let user_id: ElementId = diagram
            .elements
            .values()
            .find(|e: &&Element| e.name() == "User")
            .unwrap()
            .id;

        diagram.remove_element(user_id);

        assert_eq!(diagram.elements.len(), 2);
        assert_eq!(diagram.relationships.len(), 1); // One relationship should be removed
    }

    /// Verifies relationship queries work correctly
    #[test]
    fn relationship_queries_integration() {
        let diagram = create_container_diagram();

        // Find the API element
        let api_id: ElementId = diagram
            .elements
            .values()
            .find(|e: &&Element| e.name() == "API")
            .unwrap()
            .id;

        // API should have 3 connected relationships (1 incoming, 2 outgoing)
        let connected = diagram.relationships_connected_to(api_id);
        assert_eq!(connected.len(), 3);

        // API should have 1 incoming relationship
        let incoming = diagram.relationships_to(api_id);
        assert_eq!(incoming.len(), 1);

        // API should have 2 outgoing relationships
        let outgoing = diagram.relationships_from(api_id);
        assert_eq!(outgoing.len(), 2);
    }
}

mod serialization_workflow_tests {
    use super::*;

    /// Verifies a complete diagram can be serialized to JSON and back
    #[test]
    fn diagram_json_roundtrip() {
        let original = create_system_context_diagram();
        let element_ids: Vec<ElementId> = original.elements.keys().copied().collect();
        let relationship_count = original.relationships.len();

        let json = original.to_json().expect("Failed to serialize diagram");
        let restored = Diagram::from_json(&json).expect("Failed to deserialize diagram");

        assert_eq!(restored.name, original.name);
        assert_eq!(restored.description, original.description);
        assert_eq!(restored.diagram_type, original.diagram_type);
        assert_eq!(restored.elements.len(), original.elements.len());
        assert_eq!(restored.relationships.len(), relationship_count);

        // Verify all elements are restored
        for id in element_ids {
            assert!(restored.elements.contains_key(&id));
        }
    }

    /// Verifies complex container diagram serializes correctly
    #[test]
    fn container_diagram_serialization() {
        let original = create_container_diagram();

        let json = original.to_json().expect("Failed to serialize");
        let restored = Diagram::from_json(&json).expect("Failed to deserialize");

        assert_eq!(restored.elements.len(), 5);
        assert_eq!(restored.relationships.len(), 4);

        // Verify container types are preserved
        let container_count = restored
            .elements
            .values()
            .filter(|e| matches!(e.element_type, ElementType::Container(_)))
            .count();
        assert_eq!(container_count, 4);
    }

    /// Verifies diagram can be saved and loaded multiple times
    #[test]
    fn multiple_save_load_cycles() {
        let mut diagram = create_system_context_diagram();

        // Multiple save/load cycles
        for i in 0..5 {
            let json = diagram.to_json().expect("Failed to serialize");
            diagram = Diagram::from_json(&json).expect("Failed to deserialize");

            // Modify between cycles
            if let Some(element) = diagram.elements.values_mut().next() {
                Element::set_name(element, format!("Version {}", i));
            }
        }

        assert_eq!(diagram.elements.len(), 3);
    }
}

mod export_workflow_tests {
    use super::*;

    /// Verifies PlantUML export produces valid output for system context
    #[test]
    fn plantuml_export_system_context() {
        let diagram = create_system_context_diagram();
        let exporter = PlantUmlExporter::new();

        let output = exporter.export(&diagram);

        assert!(output.contains("@startuml"));
        assert!(output.contains("@enduml"));
        assert!(output.contains("C4_Context.puml"));
        assert!(output.contains("title System Context Diagram"));
        assert!(output.contains("Person"));
        assert!(output.contains("System"));
        assert!(output.contains("Rel("));
    }

    /// Verifies PlantUML export produces valid output for container diagram
    #[test]
    fn plantuml_export_container() {
        let diagram = create_container_diagram();
        let exporter = PlantUmlExporter::new();

        let output = exporter.export(&diagram);

        assert!(output.contains("C4_Container.puml"));
        assert!(output.contains("Container("));
        assert!(output.contains("ContainerDb"));
        assert!(output.contains("ContainerQueue"));
    }

    /// Verifies Mermaid export produces valid output for system context
    #[test]
    fn mermaid_export_system_context() {
        let diagram = create_system_context_diagram();
        let exporter = MermaidExporter::new();

        let output = exporter.export(&diagram);

        assert!(output.starts_with("C4Context"));
        assert!(output.contains("title System Context Diagram"));
        assert!(output.contains("Person("));
        assert!(output.contains("BiRel("));
    }

    /// Verifies Mermaid export produces valid output for container diagram
    #[test]
    fn mermaid_export_container() {
        let diagram = create_container_diagram();
        let exporter = MermaidExporter::new();

        let output = exporter.export(&diagram);

        assert!(output.starts_with("C4Container"));
        assert!(output.contains("Container("));
    }

    /// Verifies exports contain all elements
    #[test]
    fn export_contains_all_elements() {
        let diagram = create_container_diagram();
        let plantuml = PlantUmlExporter::new().export(&diagram);
        let mermaid = MermaidExporter::new().export(&diagram);

        // All element names should appear in exports
        for element in diagram.elements.values() {
            let name: &str = element.name();
            assert!(
                plantuml.contains(name),
                "PlantUML export missing element: {}",
                name
            );
            assert!(
                mermaid.contains(name),
                "Mermaid export missing element: {}",
                name
            );
        }
    }

    /// Verifies exports contain all relationships
    #[test]
    fn export_contains_all_relationships() {
        let diagram = create_system_context_diagram();
        let plantuml = PlantUmlExporter::new().export(&diagram);
        let mermaid = MermaidExporter::new().export(&diagram);

        // All relationship descriptions should appear
        for rel in &diagram.relationships {
            assert!(
                plantuml.contains(&rel.description),
                "PlantUML export missing relationship: {}",
                rel.description
            );
            assert!(
                mermaid.contains(&rel.description),
                "Mermaid export missing relationship: {}",
                rel.description
            );
        }
    }
}

mod end_to_end_tests {
    use super::*;

    /// Full workflow: Create -> Modify -> Save -> Load -> Export
    #[test]
    fn full_workflow_create_modify_save_load_export() {
        // Create diagram
        let mut diagram = Diagram::new(
            "E2E Test Diagram",
            "Testing complete workflow",
            DiagramType::SystemContext,
        );

        // Add elements
        let user = Element::new(
            ElementType::person("User", "End user"),
            Position::new(50.0, 50.0),
        );
        let user_id = user.id;
        diagram.add_element(user);

        let system = Element::new(
            ElementType::system("System", "Main system"),
            Position::new(200.0, 50.0),
        );
        let system_id = system.id;
        diagram.add_element(system);

        // Add relationship
        diagram.add_relationship(Relationship::new(user_id, system_id, "uses"));

        // Modify
        diagram.name = "Modified E2E Test".to_string();
        if let Some(elem) = diagram.get_element_mut(user_id) {
            Element::set_name(elem, "Modified User".to_string());
        }

        // Save
        let json = diagram.to_json().expect("Failed to serialize");

        // Load
        let loaded = Diagram::from_json(&json).expect("Failed to deserialize");

        // Export
        let plantuml = PlantUmlExporter::new().export(&loaded);
        let mermaid = MermaidExporter::new().export(&loaded);

        // Verify
        assert_eq!(loaded.name, "Modified E2E Test");
        assert!(plantuml.contains("Modified User"));
        assert!(mermaid.contains("Modified User"));
        assert!(plantuml.contains("uses"));
        assert!(mermaid.contains("uses"));
    }

    /// Tests empty diagram handling
    #[test]
    fn empty_diagram_workflow() {
        let diagram = Diagram::new("Empty", "", DiagramType::SystemContext);

        let json = diagram.to_json().expect("Failed to serialize empty");
        let loaded = Diagram::from_json(&json).expect("Failed to deserialize empty");

        assert!(loaded.elements.is_empty());
        assert!(loaded.relationships.is_empty());

        let plantuml = PlantUmlExporter::new().export(&loaded);
        let mermaid = MermaidExporter::new().export(&loaded);

        assert!(plantuml.contains("@startuml"));
        assert!(plantuml.contains("@enduml"));
        assert!(mermaid.starts_with("C4Context"));
    }

    /// Tests diagram with external elements
    #[test]
    fn external_elements_workflow() {
        let mut diagram = Diagram::new("External Test", "", DiagramType::SystemContext);

        let external_person = Element::new(
            ElementType::external_person("Customer", "External customer"),
            Position::new(0.0, 0.0),
        );
        let external_system = Element::new(
            ElementType::external_system("Payment Gateway", "Stripe"),
            Position::new(200.0, 0.0),
        );
        let internal = Element::new(
            ElementType::system("Our System", "Main application"),
            Position::new(100.0, 100.0),
        );

        diagram.add_element(external_person);
        diagram.add_element(external_system);
        diagram.add_element(internal);

        let plantuml = PlantUmlExporter::new().export(&diagram);
        let mermaid = MermaidExporter::new().export(&diagram);

        // Check for external markers
        assert!(plantuml.contains("Person_Ext") || plantuml.contains("System_Ext"));
        assert!(mermaid.contains("Person_Ext") || mermaid.contains("System_Ext"));
    }
}

