use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn SummativeEval() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-sky-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"Certifying Empathy"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-sky-500/20 text-sky-300 text-xs font-bold uppercase tracking-widest border border-sky-500/30">
                        "Evaluation & Implementation"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-sky-500"></span>
                        "Challenge: Implement Summative Evaluation Plans"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-sky-900/20 border-sky-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-sky-500/20 flex items-center justify-center text-sky-400 mb-4 shadow-lg shadow-sky-900/30 ring-1 ring-sky-500/30">
                                // Icon: User Shield / Verified / Human
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-sky-300/80 uppercase tracking-widest">"Strategy Brief"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Mentor-in-the-Loop Strategy"</h3>
                                <p class="text-sm text-sky-200/70 font-medium">"The TPACK Certification Framework"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-sky-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This strategy brief defines the Summative Evaluation plan for the platform's most critical component: the human mentor. It establishes the 'Daydream Certification,' a mandatory 'Gatekeeper' module that assesses a mentor's Technological, Pedagogical, and Content Knowledge (TPACK). This evaluation ensures that every human 'More Knowledgeable Other' possesses the empathy and skill required to maintain psychological safety before they are allowed to interact with learners."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/1G9eHvMyS-UCUh2N6v5aUaFrMMEdc2M0p2TFedqGfLZE/edit?tab=t.0"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-sky-600 hover:bg-sky-500 text-white text-sm font-semibold transition-all shadow-lg shadow-sky-900/30 hover:shadow-sky-700/50"
                                >
                                    <span>"View Certification Strategy"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-sky-500/50 via-cyan-500/30 to-transparent hidden md:block z-0"></div>

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
                        "In designing the summative evaluation plan for Daydream, I identified a critical risk: an untrained mentor, though well-intentioned, could destroy the 'Psychological Safety' created by the AI. Standard assessments often focus on content knowledge, but in a Vygotskian scaffolding model, the <strong class='text-white'>affective competence</strong> of the mentor is paramount. Therefore, I designed the 'Daydream Certification' as a rigorous summative assessment not for the learner, but for the teacher—validating their readiness to serve as a 'More Knowledgeable Other.'"
                    </p>

                    <p>
                        "I grounded this assessment in the <span class='text-sky-300 font-medium'>TPACK Framework</span> (Technological Pedagogical Content Knowledge). The certification module functions as a 'Gatekeeper' simulation where prospective mentors must navigate narrative scenarios involving at-risk learners. They are evaluated on their ability to distinguish between judgmental 'Feedback' and supportive, Socratic 'Feed-Forward.' This 'dogfooding' approach—using the platform's own narrative engine to assess its users—ensures that the evaluation is ecologically valid, measuring the mentor's actual capacity for empathy and guidance in a simulated environment."
                    </p>

                    <p>
                        "The 'grade' for this summative evaluation is binary and structural: the <strong class='text-white'>'Mentor Role Unlock.'</strong> Only upon successfully demonstrating the required affective and pedagogical skills does the system toggle the <code>is_certified</code> flag in the database, technically enabling the user to receive student connections. This architecture transforms summative evaluation from a post-hoc report into a proactive quality assurance mechanism, ensuring that the human element of the learning loop is as reliable and safe as the code itself."
                    </p>
                </div>
            </div>
        </div>
    }
}
