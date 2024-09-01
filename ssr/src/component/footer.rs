use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="p-4 text-sm text-center text-white bg-blue-800 font-roboto">
            { "Footer Content" }
            </footer>

    }
}
