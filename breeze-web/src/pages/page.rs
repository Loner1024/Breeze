use yew::{function_component, Html, html};

use crate::components::{Header, Index, Post, Footer};

#[function_component]
pub fn ArticlePage() -> Html {
    html!(
        <div>
            <body class="wrapper">
            <Header />
            <Post />
            </body>
            <Footer />
        </div>
    )
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <body class="wrapper">
            <Header />
            <Index />
            </body>
            <Footer />
        </div>
    }
}
