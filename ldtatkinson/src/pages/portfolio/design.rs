use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn Design() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg daydream-aurora"></div> // Green Theme
            <div class="max-w-5xl mx-auto space-y-12 pt-10">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-white mb-4">"Design & Development"</h1>
                    <p class="text-slate-300">"The build phase. Applying systematic models (ADDIE) to create aligned interventions."</p>
                </div>

                <div class="grid grid-cols-1 gap-6">
                    <div class="glass-card p-8 rounded-xl">
                        <h3 class="text-2xl font-bold text-emerald-400 mb-2">"Systematic Design (ADDIE)"</h3>
                        <p class="text-slate-300 leading-relaxed">"I selected the ADDIE model. Counter-intuitively, for a solo project with limited resources, a robust, front-loaded design model is a necessity, not a luxury. ADDIE maximizes scarce time by ensuring the 'Design' phase serves as a perfect blueprint for AI-assisted construction."</p>
                    </div>
                    <div class="glass-card p-8 rounded-xl">
                        <h3 class="text-2xl font-bold text-emerald-400 mb-2">"Instructional Interventions"</h3>
                        <p class="text-slate-300 leading-relaxed">"The platform is architected around Self-Determination Theory (SDT). Autonomy is fulfilled by the 'Persona Engine' (meaningful choice). Competence is fulfilled by the 'LitRPG' stats (visible progress). Relatedness is fulfilled by the system validating the learner's identity."</p>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
