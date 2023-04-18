use crate::components::content::*;
use crate::components::masthead::*;
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    view! { cx,
        <Masthead />
        <Content>
            <h1 class="text-xl">"placeholder"</h1>
        </Content>
    }
}
