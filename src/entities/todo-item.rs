use serde::Serialize;

#[derive(Serialize)]
pub struct TodoItem {
    pub id: String,
    pub label: String,
    pub details: Option<String>
}
