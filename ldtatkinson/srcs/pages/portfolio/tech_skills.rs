use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn TechSkills() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-cyan-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Learning Systems Architect"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-cyan-500/20 text-cyan-300 text-xs font-bold uppercase tracking-widest border border-cyan-500/30">
                        "ID Knowledge, Skills, & Attitudes"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-cyan-500"></span>
                        "Challenge: Acquire & Apply New Technology Skills"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-cyan-900/20 border-cyan-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-cyan-500/20 flex items-center justify-center text-cyan-400 mb-4 shadow-lg shadow-cyan-900/30 ring-1 ring-cyan-500/30">
                                // Icon: Code / Terminal / Architecture
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-cyan-300/80 uppercase tracking-widest">"Source Code"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Daydream Core Engine"</h3>
                                <p class="text-sm text-cyan-200/70 font-medium">"Rust + WebAssembly (WASM) Architecture"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-cyan-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact is the codebase itself: a full-stack web application built with Rust (Axum/Leptos) and compiled to WebAssembly. It demonstrates the acquisition of 'bleeding-edge' systems engineering skills to solve the 'Black Box' problem in EdTech, creating a local-first architecture that guarantees learner privacy by design."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://github.com/Joshua42atkinson/Day_Dream"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white text-sm font-semibold transition-all shadow-lg shadow-cyan-900/30 hover:shadow-cyan-700/50"
                                >
                                    <span>"View Repository"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-cyan-500/50 via-blue-500/30 to-transparent hidden md:block z-0"></div>

            // --- Reflection Section ---
            <div class="relative z-10 pl-0 md:pl-16">
                <div class="flex items-center gap-4 mb-6">
                    <div class="w-10 h-10 rounded-full bg-slate-800 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-lg">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                    </div>
                    <h3 class="text-2xl font-bold text-cyan-100">"Competency Reflection"</h3>
                </div>

                <div class="prose prose-invert prose-lg max-w-none text-slate-300 leading-relaxed space-y-6">
                    <p>
                        "To realize the vision of a 'privacy-first' learning environment, I engaged in the rigorous acquisition and application of the <strong class='text-white'>Rust programming language</strong> and the <span class='text-cyan-300 font-medium'>Leptos (WebAssembly)</span> framework. Unlike standard web technologies which often rely on server-side processing and opaque dependencies, Rust offers a strict, memory-safe environment that allows for a 'Local-First' architecture. I applied this skill to engineer a system where the entire application—including the narrative logic and the 'AI as a Mirror' components—compiles to WebAssembly, ensuring the code runs exclusively on the client’s device."
                    </p>

                    <p>
                        "This technical acquisition was driven by a philosophy of <strong class='text-white'>'Humanistic Engineering'</strong>—using the objectivity of code to scaffold the subjectivity of the learner. By mastering Rust’s strict type system, I was able to solve the 'Black Box' problem inherent in many educational AI tools. Instead of relying on third-party APIs that introduce latency and potential data leakage, I built a transparent, predictable 'sandbox.' This creates a learning environment with objective 'physics,' where the rules are consistent and fair, allowing the learner to experiment without fear of surveillance."
                    </p>

                    <p>
                        "The application of these advanced engineering skills demonstrates a significant shift in the role of the Instructional Designer: from a consumer of tools to an <span class='text-cyan-300 font-medium'>Architect of Environments</span>. By building the platform from the metal up, I ensured that the 'structural psychological development' of the learner remains a private process. This proves that high-level technical competency is not merely about building faster apps, but about having the agency to build ethical ones—systems that foster volition rather than compulsion by placing the full power of the computing environment directly into the hands of the learner."
                    </p>
                </div>
            </div>
        </div>
    }
}
