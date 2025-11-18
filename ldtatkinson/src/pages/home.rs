use crate::components::layout::Layout;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg home-aurora"></div>
            <div class="min-h-[80vh] flex flex-col items-center justify-center text-center px-4">
                
                // Personal Branding Hero
                <div class="inline-flex items-center px-4 py-2 rounded-full glass-panel border-cyan-500/30 text-cyan-400 text-xs font-bold tracking-widest uppercase mb-8 animate-in fade-in slide-in-from-bottom-4 duration-1000">
                    <span class="w-2 h-2 bg-cyan-400 rounded-full mr-3 animate-pulse"></span>
                    "LDT Portfolio // Fall 2025"
                </div>
                
                <h1 class="text-6xl md:text-9xl font-extrabold tracking-tight text-white mb-6 animate-in fade-in zoom-in duration-1000">
                    "Joshua " <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-600">"Atkinson"</span>
                </h1>
                
                <p class="text-xl md:text-3xl text-slate-300 max-w-4xl mx-auto leading-relaxed font-light mb-12 animate-in fade-in slide-in-from-bottom-8 duration-1000 delay-200">
                    "Bridging the gap between " <span class="text-cyan-200 font-semibold">"learning science"</span> " and " <span class="text-blue-200 font-semibold">"technical architecture"</span> "."
                </p>

                // Call to Action
                <div class="flex gap-6 animate-in fade-in slide-in-from-bottom-12 duration-1000 delay-500">
                    <A href="/portfolio" attr:class="px-8 py-4 bg-cyan-600 hover:bg-cyan-500 text-white text-lg font-bold rounded-full shadow-lg shadow-cyan-900/50 transition-all hover:scale-105">
                        "View Competency Portfolio"
                    </A>
                    <A href="/meet-the-designer" attr:class="px-8 py-4 glass-panel border border-white/20 hover:bg-white/10 text-white text-lg font-bold rounded-full transition-all hover:scale-105">
                        "Meet the Designer"
                    </A>
                </div>
            </div>
        </Layout>
    }
}
