use leptos::*;

use crate::components::navigation::Navigation;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <Navigation/>
        <h1>"Moin 👋"</h1>
        <p>"I’m Alex, DevOps/Network architect and software developer. I’m currently working as a DevOps Engineer at Materna. I help developers build a faster web and teach about web development, Kubernetes, network security and more."</p>
    }
}