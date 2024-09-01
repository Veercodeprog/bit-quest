use crate::error_template::{AppError, ErrorTemplate};
use crate::page::about::AboutPage;
use crate::page::home::HomePage;
use crate::page::web3::Web3Notes;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/fuel-dao-leptos-ssr.css"/>
        <Title text="Welcome to Leptos"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                // <nav>
                //     <a href="/" class="mr-4">"Home"</a>
                //     <a href="/about" class="mr-4">"About"</a>
                // </nav>
              <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="about" view=AboutPage/>
        <Route path="web3" view=Web3Notes/>
                </Routes>
            </main>
        </Router>
    }
}
