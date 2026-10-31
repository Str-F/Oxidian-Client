use crate::protocol::types::registry_entry::RegistryEntry;
use log::{debug, warn};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Registry {
    pub registries: HashMap<String, HashMap<String, serde_json::Value>>,
    pub assets_dir: PathBuf,
}

impl Registry {
    pub fn new(assets_dir: PathBuf) -> Self {
        Self {
            registries: HashMap::new(),
            assets_dir,
        }
    }

    pub fn assets_available(&self) -> bool {
        self.assets_dir.join("data").is_dir()
    }

    pub fn add_to_registry(&mut self, registry_id: String, entries: Vec<RegistryEntry>) {
        let assets_dir = self.assets_dir.clone();
        let registry_map = self.registries.entry(registry_id.clone()).or_default();

        for entry in entries {
            match entry.data {
                Some(data) => {
                    if let Ok(json_value) = serde_json::to_value(&data) {
                        registry_map.insert(entry.id, json_value);
                    } else {
                        warn!(
                            "Failed to convert NBT data to JSON for entry ID: {}",
                            entry.id
                        );
                    }
                }
                None => {
                    if let Some(local_json) =
                        Self::load_from_assets(&assets_dir, &registry_id, &entry.id)
                    {
                        registry_map.insert(entry.id, local_json);
                    } else {
                        debug!(
                            "No data found for entry id: {} in registry: {}",
                            entry.id, registry_id
                        );
                    }
                }
            }
        }
    }

    pub fn load_from_assets(
        assets_dir: &Path,
        registry_id: &str,
        entry_id: &str,
    ) -> Option<serde_json::Value> {
        let (namespace, path) = registry_id.split_once(':')?;

        let entry_name = entry_id.split_once(':')?.1;

        let file_path = assets_dir
            .join("data")
            .join(namespace)
            .join(path)
            .join(format!("{entry_name}.json"));

        let file_contents = fs::read_to_string(&file_path).ok()?;
        serde_json::from_str(&file_contents).ok()
    }

    #[expect(dead_code)]
    pub fn get(&self, registry_id: &str, entry_id: &str) -> Option<&serde_json::Value> {
        self.registries.get(registry_id)?.get(entry_id)
    }
}
