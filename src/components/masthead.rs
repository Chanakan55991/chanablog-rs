use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Masthead(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="masthead relative mb-6 h-80 flex justify-center items-center">
            <div class="absolute w-full h-full overflow-hidden">
                <img
                    class="absolute h-auto left-1/2 top-1/2 min-w-full min-h-full object-cover -translate-x-1/2 -translate-y-1/2 opacity-30"
                    src="/masthead.jpg"
                    alt="masthead img"
                />
            </div>
            <div class="z-10 text-center px-8 drop-shadow-lg shadow-black">
                <div class="uppercase text-sm mb-4">"Welcome to"</div>
                <div class="text-4xl font-mplus font-medium">
                    "Chanakan's "<span class="text-orange-500">"Blog"</span>
                </div>
            </div>
        </section>
    }
}
