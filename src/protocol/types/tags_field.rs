use fastnbt::Value;

#[derive(Debug)]
pub struct TagsField {
    pub name: String,
    pub entries: Vec<i32>,
}
