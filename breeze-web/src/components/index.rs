use yew::{function_component, Html, html};

#[function_component]
pub fn Index() -> Html {
    html! {
        <main>
            <section class="article-list">
                <article>
                    <h2>
                        <a href="#">{ "Title" }</a>
                        <span>{ "30度" }</span>
                    </h2>
                    <div class="excerpt">
                        <p>{ "Summary" }</p>
                    </div>
                    <div class="meta">
                        <span class="item"><i class="iconfont icon-calendar"></i>{ "time" }</span>
                        <span class="item"><i class="iconfont icon-tag"></i>{ "tag" }</span>
                        <span class="item"><i class="iconfont icon-tag"></i>{ "17度" }</span>
                        <span class="item"><i class="iconfont icon-message"></i><a href="#">{ "评论" }</a></span>
                    </div>
                </article>
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
