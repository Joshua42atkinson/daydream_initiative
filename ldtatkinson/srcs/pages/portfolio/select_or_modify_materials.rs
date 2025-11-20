use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn SelectOrModifyMaterials() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-amber-600 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"Templates for Imagination"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-amber-600/20 text-amber-300 text-xs font-bold uppercase tracking-widest border border-amber-600/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-amber-600"></span>
                        "Challenge: Select or Modify Instructional Materials"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-amber-900/20 border-amber-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-amber-500/20 flex items-center justify-center text-amber-400 mb-4 shadow-lg shadow-amber-900/30 ring-1 ring-amber-500/30">
                                // Icon: Edit / Pencil / JSON
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-amber-300/80 uppercase tracking-widest">"Data Structure"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Character Archetype Templates"</h3>
                                <p class="text-sm text-amber-200/70 font-medium">"characters.json: Digitizing Jungian Psychology"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-amber-600 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact demonstrates the selection and modification of existing psychological frameworks for instructional use. It takes the abstract concept of 'Jungian Archetypes' (selected material) and modifies them into a concrete JSON data structure (characters.json). This transformation allows abstract concepts like 'The Sage' or 'The Hero' to function as mechanically enforceable variables within the Daydream learning engine."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://github.com/Joshua42atkinson/Day_Dream/blob/main/common/src/characters.json"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold transition-all shadow-lg shadow-amber-900/30 hover:shadow-amber-700/50"
                                >
                                    <span>"View JSON Source"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-amber-500/50 via-yellow-500/30 to-transparent hidden md:block z-0"></div>

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
                        "For the Daydream Initiative, I identified a need for pre-existing instructional materials that could scaffold the learner's journey without requiring them to write a backstory from scratch. I selected <strong class='text-white'>Carl Jung’s 12 Archetypes</strong> as the foundational material, recognizing their universality and psychological depth. However, these abstract concepts could not be used in their raw form within a digital learning environment. Therefore, I <span class='text-amber-300 font-medium'>modified</span> these materials, translating the psychological traits of the 'Sage' or 'Hero' into quantifiable game mechanics."
                    </p>

                    <p>
                        "This modification process involved converting qualitative descriptions into a structured <span class='text-amber-300 font-medium'>JSON schema</span>. For example, I translated the Sage's 'desire for truth' into a mechanical '+2 Intelligence' buff and specific dialogue unlocks ('I seek the knowledge within'). This transformation allowed me to integrate the selected material directly into the 'Persona Engine,' turning a static psychological theory into a dynamic, interactive variable that drives the learner's experience."
                    </p>

                    <p>
                        "By creating these 'Character Templates' (e.g., 'Totem the Sasquatch,' 'Unit 734'), I provided a scaffolding tool that addresses the 'Creator Tooling Gap.' These modified materials serve as a starting point for both learners (who adopt a persona) and Instructional Designers (who can fork and edit the JSON). This approach demonstrates the ability to not only select appropriate theoretical materials but to fundamentally re-engineer them for a new medium, enhancing their instructional value through technological integration."
                    </p>
                </div>
            </div>
        </div>
    }
}
