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
        self.tags
            .get(tag_registry)
            .and_then(|reg_tags| reg_tags.get(tag_name))
            .map_or(false, |set| set.contains(&tag_id))
    }
}
