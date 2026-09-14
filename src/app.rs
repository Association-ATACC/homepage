use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::{Header, RegistrationForm, WeeklySchedule};

/// Full HTML document shell. Used by the server to render each page and by
/// `leptos_axum::file_and_error_handler` to render error pages.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/atacc-inscription.css"/>
        <Title text="ATACC — Inscription"/>
        <Meta name="description" content="Inscris-toi à l'ATACC, l'association de tutorat et d'aide entre étudiants."/>

        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Page introuvable."</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Home page: header, registration form, and the weekly schedule.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Header/>
        <RegistrationForm/>
        <WeeklySchedule/>
    }
}
