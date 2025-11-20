use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn InstructionalStrategies() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-red-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Pedagogical Engine"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-red-500/20 text-red-300 text-xs font-bold uppercase tracking-widest border border-red-500/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-red-500"></span>
                        "Challenge: Identify Instructional Strategies"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-red-900/20 border-red-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-red-500/20 flex items-center justify-center text-red-400 mb-4 shadow-lg shadow-red-900/30 ring-1 ring-red-500/30">
                                // Icon: Chess Piece / Strategy / Gears
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-red-300/80 uppercase tracking-widest">"Research Report"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"VaaM Research Report"</h3>
                                <p class="text-sm text-red-200/70 font-medium">"Vocabulary-as-a-Mechanic Model"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-red-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact defines the instructional strategy for vocabulary acquisition. It rejects 'flashcard' rote memorization in favor of a 'Situated Pedagogy' that treats words as functional tools. The model integrates Dual Coding Theory (multimedia acquisition) with a game-based application loop to ensure durable, transferable conceptual mastery."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/1Nlm2Q5MFzGaa3uL6Xry6gCMrtDjIKw11WIouYPBrIKY/edit?usp=sharing"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-red-600 hover:bg-red-500 text-white text-sm font-semibold transition-all shadow-lg shadow-red-900/30 hover:shadow-red-700/50"
                                >
                                    <span>"View Research Report"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-red-500/50 via-orange-500/30 to-transparent hidden md:block z-0"></div>

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
                        "To address the critical learning goal of 'durable vocabulary transfer,' I identified <strong class='text-white'>Situated Cognition</strong> (Lave & Wenger) as the primary instructional strategy for this intervention. Traditional vocabulary instruction often relies on rote memorization, which frequently results in a high rate of recall but a failure of transfer. To align the instructional strategy with the outcome of <span class='text-red-300 font-medium'>conceptual mastery</span>, I designed the 'Vocabulary-as-a-Mechanic' (VaaM) model. This approach fundamentally shifts the pedagogical framework from 'acquisition of facts' to 'participation in a context,' treating words as functional tools required to solve narrative puzzles."
                    </p>

                    <p>
                        "I further integrated <strong class='text-white'>Dual Coding Theory</strong> (Paivio) as a supporting strategy to manage cognitive load. By pairing the auditory pronunciation of a word with a generative AI visual reference (e.g., an image of a 'precarious' bridge), the design scaffolds the intrinsic load of acquisition. This multimedia approach utilizes separate cognitive channels to strengthen the memory trace without overwhelming the learner. This strategic alignment ensures that the game mechanics are not merely decorative but serve as active carriers of the instructional method."
                    </p>

                    <p>
                        "Finally, I implemented a multi-layered assessment strategy to validate these instructional interventions. Moving beyond simple 'Explicit Assessment' (recall), the system utilizes <span class='text-red-300 font-medium'>'Implicit Assessment'</span>—tracking whether a learner successfully uses a word to advance the game state. Furthermore, I integrated a 'Conceptual Assessment' layer using the 'AI as a Mirror' feature, which prompts metacognitive reflection on <em class='text-white'>why</em> a specific word was chosen. This prompts the learner to elaborate on their reasoning, maximizing germane cognitive load and ensuring deep schema construction."
                    </p>
                </div>
            </div>
        </div>
    }
}
