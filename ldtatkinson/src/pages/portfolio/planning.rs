use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Planning() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg portfolio-aurora"></div> // Purple Theme
            <div class="max-w-5xl mx-auto space-y-12 pt-10">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-white mb-4">"Planning & Analysis"</h1>
                    <p class="text-slate-300">"The diagnostic phase. Identifying performance gaps and learner contexts."</p>
                </div>

                <div class="grid grid-cols-1 gap-6">
                    <div class="glass-card p-8 rounded-xl">
                        <h3 class="text-2xl font-bold text-purple-400 mb-2">"Gap Analysis"</h3>
                        <p class="text-slate-300 leading-relaxed">"I identified the learning and performance problems by conducting a formal Gap Analysis... Gap 1: The Creator Tooling Gap. Existing tools are either too simple (Twine) or require expert programming. Gap 2: The Pedagogical-Technical Integration Gap. Educators understand CLT/SDT but lack tools to implement them."</p>
                    </div>
                    <div class="glass-card p-8 rounded-xl">
                        <h3 class="text-2xl font-bold text-purple-400 mb-2">"Target Population & Environment"</h3>
                        <p class="text-slate-300 leading-relaxed">"My analysis determined that the instructional environment was hostile to standard 'social sharing'. Legal Environment: A learner's reflection is sensitive PII. Emailing summaries is a 'catastrophic violation'. Psychological Environment: Learning creates 'interpersonal risk'. This mandated a Granular Consent Architecture."</p>
                    </div>
                     <div class="glass-card p-8 rounded-xl">
                        <h3 class="text-2xl font-bold text-purple-400 mb-2">"Analyze Technologies"</h3>
                        <p class="text-slate-300 leading-relaxed">"My analysis led me to select 'bleeding-edge' but 'unconventional' technologies (Rust, Bevy, Whisper) to create a secure, self-contained system. This choice introduced a conflict between the async web server and synchronous game engine, which I solved using a 'bridge' architectural pattern."</p>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
