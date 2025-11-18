use crate::components::layout::Layout;
use leptos::prelude::*;

#[component]
pub fn MeetTheDesigner() -> impl IntoView {
    view! {
        <Layout>
            <div class="aurora-bg meet-the-designer-aurora"></div>
            <div class="max-w-4xl mx-auto space-y-12 pt-10">
                
                 <div class="text-center space-y-4">
                    <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight text-white">
                        "Meet the " <span class="text-transparent bg-clip-text bg-gradient-to-r from-rose-400 to-orange-500">"Designer"</span>
                    </h1>
                </div>

                <div class="glass-panel p-10 rounded-3xl md:flex gap-10 items-start border border-rose-500/20 shadow-2xl shadow-rose-900/20">
                    // Avatar
                    <div class="w-40 h-40 rounded-full bg-gradient-to-br from-rose-500 to-orange-400 flex-shrink-0 mb-6 md:mb-0 border-4 border-slate-800 shadow-xl flex items-center justify-center text-5xl font-bold text-white">
                        "JA"
                    </div>
                    
                    <div class="space-y-6">
                        <div>
                            <h3 class="text-3xl font-bold text-white">"Joshua Atkinson"</h3>
                            <p class="text-rose-400 font-mono text-sm uppercase tracking-wide">"Marine Corps Veteran • Instructional Designer"</p>
                        </div>
                        
                        <p class="text-slate-300 leading-relaxed text-lg font-light italic border-l-4 border-rose-500/50 pl-4">
                            "From pastor's kid to Marine Gunnery Sergeant, from biker bars to political boardrooms, my life has been a kaleidoscope of experiences."
                        </p>
                        
                        <p class="text-slate-400 leading-relaxed">
                            "After trading in camo for cables as an IT installer, I launched a trucking company, tossed pizzas, was a bartender, and even worked in a non-profit community development program. But beneath the public service and business suits is an intense desire to develop challenging and playful learning experiences for myself and others."
                        </p>
                        
                        <div class="pt-6 flex flex-wrap gap-4 text-sm font-mono text-slate-500">
                            <a href="mailto:joshua42atkinson@gmail.com" class="hover:text-rose-400 transition flex items-center gap-2">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75" /></svg>
                                "joshua42atkinson@gmail.com"
                            </a>
                            <span>"•"</span>
                            <span>"Houlton, Maine"</span>
                        </div>
                    </div>
                 </div>
            </div>
        </Layout>
    }
}
