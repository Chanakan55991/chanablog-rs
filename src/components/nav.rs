use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="header p-4 backdrop-blur fixed w-full z-20">
            <div class="mx-auto max-w-3xl">
                <nav class="flex items-center gap-3 text-base">
                    <a href="/">
                        <h2 class="font-semibold tracking-tighter font-mplus text-lg">
                            "Chanakan"
                        </h2>
                    </a>
                    <div class="items-center gap-6 hidden md:flex font-mplus">
                        <a href="/posts">"Posts"</a>
                    </div>
                    <div class="flex-1"></div>
                    <a>"dipshit"</a>
                </nav>
            </div>
        </header>
    }
}
