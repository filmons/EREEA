use std::collections::HashMap;

#[derive(Clone)]
pub enum ResourceType {
    Energie,
    Lieu,
    Minerai,
}
#[derive(Clone)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub is_consumed: bool,
}

impl Resource {
    pub fn new(resource_type: ResourceType) -> Self {
        Self { resource_type, is_consumed: false }
    }
}