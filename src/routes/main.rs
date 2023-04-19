use crate::components::content::*;
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    view! { cx,
        <Content>
            <h1 class="text-xl">"placeholder"</h1>
        </Content>
    }
}
