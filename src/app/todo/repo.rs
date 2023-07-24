use crate::app::todo::types::{TodoList};

struct TodoRepo {
    pub lists: Vec<TodoList>,
}

// In memory database
impl TodoRepo {
    pub fn new() -> TodoRepo {
        TodoRepo {
            lists: vec![],
        }
    }
    
    pub fn find(&mut self, id: String) -> Option<TodoList> {
        let list = self.lists.iter().find(|list| {
            list.id == id
        });
        match list {
            Some(list) => Some(list.clone()),
            None => None
        }
    }
    
    pub fn upsert(&mut self, update_list: TodoList) {
        let mut after_update = vec![];
        for list in &self.lists {
            if list.id != update_list.id {
                after_update.push(list.clone())
            }
        }
        
        self.lists = after_update;
    }
    
    
    pub fn get_lists(&mut self) -> Vec<TodoList> {
        self.lists.to_vec()
    }
}