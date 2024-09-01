use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="p-4 text-xl text-center text-white bg-blue-800 font-roboto">
            <h1>{ "Learn HTML, CSS, and JavaScript from Scratch" }</h1>
        </header>
    }
}
