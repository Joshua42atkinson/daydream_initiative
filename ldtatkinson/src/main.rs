use leptos::prelude::*;
use ldtatkinson::app::App;
use wasm_bindgen::prelude::*;

// --- DAYDREAM CORE: REFLECTION INTERFACE ---
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn internal_monologue(s: &str);
}

fn trace_thought(thought: &str) {
    let timestamp = "Daydream System"; 
    unsafe {
        internal_monologue(&format!("💭 [{}] Reflection: {}", timestamp, thought));
    }
}
// -------------------------------------------

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    trace_thought("Consciousness initialized. Psychological Safety protocols active.");

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
