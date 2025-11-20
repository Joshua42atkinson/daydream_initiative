use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title, Html};
use leptos_router::{components::*, path};

use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::daydream::Daydream;
use crate::pages::meet_the_designer::MeetTheDesigner;

// Import the new Portfolio Sub-Pages
use crate::pages::portfolio::portfolio_home::PortfolioHome;
use crate::pages::portfolio::foundations::Foundations;
use crate::pages::portfolio::planning::Planning;
use crate::pages::portfolio::design::Design;
use crate::pages::portfolio::evaluation::Evaluation;
use crate::pages::portfolio::analyze_existing_emerging_technologies::AnalyzeExistingEmergingTechnologies;
use crate::pages::portfolio::assessment_alignment::AssessmentAlignment;
use crate::pages::portfolio::design_a_plan_for_dissemination_and_diffusion::DesignAPlanForDisseminationAndDiffusion;
use crate::pages::portfolio::design_and_development::DesignAndDevelopment;
use crate::pages::portfolio::engaging_presentation::EngagingPresentation;
use crate::pages::portfolio::ethics_compliance::EthicsCompliance;
use crate::pages::portfolio::ethics_constraints::EthicsConstraints;
use crate::pages::portfolio::formative_eval::FormativeEval;
use crate::pages::portfolio::gap_analysis::GapAnalysis;
use crate::pages::portfolio::instructional_sequencing::InstructionalSequencing;
use crate::pages::portfolio::instructional_strategies::InstructionalStrategies;
use crate::pages::portfolio::interaction_design_and_usability::InteractionDesignAndUsability;
use crate::pages::portfolio::key_concepts::KeyConcepts;
use crate::pages::portfolio::learning_processes::LearningProcesses;
use crate::pages::portfolio::motivational_design::MotivationalDesign;
use crate::pages::portfolio::select_or_modify_materials::SelectOrModifyMaterials;
use crate::pages::portfolio::solicit_accept_and_provide_feedback::SolicitAcceptAndProvideFeedback;
use crate::pages::portfolio::summative_eval::SummativeEval;
use crate::pages::portfolio::systems_thinking::SystemsThinking;
use crate::pages::portfolio::target_population::TargetPopulation;
use crate::pages::portfolio::tech_skills::TechSkills;
use crate::pages::portfolio::vision_of_change::VisionOfChange;
use crate::pages::portfolio::visual_design::VisualDesign;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="LDT Atkinson" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/daydream") view=Daydream />
                <Route path=path!("/meet-the-designer") view=MeetTheDesigner />

                // Portfolio Nested Routes
                <Route path=path!("/portfolio") view=PortfolioHome />
                <Route path=path!("/portfolio/foundations") view=Foundations />
                <Route path=path!("/portfolio/planning") view=Planning />
                <Route path=path!("/portfolio/design") view=Design />
                <Route path=path!("/portfolio/evaluation") view=Evaluation />
                <Route path=path!("/portfolio/analyze-existing-emerging-technologies") view=AnalyzeExistingEmergingTechnologies />
                <Route path=path!("/portfolio/assessment-alignment") view=AssessmentAlignment />
                <Route path=path!("/portfolio/design-a-plan-for-dissemination-and-diffusion") view=DesignAPlanForDisseminationAndDiffusion />
                <Route path=path!("/portfolio/design-and-development") view=DesignAndDevelopment />
                <Route path=path!("/portfolio/engaging-presentation") view=EngagingPresentation />
                <Route path=path!("/portfolio/ethics-compliance") view=EthicsCompliance />
                <Route path=path!("/portfolio/ethics-constraints") view=EthicsConstraints />
                <Route path=path!("/portfolio/formative-eval") view=FormativeEval />
                <Route path=path!("/portfolio/gap-analysis") view=GapAnalysis />
                <Route path=path!("/portfolio/instructional-sequencing") view=InstructionalSequencing />
                <Route path=path!("/portfolio/instructional-strategies") view=InstructionalStrategies />
                <Route path=path!("/portfolio/interaction-design-and-usability") view=InteractionDesignAndUsability />
                <Route path=path!("/portfolio/key-concepts") view=KeyConcepts />
                <Route path=path!("/portfolio/learning-processes") view=LearningProcesses />
                <Route path=path!("/portfolio/motivational-design") view=MotivationalDesign />
                <Route path=path!("/portfolio/select-or-modify-materials") view=SelectOrModifyMaterials />
                <Route path=path!("/portfolio/solicit-accept-and-provide-feedback") view=SolicitAcceptAndProvideFeedback />
                <Route path=path!("/portfolio/summative-eval") view=SummativeEval />
                <Route path=path!("/portfolio/systems-thinking") view=SystemsThinking />
                <Route path=path!("/portfolio/target-population") view=TargetPopulation />
                <Route path=path!("/portfolio/tech-skills") view=TechSkills />
                <Route path=path!("/portfolio/vision-of-change") view=VisionOfChange />
                <Route path=path!("/portfolio/visual-design") view=VisualDesign />
            </Routes>
        </Router>
    }
}
