use leptos::*;

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <h1>"My Leptos App"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
        </header>
    }
}
