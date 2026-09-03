use crate::protocol::types::registry_entry::RegistryEntry;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Default)]
pub struct Registry {
    pub registries: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_to_registry(&mut self, registry_id: String, entries: Vec<RegistryEntry>) {
        let registry_map = self.registries.entry(registry_id.clone()).or_default();
        for entry in entries {
            match entry.data {
                Some(data) => {
                    if let Ok(json_value) = serde_json::to_value(&data) {
                        registry_map.insert(entry.id, json_value);
                    } else {
                        eprintln!(
                            "Failed to convert NBT data to JSON for entry ID: {}",
                            entry.id
                        );
                    }
                }
                None => {
                    if let Some(local_json) = Self::load_from_assets(&registry_id, &entry.id) {
                        registry_map.insert(entry.id, local_json);
                    } else {
                        eprintln!(
                            "No data found for entry ID: {} in registry: {}",
                            entry.id, registry_id
                        );
                    }
                }
            }
        }
    }

    pub fn load_from_assets(registry_id: &str, entry_id: &str) -> Option<serde_json::Value> {
        let (namespace, path) = registry_id.split_once(':')?;

        let entry_name = entry_id.split_once(':')?.1;

        let file_path = format!("assets/data/{}/{}/{}.json", namespace, path, entry_name);

        if Path::new(&file_path).exists() {
            let file_content = fs::read_to_string(&file_path).ok()?;
            serde_json::from_str(&file_content).ok()
        } else {
            None
        }
    }

    pub fn get(&self, registry_id: &str, entry_id: &str) -> Option<&serde_json::Value> {
        self.registries.get(registry_id)?.get(entry_id)
    }
}
