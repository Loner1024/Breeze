use yew::{function_component, Html, html};

#[function_component]
pub fn Footer() -> Html {
    html! {
        <div>
            <aside>
        <div class="aside-left sidebar">
            <h3>{ "最新文章" }</h3>
            <ul>
                <li>
                    <a href="#">{ "1" }</a>
                    <span>{ "19度" }</span>
                </li>
            </ul>
            <div class="clear"></div>
        </div>
        <div class="aside-right sidebar">
            <h3>{ "分门别类" }</h3>
            <ul>
                <li>
                    <a href="#">{ "1" }</a>
                    <span>{ "19篇" }</span>
                </li>
            </ul>
            <div class="clear"></div>
        </div>
    </aside>
    <footer>
        <span>{ "© 2023-Uniix" }</span>
        <span>{ "♥" }</span>
        <div class="powered_by">
            <span>{ "Proudly published with" }</span>
            <a href="#">{ "HeHe" }</a>
        </div>
        <div class="footer_slogan">{ "HeHe" }</div>
    </footer>
        </div>
    }
}