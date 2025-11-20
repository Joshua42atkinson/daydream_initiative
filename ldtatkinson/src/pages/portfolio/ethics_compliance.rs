use leptos::prelude::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn EthicsCompliance() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-slate-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Ethical Foundation"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-slate-500/20 text-slate-300 text-xs font-bold uppercase tracking-widest border border-slate-500/30">
                        "Professional Foundations"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-slate-500"></span>
                        "Challenge: Comply with Codes of Ethics"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-slate-900/20 border-slate-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-slate-500/20 flex items-center justify-center text-slate-400 mb-4 shadow-lg shadow-slate-900/30 ring-1 ring-slate-500/30">
                                // Icon: Scale / Badge / Law
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-slate-300/80 uppercase tracking-widest">"Certification"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Responsible Conduct of Research"</h3>
                                <p class="text-sm text-slate-200/70 font-medium">"RCR Training Certificate"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-slate-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This formal certification verifies competency in the ethical standards mandated for research involving human subjects. It covers data integrity, responsible authorship, and conflicts of interest. This training serves as the regulatory bedrock for the Daydream project's privacy architecture, ensuring that 'Psychological Safety' is not just a design goal, but an ethical obligation."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="#" // Link to your specific PDF or Google Drive file
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-slate-600 hover:bg-slate-500 text-white text-sm font-semibold transition-all shadow-lg shadow-slate-900/30 hover:shadow-slate-700/50"
                                >
                                    <span>"View Certificate"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-slate-500/50 via-gray-500/30 to-transparent hidden md:block z-0"></div>

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
                        "For the competency 'Comply with organizational and professional code of ethics,' I have chosen my <strong class='text-white'>Responsible Conduct of Research (RCR) Training Certificate</strong> as the artifact. This certificate serves as tangible evidence of my engagement with professional ethical standards. The successful completion of this training directly demonstrates my commitment to adhering to a professional code of ethics, providing a framework for responsible decision-making that ensures integrity, fairness, and respect for learners and stakeholders."
                    </p>

                    <p>
                        "The RCR training specifically addressed key ethical considerations relevant to instructional design, including data integrity, responsible authorship, and conflicts of interest. My understanding of these principles is further reinforced by my prior experience with the <span class='text-slate-300 font-medium'>Citi Bank ethics course</span>, which underscored the importance of shared values in fostering a responsible organizational culture. This insight is directly transferable to my work in Learning Design and Technology (LDT), where ethical considerations must guide the creation of inclusive and respectful learning experiences that promote positive outcomes for a diverse range of learners."
                    </p>

                    <p>
                        "Through this experience, I have gained a deeper appreciation for how ethical considerations directly inform and shape responsible decision-making. Moving forward, I will actively apply these principles to ensure that my instructional designs are not only effective but also ethically sound and culturally sensitive. To further develop this competency, I intend to engage with specific ethical codes from professional organizations like the <strong class='text-white'>Association for Educational Communications and Technology (AECT)</strong> and continuously reflect on the ethical implications of my design choices in future academic and professional endeavors."
                    </p>
                </div>
            </div>
        </div>
    }
}
