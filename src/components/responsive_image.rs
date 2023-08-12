use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn ResponsiveImage(
    cx: Scope, 

    #[prop(optional)]
    height: String, 
    #[prop(optional)]
    width: String
) -> impl IntoView {
    view! { cx,
        <img src="assets/img/logo.png" width=width height=height />
    }
}
