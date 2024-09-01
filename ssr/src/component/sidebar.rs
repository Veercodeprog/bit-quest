use leptos::*;

#[component]
pub fn Sidebar(
    cx: Scope,
    current_section: Option<&'static str>,
    on_nav: impl Fn(&'static str) -> (),
) -> impl IntoView {
    view! { cx,
        <aside class="overflow-y-auto p-4 w-1/5 bg-white border-r border-gray-300 shadow-md">
            <nav>
                <ul>
                    <li class="mb-4">
                        <a href="#introduction" class={if current_section == Some("introduction") { "bg-blue-500 text-white" } else { "text-gray-800" }} onclick={move |_| on_nav("introduction")}>
                            { "Introduction" }
                            <span class="circle"></span>
                        </a>
                        <ul class="pl-4">
                            <li>
                                <a href="#getting-started" class={if current_section == Some("getting-started") { "bg-blue-500 text-white" } else { "text-gray-800" }} onclick={move |_| on_nav("getting-started")}>
                                    { "Getting Started" }
                                </a>
                            </li>
                        </ul>
                    </li>
                    {/* Repeat for other sections */}
                </ul>
            </nav>
        </aside>
    }
}
