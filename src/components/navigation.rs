use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <a href="/">"Home"</a>
            <a href="/about">"About"</a>
            <a href="/blog">"Blog"</a>
        </nav>
    }
}