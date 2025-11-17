use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn MeetTheDesigner() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg meet-the-designer-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12">
                <div class="text-center space-y-4">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-rose-900/50 text-rose-400 text-xs font-bold tracking-widest uppercase">
                        <span class="w-2 h-2 bg-rose-400 rounded-full mr-2 animate-pulse"></span>
                        "Supra-Badge 04"
                    </div>
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Evaluation & " <span class="text-gradient meet-the-designer-text-gradient">"Implementation"</span>
                    </h1>
                    <p class="text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
                        "The validation phase. Measuring efficacy through formative and summative cycles, and managing the diffusion of innovation into the real world."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="glass-card meet-the-designer-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-rose-400 transition">"Evaluate Interventions"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Designing and implementing Formative plans (iterative improvement) and Summative plans (final efficacy) to validate the design."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-rose-400 bg-rose-900/20 px-2 py-1 rounded">"Formative"</span>
                            <span class="text-xs font-mono text-rose-400 bg-rose-900/20 px-2 py-1 rounded">"Summative"</span>
                        </div>
                    </div>

                    <div class="glass-card meet-the-designer-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-rose-400 transition">"Dissemination & Diffusion"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Planning the rollout. Strategies for communicating the vision (dissemination) and achieving organizational adoption (diffusion)."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-rose-400 bg-rose-900/20 px-2 py-1 rounded">"Change Mgmt"</span>
                            <span class="text-xs font-mono text-rose-400 bg-rose-900/20 px-2 py-1 rounded">"Adoption"</span>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
