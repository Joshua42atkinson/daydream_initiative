use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn VisionOfChange() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-fuchsia-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"Solving the Paradox"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-fuchsia-500/20 text-fuchsia-300 text-xs font-bold uppercase tracking-widest border border-fuchsia-500/30">
                        "Evaluation & Implementation"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-fuchsia-500"></span>
                        "Challenge: Create a Vision of Change"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-fuchsia-900/20 border-fuchsia-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-fuchsia-500/20 flex items-center justify-center text-fuchsia-400 mb-4 shadow-lg shadow-fuchsia-900/30 ring-1 ring-fuchsia-500/30">
                                // Icon: Eye / Future / Change
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-fuchsia-300/80 uppercase tracking-widest">"Market Analysis"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"The Motivation Paradox"</h3>
                                <p class="text-sm text-fuchsia-200/70 font-medium">"A Vision for 'Tier 3+' Educational Ecosystems"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-fuchsia-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This market analysis critiques the 'Motivation Paradox' in current EdTech: the finding that extrinsic rewards (points/badges) often degrade intrinsic drive. It proposes a 'Vision of Change' that aligns organizational goals (user retention) with learning goals (mastery) by shifting to a 'Tier 3+' architecture—a 'Cognitive-Identity Tutor' that fosters volition rather than compulsion."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://docs.google.com/document/d/16q5E9qM_l-tV7O4zQ8uC5a6-Qk7tO9q/edit?usp=sharing"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-fuchsia-600 hover:bg-fuchsia-500 text-white text-sm font-semibold transition-all shadow-lg shadow-fuchsia-900/30 hover:shadow-fuchsia-700/50"
                                >
                                    <span>"Read Vision Statement"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-fuchsia-500/50 via-purple-500/30 to-transparent hidden md:block z-0"></div>

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
                        "I created a 'Vision of Change' that aligns the learning goal of <strong class='text-white'>deep conceptual mastery</strong> with the organizational goal of <span class='text-fuchsia-300 font-medium'>sustainable user engagement</span> by systematically solving the 'Motivation Paradox.' The current 'Gameducation' market often relies on 'Tier 1' gamification (extrinsic rewards), which produces short-term spikes in activity but fails to drive long-term retention or transfer. My vision proposes a shift to a 'Tier 3+' model—a 'Cognitive-Identity Tutor'—that aligns the organization's need for retention with the learner's need for psychological growth."
                    </p>

                    <p>
                        "This vision argues that 'enjoyment' is not a frivolous addition but a strategic necessity for performance. By replacing 'points-ification' with 'Deep Gamification,' the organization moves from a model of <strong class='text-white'>compulsion</strong> (hooking users with psychological tricks) to a model of <span class='text-fuchsia-300 font-medium'>volition</span> (supporting users' internal drive). This alignment ensures that the metrics of success—time-on-task and assessment scores—are driven by genuine interest rather than artificial incentives, creating a more robust and ethically sound product."
                    </p>

                    <p>
                        "Furthermore, this vision frames the 'Daydream' initiative as a solution to the 'Edutainment Gap.' It posits that by respecting the learner's intelligence and autonomy, the organization can achieve higher performance outcomes than competitors who rely on 'shallow' engagement tactics. This demonstrates the ability to articulate a strategic direction that is both pedagogically rigorous and commercially viable, positioning the instructional design function as a driver of organizational innovation."
                    </p>
                </div>
            </div>
        </div>
    }
}
