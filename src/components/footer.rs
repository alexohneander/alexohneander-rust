use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <a href="/">"Home"</a>
            <a href="/about">"About"</a>
            <a href="/blog">"Blog"</a>
        </footer>
    }
}