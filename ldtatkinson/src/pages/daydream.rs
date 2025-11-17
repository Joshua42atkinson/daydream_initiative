use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Daydream() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg daydream-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12">
                <div class="text-center space-y-4">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-emerald-900/50 text-emerald-400 text-xs font-bold tracking-widest uppercase">
                        <span class="w-2 h-2 bg-emerald-400 rounded-full mr-2 animate-pulse"></span>
                        "Supra-Badge 03"
                    </div>
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Design & " <span class="text-gradient daydream-text-gradient">"Development"</span>
                    </h1>
                    <p class="text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
                        "The build phase. Applying systematic models (ADDIE) to create aligned interventions, robust assessments, and engaging instructional materials."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="glass-card daydream-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">"ID Process & Systematic Design"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Selecting and modifying instructional design models (ADDIE, Dick & Carey) to fit the unique parameters of the project."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"ADDIE"</span>
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Systematic"</span>
                        </div>
                    </div>

                    <div class="glass-card daydream-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">"Develop Materials"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Creating or modifying print, audiovisual, and digital materials that align with instructional strategies and learning objectives."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Multimedia"</span>
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Content Creation"</span>
                        </div>
                    </div>

                    <div class="glass-card daydream-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">"Instructional Interventions"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Designing specific interventions that replicate the cognitive processes required for learning (sequencing, scaffolding, practice)."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Strategies"</span>
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Scaffolding"</span>
                        </div>
                    </div>

                     <div class="glass-card daydream-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">"Learning Assessment"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Ensuring constructive alignment between instructional goals, strategies, and the assessments used to measure mastery."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Alignment"</span>
                            <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">"Measurement"</span>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
