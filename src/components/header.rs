use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="site-mark">
                <img class="site-logo" src="/atacc_logo.svg" alt="" width="40" height="40" />
                <span class="site-name">"ATACC"</span>
                <a id="biblio" href="https://biblio.atacc.org">ATACCothèque</a>
            </div>
            <div class="hero">
                <h1>"Inscris-toi pour rester informé des événements de l'ATACC"</h1>
                <p>
                    "L'association de tutorat et d'aide met en lien des étudiants autour de créneaux d'entraide."
                    <br/>
                    "Inscris-toi ci-dessous : tu recevras un email pour confirmer ton adresse."
                </p>
            </div>
        </header>
    }
}
