use yew::{function_component, html, Html, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Children,
}

#[function_component]
pub fn Page(props: &PageProps) -> Html {
    html! {
        // <!DOCTYPE html>
        <html>
            <head>
            </head>
            <body>
            { for props.children.iter() }
            </body>
            <script src="https://unpkg.com/htmx.org@1.9.3"></script>
        </html>
    }
}
