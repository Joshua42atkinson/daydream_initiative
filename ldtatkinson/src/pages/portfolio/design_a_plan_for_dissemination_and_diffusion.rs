use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn DesignAPlanForDisseminationAndDiffusion() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-lime-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Open Source Strategy"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-lime-500/20 text-lime-300 text-xs font-bold uppercase tracking-widest border border-lime-500/30">
                        "Evaluation & Implementation"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-lime-500"></span>
                        "Challenge: Create a Plan for Dissemination & Diffusion"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-lime-900/20 border-lime-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-lime-500/20 flex items-center justify-center text-lime-400 mb-4 shadow-lg shadow-lime-900/30 ring-1 ring-lime-500/30">
                                // Icon: Broadcast / Share / Globe
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-lime-300/80 uppercase tracking-widest">"Strategic Roadmap"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Daydream 3.0 Implementation Plan"</h3>
                                <p class="text-sm text-lime-200/70 font-medium">"University Partnership & Non-Profit Operational Model"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-lime-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This strategic roadmap outlines the phased dissemination of the Daydream platform, moving from a 'Technical Spike' to a public 'Open Source' launch. It leverages a Non-Profit operational model and a formal University Partnership to ensure the sustainable diffusion of the tool within the educational community, bypassing traditional commercial barriers."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/13J43zG_4yX_96r7W8V9L_X2J3k4w5y6/edit?usp=sharing"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-lime-600 hover:bg-lime-500 text-white text-sm font-semibold transition-all shadow-lg shadow-lime-900/30 hover:shadow-lime-700/50"
                                >
                                    <span>"View Implementation Plan"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-lime-500/50 via-green-500/30 to-transparent hidden md:block z-0"></div>

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
                        "I created a comprehensive dissemination plan centered on a <strong class='text-white'>'Phased Rollout Strategy'</strong> designed to de-risk the technology before scaling. The roadmap begins with a 'Technical Spike' (Phase 0) to validate the Rust/Bevy architecture, moves to an 'Authoring Core' MVP (Phase 1) for internal testing, and culminates in a 'Public Launch' (Phase 4). This systematic approach ensures that the diffusion of the innovation is driven by stability and user trust, rather than a premature marketing push."
                    </p>

                    <p>
                        "To facilitate widespread diffusion, I architected the project's operational model around the <strong class='text-white'>'Gift Ethos'</strong> of the Open Source community. By mandating the <span class='text-lime-300 font-medium'>GNU General Public License (GPLv3)</span>, the plan ensures that the platform remains a permanent public good. This license prevents the 'capture' of the code by for-profit entities, legally requiring any improvements to be shared back to the community. This strategy effectively removes cost barriers for educators, creating a viral mechanism for adoption."
                    </p>

                    <p>
                        "Finally, the plan strategically leverages institutional partnership as a diffusion engine. By aligning the project with <span class='text-lime-300 font-medium'>Purdue University's research mission</span>, I transformed the dissemination strategy from an individual effort into an institutional mandate. The proposal for a 'University Partnership' allows the project to access the university's existing distribution channels and credibility, ensuring that the tool reaches its target audience—Instructional Designers and Learning Scientists—with the endorsement of a Tier 1 research institution."
                    </p>
                </div>
            </div>
        </div>
    }
}
