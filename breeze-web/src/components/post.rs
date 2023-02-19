use yew::{function_component, Html, html};

#[function_component]
pub fn Post() -> Html {
    html! {
        <main>
            <article class="content">
                <h1>{ "Title" }</h1>
                <div class="meta">
                    <span class="item">{ "2023-02-19" }</span>
                    <span class="item">{ "Category" }</span>
                    <span class="item">{ "19度" }</span>
                    <span class="item">{ "20评" }</span>
                </div>
                <div class="post">
                    <p>
                        { "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ab accusantium dicta dolorum fugit veritatis?
                        Accusantium blanditiis deleniti, deserunt error illo ipsam odio officiis provident reiciendis
                        reprehenderit
                        sed tempora tempore totam." }
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
