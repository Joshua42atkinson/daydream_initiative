use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            // --- GLOBAL NAVIGATION BAR ---
            <nav class="sticky top-0 z-50 w-full glass-panel border-b border-slate-700/30 backdrop-blur-md">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-16">
                        // Logo / Home Link - NOW FUNCTIONAL
                        <div class="flex-shrink-0">
                            <A href="/" attr:class="group flex items-center gap-2 text-xl font-bold text-white hover:text-cyan-400 transition tracking-tight cursor-pointer select-none">
                                <div class="w-8 h-8 bg-gradient-to-br from-cyan-500 to-blue-600 rounded-lg flex items-center justify-center text-white font-mono text-sm shadow-lg group-hover:shadow-cyan-500/50 transition-all">"LA"</div>
                                <span>"LDT " <span class="text-cyan-400 group-hover:text-white transition">"Atkinson"</span></span>
                            </A>
                        </div>
                        
                        // Desktop Menu
                        <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-8">
                                <A href="/portfolio" attr:class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition">"Portfolio"</A>
                                <A href="/daydream" attr:class="text-gray-300 hover:text-emerald-400 px-3 py-2 rounded-md text-sm font-medium transition">"Daydream"</A>
                                <A href="/meet-the-designer" attr:class="text-gray-300 hover:text-rose-400 px-3 py-2 rounded-md text-sm font-medium transition">"About Me"</A>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            // --- PAGE CONTENT ---
            <main class="flex-grow p-4 md:p-8">
                {children()}
            </main>
            
            // --- FOOTER ---
            <footer class="p-6 text-center text-slate-600 text-sm mt-auto">
                "Designed with Rust/Leptos • " <a href="https://github.com/joshua42atkinson/daydream_initiative" class="hover:text-cyan-500">"GitHub"</a>
            </footer>
        </div>
    }
}
