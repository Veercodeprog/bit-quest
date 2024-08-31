pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

// Add the pages module
pub mod page {
    pub mod about;
    pub mod home;
}

// Expose the HomePage and AboutPage components
pub use page::about::AboutPage;
pub use page::home::HomePage;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
