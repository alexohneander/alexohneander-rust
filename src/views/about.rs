use leptos::*;
use leptos_meta::*;

use crate::components::navigation::Navigation;

/// Renders the home page of your application.
#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Alex Wellnitz DevOps/Software Engineer - About Me"/>

        <Navigation/>
        <h1>"Welcome to Leptos!"</h1>
    }
}