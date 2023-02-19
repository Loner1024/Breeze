use yew::{function_component, Html, html};
use yew_router::{BrowserRouter, Switch};

use router::{Route, switch};

mod components;
mod router;
mod pages;
mod api;


#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
