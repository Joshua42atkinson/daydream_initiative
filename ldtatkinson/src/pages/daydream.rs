use crate::components::layout::Layout;
use leptos::prelude::*;

#[derive(Clone)]
struct DaydreamItem {
    title: &'static str,
    summary: &'static str,
    reflection: &'static str,
    tags: Vec<&'static str>,
}

#[component]
pub fn Daydream() -> impl IntoView {
    let (selected_item, set_selected_item) = create_signal::<Option<DaydreamItem>>(None);

    // --- ARTISTIC FLAIR: The Persona Quiz State ---
    let (quiz_stage, set_quiz_stage) = create_signal(0); // 0=Start, 1=Question, 2=Result
    let (archetype, set_archetype) = create_signal(""); 

    let items = vec![
        DaydreamItem {
            title: "Systematic Design (ADDIE)",
            summary: "Selecting and modifying instructional design models to fit unique parameters.",
            reflection: "I selected the ADDIE model. Counter-intuitively, for a solo project with limited resources, a robust, front-loaded design model is a necessity, not a luxury. \n\nADDIE maximizes scarce time by ensuring the 'Design' phase serves as a perfect blueprint for AI-assisted construction.",
            tags: vec!["ADDIE", "Dick & Carey", "Systematic"],
        },
        DaydreamItem {
            title: "Instructional Interventions",
            summary: "Designing specific interventions that replicate cognitive processes.",
            reflection: "The platform is architected around Self-Determination Theory (SDT). \n\nAutonomy is fulfilled by the 'Persona Engine' (meaningful choice). Competence is fulfilled by the 'LitRPG' stats (visible progress). Relatedness is fulfilled by the system validating the learner's identity.",
            tags: vec!["SDT", "Motivation", "Scaffolding"],
        },
        DaydreamItem {
            title: "Develop Materials",
            summary: "Creating multimedia materials that align with instructional strategies.",
            reflection: "I created materials in multiple formats to suit different needs: a static Infographic for stakeholders (declarative 'pull' learning) and a dynamic Screencast for students (procedural 'push' learning). \n\nThis flexibility ensures the message is delivered effectively for the specific audience.",
            tags: vec!["Multimedia", "Content Creation", "Infographic"],
        },
        DaydreamItem {
            title: "Learning Assessment",
            summary: "Ensuring alignment between goals, strategies, and measurement.",
            reflection: "Daydream fuses learning processes with outcomes. \n\nCognitive Loop: Progress is the outcome. The learner must apply vocabulary to advance the story. \n\nMetacognitive Loop: The process of reflecting creates the artifact (the journal entry), which is then measured for self-awareness.",
            tags: vec!["Alignment", "Measurement", "Metacognition"],
        }
    ];

    view! {
        <Layout>
            <div class="aurora-bg daydream-aurora"></div>
            <div class="max-w-5xl mx-auto space-y-12">
                <div class="text-center space-y-4">
                    <div class="inline-flex items-center px-3 py-1 rounded-full glass-panel border-emerald-900/50 text-emerald-400 text-xs font-bold tracking-widest uppercase">
                        <span class="w-2 h-2 bg-emerald-400 rounded-full mr-2 animate-pulse"></span>
                        "Supra-Badge 03"
                    </div>
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Design & " <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-green-600">"Development"</span>
                    </h1>
                </div>

                // --- INTERACTIVE PERSONA QUIZ (Artistic Flair) ---
                <div class="glass-panel p-8 rounded-2xl border border-emerald-500/30 text-center relative overflow-hidden transition-all duration-500">
                    <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-emerald-400 to-cyan-500"></div>
                    
                    {move || match quiz_stage.get() {
                        0 => view! {
                            <div class="space-y-4 animate-in fade-in zoom-in duration-300">
                                <h2 class="text-2xl font-bold text-white">"Discover Your Learning Persona"</h2>
                                <p class="text-slate-300">"The Daydream platform adapts to you. Take the archetype assessment to see the engine in action."</p>
                                <button 
                                    class="px-6 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-full font-bold transition shadow-lg shadow-emerald-900/50"
                                    on:click=move |_| set_quiz_stage.set(1)
                                >
                                    "Begin Assessment"
                                </button>
                            </div>
                        }.into_any(),
                        1 => view! {
                            <div class="space-y-6 text-left max-w-xl mx-auto animate-in slide-in-from-right duration-300">
                                <span class="text-xs font-mono text-emerald-400 uppercase tracking-widest">"Scenario 01: The Marketplace"</span>
                                <p class="text-lg text-white font-medium">"You see a merchant being harassed by a corrupt guard. The crowd watches in silence. What is your instinct?"</p>
                                <div class="grid grid-cols-1 gap-3">
                                    <button 
                                        class="p-4 text-left rounded-xl bg-slate-800/50 border border-slate-700 hover:border-emerald-400 hover:bg-slate-800 transition group"
                                        on:click=move |_| { set_archetype.set("The Hero"); set_quiz_stage.set(2); }
                                    >
                                        <span class="font-bold text-emerald-300 group-hover:text-white">"Intervene."</span>
                                        <span class="block text-sm text-slate-400">"Step between them. Justice requires action, regardless of the risk."</span>
                                    </button>
                                    <button 
                                        class="p-4 text-left rounded-xl bg-slate-800/50 border border-slate-700 hover:border-cyan-400 hover:bg-slate-800 transition group"
                                        on:click=move |_| { set_archetype.set("The Sage"); set_quiz_stage.set(2); }
                                    >
                                        <span class="font-bold text-cyan-300 group-hover:text-white">"Observe."</span>
                                        <span class="block text-sm text-slate-400">"Watch the guard's pattern. Understanding the system reveals the weakness."</span>
                                    </button>
                                </div>
                            </div>
                        }.into_any(),
                        2 => view! {
                            <div class="space-y-4 animate-in zoom-in duration-500">
                                <div class="inline-block p-4 rounded-full bg-emerald-500/20 mb-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-12 h-12 text-emerald-400">
                                      <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                                    </svg>
                                </div>
                                <h2 class="text-3xl font-bold text-white">"Archetype Identified: " <span class="text-emerald-400">{archetype.get()}</span></h2>
                                <p class="text-slate-300 max-w-lg mx-auto">
                                    {move || if archetype.get() == "The Hero" {
                                        "Your learning style is active and challenge-driven. The system has applied a +2 Bravery buff to your profile."
                                    } else {
                                        "Your learning style is analytical and observant. The system has applied a +2 Insight buff to your profile."
                                    }}
                                </p>
                                <button 
                                    class="text-sm text-slate-500 hover:text-white underline decoration-dotted"
                                    on:click=move |_| set_quiz_stage.set(0)
                                >
                                    "Reset Simulation"
                                </button>
                            </div>
                        }.into_any(),
                        _ => view! { <div></div> }.into_any()
                    }}
                </div>

                // Content Cards
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {items.into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <div 
                                class="glass-card p-6 rounded-xl flex flex-col h-full relative overflow-hidden group cursor-pointer hover:border-emerald-400/50 transition-all duration-300"
                                on:click=move |_| set_selected_item.set(Some(item_clone.clone()))
                            >
                                <h3 class="text-xl font-bold text-white mb-2 group-hover:text-emerald-400 transition">{item.title}</h3>
                                <p class="text-sm text-slate-400 flex-grow">{item.summary}</p>
                                <div class="mt-4 pt-4 border-t border-slate-700/50 flex gap-2">
                                    {item.tags.iter().map(|tag| view! { 
                                        <span class="text-xs font-mono text-emerald-400 bg-emerald-900/20 px-2 py-1 rounded">{*tag}</span>
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
                        <div class="glass-panel p-8 max-w-2xl w-full rounded-2xl relative border border-emerald-500/30 shadow-2xl shadow-emerald-900/20"
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
