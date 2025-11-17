use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <main class="p-4 md:p-8">
            {children()}
        </main>
    }
}
