use yew::{function_component, Html, html};

use components::{header::Header, index::Index};

mod components;

#[function_component]
fn App() -> Html {
    html! {
        <body class="wrapper">
        <Header />
        <Index />
        </body>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
