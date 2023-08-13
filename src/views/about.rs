use leptos::*;
use leptos_meta::*;

use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

/// Renders the home page of your application.
#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="About Me - Alex Wellnitz DevOps/Software Engineer"/>

        <Navigation/>
        
        <div id="top" class="page" role="document">
            <h1>"Welcome to Leptos!"</h1>
        </div>

        <Footer/>
    }
}