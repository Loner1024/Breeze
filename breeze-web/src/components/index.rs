use yew::{function_component, Html, html, Properties};
use yew_router::components::Link;
use yew_hooks::{use_async_with_options, use_state_ptr_eq, UseAsyncOptions};
use std::rc::Rc;

use model::{PostList, PostListItem};

use crate::api::{ApiError, ApiRequest};

use crate::router::Route;


#[function_component]
pub fn Index() -> Html {
    let posts = use_state_ptr_eq(|| vec![]);
    use_async_with_options(
        {
            let posts = posts.clone();
            async move {
                let p: PostList = ApiRequest::get("https://breeze-production.up.railway.app/list").json_response().await?;
                posts.set(p.data);
                Ok::<_, Rc<ApiError>>(())
            }
        },
        UseAsyncOptions::enable_auto(),
    );
    html! {
        <main>
            <section class="article-list">
                {
                    for posts.iter().map(|item| html!{
                        <ListCard data={ item.clone() }/>
                    })
                }
            </section>
            <section class="list-pager">
                <a class="pre" href="#" style="display:inline-flex; justify-items: center">
                    <iconpark-icon name="left"></iconpark-icon>
                    { "上一页" }
                </a>
                <a class="next" href="#" style="display:inline-flex; justify-items: center">
                    <i class="fa-thin fa-right"></i>
                    { "下一页" }
                </a>
                <div class="clear"></div>
            </section>
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct ListCardProps {
    data: PostListItem,
}

#[function_component]
fn ListCard(props: &ListCardProps) -> Html {
    let ListCardProps { data } = props;
    html! {
            <article key={data.id}>
                <h2>
                    <Link<Route> to={Route::Post{ id: data.id }}>{ &data.title }</Link<Route>>
                    <span>{ "30度" }</span>
                </h2>
                <div class="excerpt">
                    <p>{ &data.summary }</p>
                </div>
                <div class="meta">
                    <span class="item"><i class="iconfont icon-calendar"></i>{ &data.create_time }</span>
                    <span class="item"><i class="iconfont icon-tag"></i>{ "tag" }</span>
                    <span class="item"><i class="iconfont icon-tag"></i>{ "17度" }</span>
                    <span class="item"><i class="iconfont icon-message"></i><a href="#">{ "评论" }</a></span>
                </div>
            </article>
    }
}
