use leptos::prelude::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn GapAnalysis() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-amber-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Creator Tooling Gap"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-amber-500/20 text-amber-300 text-xs font-bold uppercase tracking-widest border border-amber-500/30">
                        "Planning & Analysis"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
                        "Challenge: Conduct a Gap Analysis"
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
                                // Icon: Gap / Bridge / Disconnect
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-amber-300/80 uppercase tracking-widest">"Market Analysis"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Project Daydream: Needs Assessment"</h3>
                                <p class="text-sm text-amber-200/70 font-medium">"Identifying the 'Edutainment' & 'Tooling' Gaps"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-amber-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This strategic analysis identifies a critical market failure in Instructional Design: the 'Creator Tooling Gap.' It presents evidence that IDs are currently forced to choose between narrative flexibility (Twine) and technical power (Unity), leaving them unable to build complex, 'Tier 3' Intelligent Tutoring Systems without specialized engineering support."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/13J43zG_4yX_96r7W8V9L_X2J3k4w5y6/edit?usp=sharing"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold transition-all shadow-lg shadow-amber-900/30 hover:shadow-amber-700/50"
                                >
                                    <span>"View Gap Analysis"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-amber-500/50 via-orange-500/30 to-transparent hidden md:block z-0"></div>

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
                        "In the analysis phase of this initiative, I conducted a formal gap analysis to determine why narrative-driven instruction is rarely implemented at scale. My investigation revealed a dichotomy between the 'Actual Situation'—where Instructional Designers (IDs) are restricted to linear authoring tools like Storyline or Articulate—and the 'Desired Situation,' where IDs could build complex, branching Intelligent Tutoring Systems (ITS). I identified this as the <strong class='text-white'>'Creator Tooling Gap,'</strong> quantifying it by the high technical barrier (coding expertise) required to use game engines like Unity for educational purposes."
                    </p>

                    <p>
                        "To validate this gap, I analyzed the characteristics of the current market, specifically the 'Tier 1' gamification tools that focus on extrinsic rewards (points/badges) rather than intrinsic narrative structures. My data showed that while IDs possess the <span class='text-amber-300 font-medium'>pedagogical knowledge</span> to design complex learning environments (e.g., Vygotskian scaffolding), they lack the <span class='text-amber-300 font-medium'>technical agency</span> to implement them. This disconnect creates a 'Pedagogical-Technical Integration Gap,' where advanced learning science remains theoretical because the tools to operationalize it are inaccessible."
                    </p>

                    <p>
                        "This analysis directly informed the strategic direction of the Daydream project. Instead of building a single educational game, I determined the solution was to build a <span class='text-amber-300 font-medium'>'Creator's Sandbox'</span>—a platform specifically architected to bridge this gap. By automating the complex backend logic (Rust/Bevy) and presenting it through a familiar, no-code interface (Twine/Storyline paradigms), the initiative effectively closes the gap, empowering non-technical authors to deploy 'Tier 3' cognitive tutors. This demonstrates that effective ID intervention often requires looking beyond content gaps to solve systemic infrastructural deficiencies."
                    </p>
                </div>
            </div>
        </div>
    }
}
