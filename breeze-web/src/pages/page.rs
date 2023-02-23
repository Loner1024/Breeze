use yew::{function_component, Html, html, Properties};

use crate::components::{Header, Index, Post, Footer};

#[derive(Properties, PartialEq)]
pub struct ArticlePageProps {
    pub id: i64,
}

#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
    let ArticlePageProps { id } = props;
    html!(
        <div>
            <body class="wrapper">
            <Header />
            <Post id = { id } />
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
