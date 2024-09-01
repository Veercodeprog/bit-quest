use crate::components::{Footer, Header, Sidebar, SidebarRight};
use leptos::*; // Ensure all necessary imports from Leptos are included

#[component]
pub fn Web3Notes(cx: Scope) -> impl IntoView {
    // State to keep track of the current section
    let current_section = create_signal(cx, None);

    // Navigation function to set the current section
    let navigate_to = move |id: &'static str| {
        current_section.set(Some(id));
        // Add scrolling behavior if needed
    };

    view! { cx,
        <div class="flex flex-col h-screen">
            <Header />
            <div class="flex overflow-hidden flex-1">
                <Sidebar current_section=*current_section.get() on_nav=navigate_to />
                <main class="overflow-y-auto flex-1 p-8 bg-white">
                    {/* Content Sections */}
                    <section id="introduction">
                        <h2 class="mt-6 mb-2 text-2xl font-bold text-gray-800">{ "Introduction" }</h2>
                        <p class="mb-6 text-lg text-gray-700">{ "Welcome to the comprehensive guide to web development. In this course, you will learn the fundamentals of HTML, CSS, and JavaScript, which are essential for creating modern web applications." }</p>
                    </section>
                    {/* Repeat for other sections */}
                </main>
                <SidebarRight />
            </div>
            <Footer />
        </div>
    }
}
