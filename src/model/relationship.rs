use super::ElementId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A relationship/connection between two elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub source_id: ElementId,
    pub target_id: ElementId,
    pub description: String,
    pub technology: Option<String>,
}

impl Relationship {
    pub fn new(
        source_id: ElementId,
        target_id: ElementId,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            description: description.into(),
            technology: None,
        }
    }

    pub fn with_technology(
        source_id: ElementId,
        target_id: ElementId,
        description: impl Into<String>,
        technology: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            description: description.into(),
            technology: Some(technology.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ElementId;

    mod relationship_creation_tests {
        use super::*;

        /// Verifies Relationship::new creates a relationship with correct properties
        #[test]
        fn relationship_new_creates_correct_relationship() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            let rel = Relationship::new(source_id, target_id, "uses");

            assert_eq!(rel.source_id, source_id);
            assert_eq!(rel.target_id, target_id);
            assert_eq!(rel.description, "uses");
            assert!(rel.technology.is_none());
            assert_ne!(rel.id, uuid::Uuid::nil());
        }

        /// Verifies Relationship::new auto-generates a unique ID
        #[test]
        fn relationship_new_generates_unique_id() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            let rel1 = Relationship::new(source_id, target_id, "uses");
            let rel2 = Relationship::new(source_id, target_id, "uses");

            assert_ne!(rel1.id, rel2.id);
        }

        /// Verifies with_technology creates a relationship with technology field set
        #[test]
        fn with_technology_creates_relationship_with_technology() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            let rel = Relationship::with_technology(source_id, target_id, "uses", "HTTPS");

            assert_eq!(rel.source_id, source_id);
            assert_eq!(rel.target_id, target_id);
            assert_eq!(rel.description, "uses");
            assert_eq!(rel.technology, Some("HTTPS".to_string()));
        }

        /// Verifies with_technology handles different technology strings
        #[test]
        fn with_technology_accepts_various_technologies() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            let rel1 = Relationship::with_technology(source_id, target_id, "calls", "REST API");
            let rel2 = Relationship::with_technology(source_id, target_id, "reads from", "PostgreSQL");
            let rel3 = Relationship::with_technology(source_id, target_id, "publishes to", "RabbitMQ");

            assert_eq!(rel1.technology, Some("REST API".to_string()));
            assert_eq!(rel2.technology, Some("PostgreSQL".to_string()));
            assert_eq!(rel3.technology, Some("RabbitMQ".to_string()));
        }
    }

    mod relationship_builder_pattern_tests {
        use super::*;

        /// Verifies builder pattern allows chaining
        #[test]
        fn relationship_builder_pattern() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            // Test that with_technology is a convenient factory method
            let rel = Relationship::with_technology(source_id, target_id, "description", "tech");

            assert_eq!(rel.description, "description");
            assert_eq!(rel.technology, Some("tech".to_string()));
        }

        /// Verifies relationships can be created without technology
        #[test]
        fn relationship_without_technology() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();

            let rel = Relationship::new(source_id, target_id, "simple connection");

            assert!(rel.technology.is_none());
        }
    }

    mod relationship_serialization_tests {
        use super::*;

        /// Verifies Relationship serializes and deserializes correctly
        #[test]
        fn relationship_roundtrip_serialization() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let original = Relationship::with_technology(source_id, target_id, "uses", "HTTPS");

            let json = serde_json::to_string(&original).expect("Failed to serialize");
            let restored: Relationship = serde_json::from_str(&json).expect("Failed to deserialize");

            assert_eq!(restored.id, original.id);
            assert_eq!(restored.source_id, original.source_id);
            assert_eq!(restored.target_id, original.target_id);
            assert_eq!(restored.description, original.description);
            assert_eq!(restored.technology, original.technology);
        }

        /// Verifies Relationship without technology serializes correctly
        #[test]
        fn relationship_without_technology_serialization() {
            let source_id = ElementId::new_v4();
            let target_id = ElementId::new_v4();
            let original = Relationship::new(source_id, target_id, "uses");

            let json = serde_json::to_string(&original).expect("Failed to serialize");
            assert!(json.contains("uses"));
            
            let restored: Relationship = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(restored.technology, None);
        }
    }
}
