use cfg_if::cfg_if;
use leptos::{component, view, IntoView, Scope};
use leptos_meta::*;
use leptos_router::*;

// mod api; // could possibly be use for full stack applciation
pub mod error_template;
pub mod fileserv;
pub mod handlers;

mod routes;
use routes::main::*;
use routes::posts::*;

mod components;
use components::masthead::*;
use components::nav::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <>
             <script src="https://unpkg.com/htmx.org@1.9.0"></script>
            <Stylesheet id="leptos" href="/pkg/chanablog-rs.css" />
            <Meta name="description" content="My own blog made with Rust and Leptos" />
            <Router>
                <Nav />
                <Masthead />
                <main>
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <Main/> }/>
                        <Route path="posts" view=|cx| view! { cx, <Posts/> }/>
                    </Routes>
                </main>
            </Router>
        </>
    }
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
