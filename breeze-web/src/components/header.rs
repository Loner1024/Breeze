use yew::{function_component, Html, html};

#[function_component]
pub fn Header() -> Html {
    html! {
<header>
    <a class="logo">
        <img src="https://images.unsplash.com/photo-1533738363-b7f9aef128ce?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1335&q=80"/>
    </a>
    <div class="description">
        <h1>{ "Uniiiiiiiiix" }</h1>
        <h2>{ "Somethings..." }</h2>
        <nav>
            <div class="bitcron_nav_container">
                <div class="bitcron_nav">
                    <div class="mixed_site_nav_wrap site_nav_wrap">
                        <ul class="mixed_site_nav site_nav sm sm-base">
                            <li><a class="selected active current nav__item" href="/"> { "首页" }</a>
                            </li>
                            <li><a class=" nav__item" href="/archive">{ "归档" }</a></li>
                            <li><a class=" nav__item" href="#">{ "管理" }</a></li>
                        </ul>
                        <div class="clear clear_nav_inline_end">
                        </div>
                    </div>
                </div>
                <div class="clear clear_nav_end"></div>
            </div>
        </nav>
    </div>
</header>
    }
}