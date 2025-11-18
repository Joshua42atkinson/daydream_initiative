use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

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
            </Routes>
        </Router>
    }
}
