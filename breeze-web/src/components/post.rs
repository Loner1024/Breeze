use yew::{function_component, Html, html, Properties};
use yew_hooks::{use_async_with_options, use_state_ptr_eq, UseAsyncOptions};
use std::rc::Rc;

use model::{PostDetail};

use crate::api::{ApiError, ApiRequest};


#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: i64,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    let PostProps { id } = props;
    let post_detail = use_state_ptr_eq(|| PostDetail::default());
    use_async_with_options(
        {
            let url = format!("https://breeze-production.up.railway.app/post/{0}", id);
            let post_detail = post_detail.clone();
            async move {
                let p: PostDetail = ApiRequest::get(url).json_response().await?;
                post_detail.set(p);
                Ok::<_, Rc<ApiError>>(())
            }
        },
        UseAsyncOptions::enable_auto(),
    );
    html! {
        <main>
            <article class="content">
                <h1>{ &post_detail.title }</h1>
                <div class="meta">
                    <span class="item">{ &post_detail.create_time }</span>
                    <span class="item">{ "Category" }</span>
                    <span class="item">{ "19度" }</span>
                    <span class="item">{ "20评" }</span>
                </div>
                <div class="post">
                    <p>
                        { &post_detail.content }
                    </p>
                </div>
            </article>
            <section class="pager">
                <a class="pre" href="#">{ "pre title" }</a>
                <a class="next" href="#">{ "next title" }</a>
            </section>
        </main>
    }
}
