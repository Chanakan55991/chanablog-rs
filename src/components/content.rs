use leptos::{component, view, Children, IntoAttribute, IntoView, Scope};

#[component]
pub fn Content(
    cx: Scope,
    #[prop(optional, into)] class_name: String,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <article class=format!("px-8 mx-auto max-w-3xl {}", class_name)>{children(cx)}</article>
    }
}
