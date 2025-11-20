use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn KeyConcepts() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-blue-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Persona Engine"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-blue-500/20 text-blue-300 text-xs font-bold uppercase tracking-widest border border-blue-500/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
                        "Challenge: Apply Interaction Design & Usability"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-blue-900/20 border-blue-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-blue-500/20 flex items-center justify-center text-blue-400 mb-4 shadow-lg shadow-blue-900/30 ring-1 ring-blue-500/30">
                                // Icon: Fingerprint / Identity / Interaction
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-blue-300/80 uppercase tracking-widest">"Research Outline"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Jungian Archetype Integration"</h3>
                                <p class="text-sm text-blue-200/70 font-medium">"UI/UX Specification for Character Creation"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-blue-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This design specification outlines the 'Persona Engine,' a character creation interface that rejects standard 'class pickers' in favor of a narrative-based diagnostic quiz. By integrating Jungian Archetypes with LitRPG mechanics, the design utilizes Cognitive Load Theory to function as an 'advance organizer,' simplifying complex decision-making while maximizing learner autonomy."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/15_785786555555555555" // Placeholder - Update with actual link if available
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold transition-all shadow-lg shadow-blue-900/30 hover:shadow-blue-700/50"
                                >
                                    <span>"View Spec"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-blue-500/50 via-cyan-500/30 to-transparent hidden md:block z-0"></div>

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
                        "In designing the user interface for character creation, I applied interaction design principles grounded in <strong class='text-white'>Cognitive Load Theory (CLT)</strong> to solve a common usability hurdle: 'Choice Paralysis.' Standard RPG interfaces often present users with a 'Class Picker' list (e.g., 'Select: Warrior, Mage, Rogue'), which imposes high extraneous cognitive load by forcing the learner to evaluate abstract variables without context. I rejected this 'flashcard' model in favor of a <span class='text-blue-300 font-medium'>'Situation-Based Quiz,'</span> a narrative interaction design that scaffolds the decision-making process."
                    </p>

                    <p>
                        "I operationalized this design by integrating <strong class='text-white'>Jungian Archetypes</strong> (e.g., The Sage, The Hero) as an 'advance organizer' for the interface. Instead of asking learners to select stats directly, the system presents narrative dilemmas (e.g., 'You see a bully... do you confront or observe?'). This interaction maps the learner's intuitive moral choices to complex backend game mechanics ('LitRPG Stats'). This approach ensures that the interface is not merely functional but 'psychologically ergonomic,' bridging the gap between the learner's identity and the system's variables without overwhelming them."
                    </p>

                    <p>
                        "Furthermore, this interaction design directly supports the project's pedagogical goal of <span class='text-blue-300 font-medium'>Self-Determination</span>. By framing the initial interaction as a 'Reflection Quest' rather than a setup menu, the system immediately validates the learner's autonomy and fosters 'Relatedness.' The 'Reveal' moment—where the system mirrors the learner's choices back to them ('Your answers reveal the heart of a Sage')—transforms a standard configuration step into a meaningful psychological anchor. This demonstrates how rigorous usability principles can be applied to transform administrative tasks into core learning experiences."
                    </p>
                </div>
            </div>
        </div>
    }
}
