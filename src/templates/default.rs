use leptos::*;

use crate::components::navigation::Navigation;
use crate::components::responsive_image::ResponsiveImage;
use crate::components::footer::Footer;

/// Renders the home page of your application.
#[component]
pub fn DefaultTemplate(cx: Scope) -> impl IntoView {
    view! { cx,
        <Navigation/>

        <div id="top" class="page" role="document">
            <main role="main">
                <section id="text">
                    <h1>"Moin 👋"</h1>

                    <ResponsiveImage
                        height="50px".to_string()
                        width="50px".to_string()
                    />
                    <p>"I’m Alex, DevOps/Network architect and software developer. I’m currently working as a DevOps Engineer at Materna. I help developers build a faster web and teach about web development, Kubernetes, network security and more."</p>
                    <kbd>$ ls -la</kbd>
                </section>
            </main>
        </div>

        <Footer/>
    }
}
