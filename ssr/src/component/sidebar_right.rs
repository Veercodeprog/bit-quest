use leptos::*;

#[component]
pub fn SidebarRight(cx: Scope) -> impl IntoView {
    view! { cx,
        <aside class="overflow-y-auto p-4 w-1/4 bg-white border-l border-gray-300 shadow-md">
            <div>
                <h3 class="text-gray-700">{ "Related Courses" }</h3>
                <ul>
                    <li><a href="#" class="text-blue-500">{ "Advanced HTML" }</a></li>
                    <li><a href="#" class="text-blue-500">{ "CSS for Beginners" }</a></li>
                    <li><a href="#" class="text-blue-500">{ "JavaScript Mastery" }</a></li>
                </ul>
            </div>
        </aside>
    }
}
