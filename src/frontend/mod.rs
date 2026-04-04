use leptos::prelude::*;

#[derive(Clone)]
pub struct NavItem {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub fn render_dashboard() -> String {
    let nav_items = vec![
        NavItem {
            id: "courses",
            title: "Courses",
            description: "Create and manage class demand inputs.",
        },
        NavItem {
            id: "instructors",
            title: "Instructors",
            description: "Maintain workload limits and assignment constraints.",
        },
        NavItem {
            id: "rooms",
            title: "Rooms",
            description: "Track capacities and room availability.",
        },
        NavItem {
            id: "generation",
            title: "Schedule Generation",
            description: "Run GA iterations and compare score history.",
        },
    ];

    let content = leptos::ssr::render_to_string(move || {
        view! {
            <main class="layout">
                <header class="hero">
                    <p class="eyebrow">"ClassGA Planner"</p>
                    <h1>"Leptos Admin Console"</h1>
                    <p>
                        "A modular UI shell ready for feature modules, API adapters, and role-based views."
                    </p>
                </header>

                <section class="grid">
                    <For
                        each=move || nav_items.clone().into_iter()
                        key=|item| item.id
                        children=move |item| {
                            view! {
                                <article class="card" id=item.id>
                                    <h2>{item.title}</h2>
                                    <p>{item.description}</p>
                                    <button type="button">"Open module"</button>
                                </article>
                            }
                        }
                    />
                </section>

                <section class="roadmap">
                    <h2>"Scalability foundations"</h2>
                    <ul>
                        <li>"Feature-sliced component boundaries for each scheduling domain."</li>
                        <li>"Dedicated typed API clients to isolate transport and auth concerns."</li>
                        <li>"Server-side rendering today, with hydration-ready component structure."</li>
                    </ul>
                </section>
            </main>
        }
    });

    format!(
        r#"<!doctype html>
<html lang=\"en\">
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<title>ClassGA Console</title>
<style>
:root {{ font-family: Inter, system-ui, sans-serif; color-scheme: light dark; }}
body {{ margin: 0; background: #0b1020; color: #e2e8f0; }}
.layout {{ max-width: 1100px; margin: 0 auto; padding: 2rem 1rem 4rem; }}
.hero {{ margin-bottom: 1.75rem; }}
.eyebrow {{ text-transform: uppercase; letter-spacing: 0.08em; color: #93c5fd; font-size: 0.8rem; }}
.grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; }}
.card {{ border: 1px solid #334155; border-radius: 0.75rem; padding: 1rem; background: #131a2b; }}
button {{ border-radius: 0.5rem; border: none; padding: 0.6rem 0.8rem; background: #3b82f6; color: white; cursor: pointer; }}
.roadmap {{ margin-top: 1.75rem; border-left: 4px solid #3b82f6; padding-left: 1rem; }}
</style>
</head>
<body>{content}</body>
</html>"#
    )
}
