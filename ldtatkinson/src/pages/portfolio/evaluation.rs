use crate::components::layout::Layout;
use leptos::prelude::*;

#[derive(Clone)]
struct EvaluationItem {
    title: &'static str,
    summary: &'static str,
    reflection: &'static str,
    tags: Vec<&'static str>,
}

#[component]
pub fn Evaluation() -> impl IntoView {
    let (selected_item, set_selected_item) = create_signal::<Option<EvaluationItem>>(None);

    let items = vec![
        EvaluationItem {
            title: "Evaluate Interventions",
            summary: "Formative and Summative plans to validate design efficacy.",
            reflection: "Implementation utilizes an 'AI Mentor' to provide continuous, Socratic feedback (Formative). Summative evaluation ensures alignment via a mixed-methods assessment: a quantitative checklist for the 'Capstone Narrative' and a qualitative rubric for the 'Design Rationale'.",
            tags: vec!["Formative", "Summative", "Validation"],
        },
        EvaluationItem {
            title: "Dissemination & Diffusion",
            summary: "Strategies for communicating vision and achieving adoption.",
            reflection: "The plan leverages the Google for Nonprofits suite for external outreach. Strategically uses the MIT License (and planned GPLv3) as a diffusion mechanism, ensuring the IP fosters a virtuous cycle of open-source innovation.",
            tags: vec!["Change Mgmt", "Adoption", "Open Source"],
        }
    ];

    view! {
        <Layout>
            <div class="aurora-bg meet-the-designer-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12 pt-10">
                <div class="text-center">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-rose-900/50 text-rose-400 text-xs font-bold tracking-widest uppercase mb-4">
                        "Supra-Badge 04"
                    </div>
                    <h1 class="text-4xl font-bold text-white mb-4">"Evaluation & Implementation"</h1>
                    <p class="text-slate-300">"Measuring efficacy and managing diffusion."</p>
                </div>

                 <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {items.into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <div 
                                class="glass-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group cursor-pointer hover:border-rose-400/50 transition-all duration-300"
                                on:click=move |_| set_selected_item.set(Some(item_clone.clone()))
                            >
                                <h3 class="text-xl font-bold text-white mb-2 group-hover:text-rose-400 transition">{item.title}</h3>
                                <p class="text-sm text-slate-400 flex-grow">{item.summary}</p>
                                <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                                    {item.tags.iter().map(|tag| view! { 
                                        <span class="text-xs font-mono text-rose-400 bg-rose-900/20 px-2 py-1 rounded">{*tag}</span>
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
                
                // Modal Logic (Same as other pages)
                {move || selected_item.get().map(|item| view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
                         on:click=move |_| set_selected_item.set(None)>
                        <div class="glass-panel p-8 max-w-2xl w-full rounded-2xl relative border border-rose-500/30 shadow-2xl shadow-rose-900/20"
                             on:click=move |e| e.stop_propagation()>
                            <button class="absolute top-4 right-4 text-slate-400 hover:text-white" on:click=move |_| set_selected_item.set(None)>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /></svg>
                            </button>
                            <h2 class="text-3xl font-bold text-white mb-4">{item.title}</h2>
                            <div class="prose prose-invert max-w-none text-slate-300 whitespace-pre-wrap leading-relaxed">{item.reflection}</div>
                        </div>
                    </div>
                })}
            </div>
        </Layout>
    }
}
