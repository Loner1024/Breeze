use yew::{Html, html};
use yew_router::Routable;
use crate::pages::{Home, ArticlePage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    App,
    #[at("/post/:id")]
    Post { id: i64 },
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::App => html!(<Home />),
        Route::Post { id } => html!(<ArticlePage id={ id }  />),
    }
}
