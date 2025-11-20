use crate::components::layout::Layout;
use leptos::prelude::*;

#[derive(Clone)]
struct FoundationItem {
    title: &'static str,
    summary: &'static str,
    reflection: &'static str,
    tags: Vec<&'static str>,
}

#[component]
pub fn Foundations() -> impl IntoView {
    let (selected_item, set_selected_item) = signal::<Option<FoundationItem>>(None);

    let items = vec![
        FoundationItem {
            title: "ID Professional Communicator",
            summary: "Mastering clear messaging and soliciting constructive feedback.",
            reflection: "I acquired the skill of reflective writing using a blog platform ('From Mindmap to Model'). \n\nIn ID practice, this serves as a 'dev blog' for documenting design decisions and managing stakeholders. It demonstrates the ability to write clear, concise messages in a public instructional context.",
            tags: vec!["Constructive Feedback", "Engagement"],
        },
        FoundationItem {
            title: "Applying ID Research & Theory",
            summary: "Utilizing Systems Thinking (ADDIE, SDT) to break down problems.",
            reflection: "My 'Vocabulary-as-a-Mechanic' (VaaM) report demonstrates systems thinking by breaking vocabulary acquisition into a three-phase loop: Acquisition (Input), Application (Process), and Reinforcement (Output/Feedback).",
            tags: vec!["Systems Thinking", "ID Concepts"],
        },
        FoundationItem {
            title: "ID Knowledge & Skills",
            summary: "Continuous professional development in a rapidly evolving field.",
            reflection: "I earned the 'LDT Technology Badge in Website Development' using Google Sites. Limitations in customization drove me to explore advanced stacks (like this Rust/WASM site).",
            tags: vec!["Professional Dev", "Tech Skills"],
        },
        FoundationItem {
            title: "Ethical & Legal Implications",
            summary: "Navigating constraints and complying with ethical codes.",
            reflection: "My 'Responsible Conduct of Research (RCR)' certificate demonstrates commitment to data integrity. I connect this to my 'Privacy-First' architecture for Daydream.",
            tags: vec!["Constraints", "Code of Ethics"],
        },
    ];

    view! {
        <Layout>
            <div class="aurora-bg home-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12 pt-10">
                <div class="text-center">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-blue-900/50 text-blue-400 text-xs font-bold tracking-widest uppercase mb-4">
                        "Supra-Badge 01"
                    </div>
                    <h1 class="text-4xl font-bold text-white mb-4">"Professional Foundations"</h1>
                    <p class="text-slate-300">"The bedrock of the Learning Engineer. Ethical, theoretical, and communicative standards."</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {items.into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <div 
                                class="glass-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group cursor-pointer hover:border-blue-400/50 transition-all duration-300"
                                on:click=move |_| set_selected_item.set(Some(item_clone.clone()))
                            >
                                <h3 class="text-xl font-bold text-white mb-2 group-hover:text-blue-400 transition">{item.title}</h3>
                                <p class="text-sm text-slate-400 flex-grow">{item.summary}</p>
                                <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                                    {item.tags.iter().map(|tag| view! { 
                                        <span class="text-xs font-mono text-blue-400 bg-blue-900/20 px-2 py-1 rounded">{*tag}</span>
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>

                // Modal
                {move || selected_item.get().map(|item| view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
                         on:click=move |_| set_selected_item.set(None)>
                        <div class="glass-panel p-8 max-w-2xl w-full rounded-2xl relative border border-blue-500/30 shadow-2xl shadow-blue-900/20"
                             on:click=move |e| e.stop_propagation()>
                            <button class="absolute top-4 right-4 text-slate-400 hover:text-white"
                                    on:click=move |_| set_selected_item.set(None)>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /></svg>
                            </button>
                            <h2 class="text-3xl font-bold text-white mb-4">{item.title}</h2>
                            <div class="prose prose-invert max-w-none text-slate-300 whitespace-pre-wrap leading-relaxed">
                                {item.reflection}
                            </div>
                        </div>
                    </div>
                })}
            </div>
        </Layout>
    }
}
