use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn InstructionalSequencing() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-yellow-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Narrative Syllabus"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-yellow-500/20 text-yellow-300 text-xs font-bold uppercase tracking-widest border border-yellow-500/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-yellow-500"></span>
                        "Challenge: Identify and Sequence Instructional Goals"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-yellow-900/20 border-yellow-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-yellow-500/20 flex items-center justify-center text-yellow-400 mb-4 shadow-lg shadow-yellow-900/30 ring-1 ring-yellow-500/30">
                                // Icon: Roadmap / Sequence / Steps
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-yellow-300/80 uppercase tracking-widest">"Source Data"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Quest Engine Data Structure"</h3>
                                <p class="text-sm text-yellow-200/70 font-medium">"quests.json: Algorithmic Pedagogical Sequencing"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-yellow-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact, the raw 'quests.json' data file, demonstrates the identification and sequencing of instructional goals within the Daydream engine. It maps the 'Hero's Journey' narrative arc directly to specific performance outcomes (e.g., 'STEP_01_ANALYZE_FOUNTAIN'), creating a dependency graph where a learner must demonstrate one competency before mechanically unlocking the next."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://github.com/Joshua42atkinson/Day_Dream/blob/main/common/src/quests.json"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-yellow-600 hover:bg-yellow-500 text-white text-sm font-semibold transition-all shadow-lg shadow-yellow-900/30 hover:shadow-yellow-700/50"
                                >
                                    <span>"View Quest Logic"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-yellow-500/50 via-amber-500/30 to-transparent hidden md:block z-0"></div>

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
                        "In designing the learning path for the Daydream platform, I identified the instructional goals by decomposing the complex competency of 'Metacognitive Reflection' into discrete, actionable behaviors. Using <strong class='text-white'>Bloom’s Taxonomy</strong> as a guide, I sequenced these goals from lower-order thinking skills (Observation/Recall) to higher-order skills (Analysis/Synthesis). For example, in the 'Faulty Fountain' quest, the learner must first 'Analyze' the disorder (Step 1) before they can 'Synthesize' a repair plan (Step 4). This sequencing ensures that the learner possesses the prerequisite knowledge before attempting the terminal performance."
                    </p>

                    <p>
                        "I operationalized this sequence through the <span class='text-yellow-300 font-medium'>'Quest Engine'</span> architecture. Instead of a linear syllabus, I engineered a 'Dependency Graph' where instructional goals are represented as 'Quest Steps' (e.g., <code>STEP_02_IDENTIFY_PARTS</code>). The code strictly enforces this sequence; the 'Repair' interaction is mechanically locked until the 'Analyze' flag is set to true. This transforms the instructional sequence from a passive suggestion into an active rule of the environment, ensuring that 'skipping ahead'—and thus cognitive overload—is impossible."
                    </p>

                    <p>
                        "This approach to sequencing aligns with the project's 'Hero's Journey' narrative framework. By mapping instructional goals to plot points (e.g., The Call to Adventure = Problem Identification), I ensured that the <span class='text-yellow-300 font-medium'>pedagogical progression</span> mirrors the <span class='text-yellow-300 font-medium'>emotional progression</span> of the story. This 'Narrative-Pedagogical Alignment' reduces the friction often found in gamified learning, where the game and the learning feel disconnected. Here, the act of learning is the mechanism for advancing the story, creating a seamless and motivating user experience."
                    </p>
                </div>
            </div>
        </div>
    }
}
