use leptos::*;

use crate::components::navigation::Navigation;

/// Renders the home page of your application.
#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {

    view! { cx,
        <Navigation/>
        <h1>"Welcome to Leptos!"</h1>
    }
}