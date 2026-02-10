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
