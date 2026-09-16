use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Tags {
    pub tags: HashMap<String, HashMap<String, HashSet<i32>>>,
}

impl Tags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_tag(&mut self, tag_registry: String, tag_name: String, entries: Vec<i32>) {
        let registry_tags = self.tags.entry(tag_registry).or_default();
        registry_tags.insert(tag_name, entries.into_iter().collect());
    }

    pub fn is_in_tag(&self, tag_registry: &str, tag_name: &str, tag_id: i32) -> bool {
        self.tags.get(tag_registry).is_some_and(|registry_tags| {
            registry_tags
                .get(tag_name)
                .is_some_and(|tag_entries| tag_entries.contains(&tag_id))
        })
    }
}
