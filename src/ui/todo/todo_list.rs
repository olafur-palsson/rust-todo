use yew::prelude::*;
use crate::app::todo::types::TodoList;
use crate::ui::todo::todo_item::TodoItemComp;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub list: TodoList,
}

#[function_component]
pub fn TodoComp(props: &Props) -> Html {
    html! {
        <>
            <h1>{props.list.name.clone()}</h1>
            {
                for props.list.items.iter().map(|other_todo| {
                    html! {
                        <TodoItemComp todo={other_todo.clone()}></TodoItemComp>
                    }
                })
            }
        </>
    }
}
