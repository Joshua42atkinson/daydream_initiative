use leptos::*;

#[component]
pub fn GlassPanel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("glass-panel {}", class)>
            {children()}
        </div>
    }
}
