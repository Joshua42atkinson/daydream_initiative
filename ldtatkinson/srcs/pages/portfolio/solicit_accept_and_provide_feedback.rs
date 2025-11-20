use leptos::*;
use crate::components::glass_panel::GlassPanel;

#[component]
pub fn SolicitAcceptAndProvideFeedback() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-orange-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Feedback Loop"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-orange-500/20 text-orange-300 text-xs font-bold uppercase tracking-widest border border-orange-500/30">
                        "Professional Foundations in LDT"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-orange-500"></span>
                        "Challenge: Solicit, Accept, & Provide Feedback"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-orange-900/20 border-orange-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-orange-500/20 flex items-center justify-center text-orange-400 mb-4 shadow-lg shadow-orange-900/30 ring-1 ring-orange-500/30">
                                // Icon: Chat / Feedback / Network
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-orange-300/80 uppercase tracking-widest">"Dev Blog"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"From Mindmap to Model"</h3>
                                <p class="text-sm text-orange-200/70 font-medium">"The Rise of AI: A Reflective Journal"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-orange-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact is a dynamic 'Dev Blog' established to document the evolution of the Daydream project. It serves as a transparent mechanism for stakeholder management, transforming the private design process into a public 'Reflective Practice.' By showcasing rationale and prototypes, it creates a centralized hub for soliciting and integrating feedback from the professional learning network (PLN)."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://daydream67.blogspot.com/2025/11/from-mindmap-to-model-rise-of-ai.html"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-orange-600 hover:bg-orange-500 text-white text-sm font-semibold transition-all shadow-lg shadow-orange-900/30 hover:shadow-orange-700/50"
                                >
                                    <span>"Read Blog Post"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-orange-500/50 via-red-500/30 to-transparent hidden md:block z-0"></div>

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
                        "The technical acquisition of reflective writing skills was honed through the deployment of a Blogger platform, fulfilling the criteria for a well-structured, hyperlinked post. However, the true competency lies in applying this tool within a professional context as a vehicle for <strong class='text-white'>'Reflective Practice.'</strong> By maintaining a 'dev blog,' I established a discipline of documenting design decisions and tracing the evolution of the Daydream project, transforming a simple publishing tool into a foundational element of professional growth."
                    </p>

                    <p>
                        "This artifact further demonstrates the ability to <span class='text-orange-300 font-medium'>'Deliver presentations that effectively engage audiences.'</span> A blog post functions as a static, asynchronous presentation, capable of introducing new topics or posing reflective questions within an instructional module (e.g., Brightspace). This application validates the competency to write and edit messages that are clear, concise, and grammatically correct, proving that effective instructional communication can transcend synchronous delivery to engage audiences in a public-facing context."
                    </p>

                    <p>
                        "Crucially, this blog serves as a strategic instrument for stakeholder management and the <strong class='text-white'>'Solicitation of Feedback.'</strong> By publicly showcasing prototype screenshots and explaining design rationales, the blog creates a transparent record of the project's lifecycle. This centralized location allows for the efficient soliciting, accepting, and providing of constructive feedback from Subject Matter Experts (SMEs) and the project team, ensuring that the design process is collaborative and responsive to stakeholder input."
                    </p>
                </div>
            </div>
        </div>
    }
}
