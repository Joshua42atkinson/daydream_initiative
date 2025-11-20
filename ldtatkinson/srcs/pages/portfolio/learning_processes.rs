use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn LearningProcesses() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-pink-600 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"Beyond Rote Recall"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-pink-600/20 text-pink-300 text-xs font-bold uppercase tracking-widest border border-pink-600/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-pink-600"></span>
                        "Challenge: Identify Learning Processes to be Measured"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-pink-900/20 border-pink-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-pink-500/20 flex items-center justify-center text-pink-400 mb-4 shadow-lg shadow-pink-900/30 ring-1 ring-pink-500/30">
                                // Icon: Brain / Measurement / Target
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-pink-300/80 uppercase tracking-widest">"Research Report"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"VaaM Assessment Framework"</h3>
                                <p class="text-sm text-pink-200/70 font-medium">"Measuring Transfer & Metacognition"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-pink-600 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This assessment framework identifies 'Conceptual Mastery' and 'Contextual Transfer' as the primary learning outcomes. It rejects standard multiple-choice testing in favor of a dual-layer approach: 'Implicit Assessment' (measuring successful application in gameplay) and 'Conceptual Assessment' (measuring metacognitive depth via AI dialogue)."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/1Nlm2Q5MFzGaa3uL6Xry6gCMrtDjIKw11WIouYPBrIKY/edit?usp=sharing"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-pink-600 hover:bg-pink-500 text-white text-sm font-semibold transition-all shadow-lg shadow-pink-900/30 hover:shadow-pink-700/50"
                                >
                                    <span>"View Assessment Logic"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-pink-500/50 via-rose-500/30 to-transparent hidden md:block z-0"></div>

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
                        "For this project, I identified <strong class='text-white'>metacognition</strong> and <strong class='text-white'>contextual application</strong> as the specific learning processes to be measured, consciously moving the assessment criteria beyond simple 'recall.' Standard multiple-choice assessments often fail to measure the depth of a learner's semantic network; a correct answer may indicate recognition rather than understanding. To address this, I developed a multi-layered assessment framework to measure 'conceptual mastery' across different cognitive depths."
                    </p>

                    <p>
                        "The first layer, <span class='text-pink-300 font-medium'>'Implicit Assessment,'</span> measures the behavioral outcome of the learning process. This is a form of 'stealth assessment' where the gameplay action itself validates the learning: Did the learner successfully use the word to advance the game state? If the learner correctly identifies the 'rune of imploring' to open a door, they have demonstrated a functional understanding of the concept in context. This measures the process of application without interrupting the flow of the learning experience with an extrinsic test."
                    </p>

                    <p>
                        "The second, deeper layer utilizes an AI-driven Socratic Guide to measure the process of <span class='text-pink-300 font-medium'>metacognition</span>. Rather than asking 'what' a word means, the system prompts the learner to explain 'why' they chose a specific word in a dialogue tree (e.g., 'You chose to implore... what did you feel that made you choose that?'). This innovative assessment instrument probes the learner's reasoning and intentionality, providing a qualitative measure of their conceptual understanding. By identifying these specific, high-level processes, the assessment design ensures that we are measuring the quality of the learner's mental model, not just the quantity of retained facts."
                    </p>
                </div>
            </div>
        </div>
    }
}
