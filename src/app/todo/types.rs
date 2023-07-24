use serde::Serialize;
use yew::Properties;

#[derive(Debug, Serialize, Default, Properties, PartialEq, Clone)]
pub struct TodoList {
    pub id: String,
    pub name: String,
    pub items: Vec<TodoItem>
}


#[derive(Debug, Serialize, Default, Properties, PartialEq, Clone)]
pub struct TodoItem {
    pub id: String,
    pub list_id: String,
    pub label: String,
    pub details: Option<String>,
    pub is_done: bool,
}
