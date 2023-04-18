use glob::glob;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Metadata {
    title: String,
    description: String,
    tags: Vec<String>,
    slug: String,
    created_at: String,
}

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    let matter = Matter::<YAML>::new();
    let mut frontmatters = Vec::new();

    for post in glob("posts/**/*.md").expect("Failed to read glob") {
        match post {
            Ok(path) => {
                let md_contents = fs::read_to_string(path).expect("Cannot read markdown file");
                let frontmatter = matter.parse_with_struct::<Metadata>(&md_contents).unwrap();
                frontmatters.push(frontmatter.data)
            }
            Err(e) => println!("{:?}", e),
        }
    }

    view! { cx,
        <For
            each=move || frontmatters.clone()
            key=|frontmatter| frontmatter.slug.clone()
            view=move |cx, frontmatter: Metadata| {
                    view! { cx,
                        <Post frontmatter />
                    }
                }
        />
    }
}

#[component]
pub fn Post(
    cx: Scope,
    #[prop(optional)] slug: String,
    #[prop(optional)] frontmatter: Metadata,
) -> impl IntoView {
    let params = use_params_map(cx);
    // handle if frontmatter/slug is the default value/empty later
    // fetch blog based on slug here, from markdown frontmatter
    view! { cx,
        <h1>{frontmatter.title}</h1>
    }
}
