use yew::prelude::*;
use crate::app::todo::types::TodoItem;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub todo: TodoItem
}

#[function_component]
pub fn TodoItemComp(props: &Props) -> Html {
    html! {
        <>
            <p>{props.todo.label.clone()}</p>
            <label>{"Done"}</label>
            <input id={props.todo.id.clone()} type="checkbox" checked={props.todo.is_done} />
        </>
    }
}
