use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="p-20 bg-gray-100 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-blue-600 font-bold mb-4">"About Us"</h1>
            <p class="text-gray-700">"This is the about page of your Leptos application."</p>
        </div>
    }
}
