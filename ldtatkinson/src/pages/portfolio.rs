use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg portfolio-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12">
                <div class="text-center space-y-4">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-purple-900/50 text-purple-400 text-xs font-bold tracking-widest uppercase">
                        <span class="w-2 h-2 bg-purple-400 rounded-full mr-2 animate-pulse"></span>
                        "Supra-Badge 02"
                    </div>
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Planning & " <span class="text-gradient portfolio-text-gradient">"Analysis"</span>
                    </h1>
                    <p class="text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
                        "The diagnostic phase. Identifying performance gaps, understanding learner contexts, and selecting the right technologies before design begins."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="glass-card portfolio-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-purple-400 transition">"Gap Analysis"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Defining the crucial delta between current performance and desired outcomes. Identifying root causes to propose targeted instructional solutions."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Root Cause"</span>
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Needs Assessment"</span>
                        </div>
                    </div>

                    <div class="glass-card portfolio-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-purple-400 transition">"Target Population & Environment"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Analyzing learner characteristics (entry behaviors, attitudes) and the environmental context to ensure design relevance and feasibility."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Context Analysis"</span>
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Learner Profile"</span>
                        </div>
                    </div>

                    <div class="glass-card portfolio-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-purple-400 transition">"Analysis Techniques"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Validating content accuracy through rigorous task analysis and subject matter expert (SME) collaboration."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Task Analysis"</span>
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Validation"</span>
                        </div>
                    </div>

                     <div class="glass-card portfolio-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-purple-400 transition">"Analyze Technologies"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Evaluating emerging and existing technologies to determine their affordances, limitations, and suitability for the learning goals."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Emerging Tech"</span>
                            <span class="text-xs font-mono text-purple-400 bg-purple-900/20 px-2 py-1 rounded">"Tool Selection"</span>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
