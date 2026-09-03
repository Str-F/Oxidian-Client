use fastnbt::Value;

#[derive(Debug)]
pub struct RegistryEntry {
    pub id: String,
    pub data: Option<Value>,
}
