use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn EngagingPresentation() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-indigo-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Narrative Pitch"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-indigo-500/20 text-indigo-300 text-xs font-bold uppercase tracking-widest border border-indigo-500/30">
                        "Professional Foundations"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-indigo-500"></span>
                        "Challenge: Deliver Engaging Presentations"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-indigo-900/20 border-indigo-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-indigo-500/20 flex items-center justify-center text-indigo-400 mb-4 shadow-lg shadow-indigo-900/30 ring-1 ring-indigo-500/30">
                                // Icon: Presentation / Video / Play
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-indigo-300/80 uppercase tracking-widest">"Tech Demo"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"The Engine of Enjoyment"</h3>
                                <p class="text-sm text-indigo-200/70 font-medium">"Multimedia Presentation on Deep Gamification"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-indigo-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This video presentation translates the complex psychological framework of the Daydream Initiative into a compelling visual narrative. Designed for stakeholders and skeptics, it uses the metaphor of a 'mechanical engine' to explain how the platform solves the 'Motivation Paradox,' effectively selling the pedagogical value of the 'Virtuous Cycle.'"
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://youtu.be/dYxmWd50xgs"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-semibold transition-all shadow-lg shadow-indigo-900/30 hover:shadow-indigo-700/50"
                                >
                                    <span>"Watch Presentation"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-indigo-500/50 via-violet-500/30 to-transparent hidden md:block z-0"></div>

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
                        "To demonstrate the competency of delivering engaging presentations, I produced 'The Engine of Enjoyment,' a multimedia tech demo designed for a skeptical audience of educators and stakeholders. I recognized that the technical terminology of my research ('Self-Determination Theory,' 'Cognitive Load') could alienate non-specialists. Therefore, I employed a <strong class='text-white'>'Narrative Communication Strategy,'</strong> framing the technical project through the lens of a story—the quest to solve the 'Edutainment Gap.' This approach transformed dry data into a compelling problem-solution narrative that hooked the audience emotionally before engaging them intellectually."
                    </p>

                    <p>
                        "I utilized <span class='text-indigo-300 font-medium'>Visual Metaphor</span> as a primary tool for engagement and clarity. Instead of presenting abstract diagrams of feedback loops, I visualized the system as a mechanical 'Engine'. This concrete imagery allowed the audience to grasp the complex interdependence of the 'Virtuous Cycle' (Identity → Mechanics → Gameplay → Reward) intuitively. By synchronizing this visual narrative with a scripted walkthrough of the 'Hero's Journey' pedagogy, I ensured that the presentation communicated the 'privacy-first' architectural message clearly without getting bogged down in code-level details."
                    </p>

                    <p>
                        "The effectiveness of this presentation lies in its ability to model the very principles it advocates. Just as the Daydream platform uses narrative to scaffold learning, this presentation uses narrative to scaffold stakeholder buy-in. It demonstrates the ability to translate complex, high-level systems architecture into accessible, persuasive, and visually engaging messages, proving that I can advocate for educational innovation not just through engineering, but through powerful <span class='text-indigo-300 font-medium'>storytelling</span>."
                    </p>
                </div>
            </div>
        </div>
    }
}
