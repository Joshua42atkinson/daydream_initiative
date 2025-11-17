use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg home-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12">
                <div class="text-center space-y-4">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-blue-900/50 text-blue-400 text-xs font-bold tracking-widest uppercase">
                        <span class="w-2 h-2 bg-blue-400 rounded-full mr-2 animate-pulse"></span>
                        "Supra-Badge 01"
                    </div>

                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Professional " <span class="text-gradient home-text-gradient">"Foundations"</span>
                    </h1>

                    <p class="text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
                        "The bedrock of the Learning Engineer. Establishing the " <span class="text-cyan-200">"ethical"</span> ", " <span class="text-cyan-200">"theoretical"</span> ", and " <span class="text-cyan-200">"communicative"</span> " standards required to architect effective learning ecosystems."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="glass-card home-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-cyan-400 transition">"ID Professional Communicator"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Mastering the art of clear, concise, and grammatically precise messaging. Soliciting and integrating constructive feedback to refine the design process."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-cyan-500 bg-cyan-900/20 px-2 py-1 rounded">"Constructive Feedback"</span>
                            <span class="text-xs font-mono text-cyan-500 bg-cyan-900/20 px-2 py-1 rounded">"Engagement"</span>
                        </div>
                    </div>

                    <div class="glass-card home-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-blue-400 transition">"Applying ID Research & Theory"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Bridging the gap between academic theory and practical application. Utilizing Systems Thinking (ADDIE, SDT) to break down complex performance problems."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-blue-500 bg-blue-900/20 px-2 py-1 rounded">"Systems Thinking"</span>
                            <span class="text-xs font-mono text-blue-500 bg-blue-900/20 px-2 py-1 rounded">"ID Concepts"</span>
                        </div>
                    </div>

                    <div class="glass-card home-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-purple-400 transition">"ID Knowledge & Skills"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Continuous professional development in a rapidly evolving field. Acquiring and applying new technology skills to stay at the bleeding edge of LDT."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-purple-500 bg-purple-900/20 px-2 py-1 rounded">"Professional Dev"</span>
                            <span class="text-xs font-mono text-purple-500 bg-purple-900/20 px-2 py-1 rounded">"Tech Skills"</span>
                        </div>
                    </div>

                    <div class="glass-card home-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group">
                        <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">"Ethical & Legal Implications"</h3>
                        <p class="text-sm text-slate-400 flex-grow">
                            "Navigating the complex landscape of organizational constraints and professional ethics. Recognizing, respecting, and complying with the laws that govern design."
                        </p>
                        <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                            <span class="text-xs font-mono text-emerald-500 bg-emerald-900/20 px-2 py-1 rounded">"Constraints"</span>
                            <span class="text-xs font-mono text-emerald-500 bg-emerald-900/20 px-2 py-1 rounded">"Code of Ethics"</span>
                        </div>
                    </div>
                </div>

                <div class="text-center pt-4">
                    <p class="text-xs text-slate-500 uppercase tracking-widest">
                        "Select a competency below to view artifacts"
                    </p>
                </div>

                <nav class="flex justify-center space-x-4">
                    <a href="/portfolio" class="text-purple-400 hover:text-purple-300">"Planning & Analysis"</a>
                    <a href="/daydream" class="text-emerald-400 hover:text-emerald-300">"Design & Development"</a>
                    <a href="/meet-the-designer" class="text-rose-400 hover:text-rose-300">"Evaluation & Implementation"</a>
                </nav>
            </div>
        </Layout>
    }
}
