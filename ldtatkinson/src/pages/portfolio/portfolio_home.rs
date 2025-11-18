use crate::components::layout::Layout;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn PortfolioHome() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg portfolio-aurora"></div>
            <div class="max-w-6xl mx-auto space-y-12 text-center pt-10">
                
                <h1 class="text-5xl font-extrabold text-white tracking-tight">
                    "Competency " <span class="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-600">"Portfolio"</span>
                </h1>
                <p class="text-xl text-slate-300 max-w-2xl mx-auto">
                    "A comprehensive evidence-based record of mastery in Learning Design & Technology. Select a domain below to explore artifacts and reflections."
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mt-12">
                    // Card 1: Foundations
                    <A href="/portfolio/foundations" attr:class="group relative block h-64 rounded-2xl overflow-hidden glass-card hover:border-cyan-400/50 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-br from-cyan-900/40 to-blue-900/40 group-hover:scale-105 transition-transform duration-700"></div>
                        <div class="relative z-10 p-8 h-full flex flex-col justify-end text-left">
                            <span class="text-xs font-mono text-cyan-400 mb-2">"SUPRA-BADGE 01"</span>
                            <h3 class="text-3xl font-bold text-white group-hover:text-cyan-300 transition">"Professional Foundations"</h3>
                        </div>
                    </A>

                    // Card 2: Planning
                    <A href="/portfolio/planning" attr:class="group relative block h-64 rounded-2xl overflow-hidden glass-card hover:border-purple-400/50 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-br from-purple-900/40 to-indigo-900/40 group-hover:scale-105 transition-transform duration-700"></div>
                        <div class="relative z-10 p-8 h-full flex flex-col justify-end text-left">
                            <span class="text-xs font-mono text-purple-400 mb-2">"SUPRA-BADGE 02"</span>
                            <h3 class="text-3xl font-bold text-white group-hover:text-purple-300 transition">"Planning & Analysis"</h3>
                        </div>
                    </A>

                    // Card 3: Design
                    <A href="/portfolio/design" attr:class="group relative block h-64 rounded-2xl overflow-hidden glass-card hover:border-emerald-400/50 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-br from-emerald-900/40 to-green-900/40 group-hover:scale-105 transition-transform duration-700"></div>
                        <div class="relative z-10 p-8 h-full flex flex-col justify-end text-left">
                            <span class="text-xs font-mono text-emerald-400 mb-2">"SUPRA-BADGE 03"</span>
                            <h3 class="text-3xl font-bold text-white group-hover:text-emerald-300 transition">"Design & Development"</h3>
                        </div>
                    </A>

                    // Card 4: Evaluation
                    <A href="/portfolio/evaluation" attr:class="group relative block h-64 rounded-2xl overflow-hidden glass-card hover:border-rose-400/50 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-br from-rose-900/40 to-red-900/40 group-hover:scale-105 transition-transform duration-700"></div>
                        <div class="relative z-10 p-8 h-full flex flex-col justify-end text-left">
                            <span class="text-xs font-mono text-rose-400 mb-2">"SUPRA-BADGE 04"</span>
                            <h3 class="text-3xl font-bold text-white group-hover:text-rose-300 transition">"Evaluation & Implementation"</h3>
                        </div>
                    </A>
                </div>
            </div>
        </Layout>
    }
}
