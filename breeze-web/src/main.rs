use yew::{function_component, Html, html};
use yew_router::{BrowserRouter, Switch};

use router::{Route, switch};

mod components;
mod router;
mod pages;


#[function_component]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<app>::new().render();
}
