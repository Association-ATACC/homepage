//! Site header component: logo, link to the ``ATACCothèque`` and the hero
//! section inviting visitors to sign up for event notifications.

use leptos::prelude::*;

/// Site header with the ATACC logo, a link to the ``ATACCothèque``, and the
/// hero section that invites visitors to sign up for event notifications.
#[component]
pub fn Header() -> impl IntoView {
  view! {
    <header class="site-header">
      <div class="site-mark">
        <img class="site-logo" src="/atacc_logo.svg" alt="" width="40" height="40" />
        <span class="site-name">"ATACC"</span>
        <a id="biblio" class="button" href="https://biblio.atacc.org">
          "ATACCothèque"
        </a>
      </div>
      <div class="hero">
        <h1>"Inscris-toi pour rester informé des événements de l'ATACC"</h1>
        <p>
          "L'association de tutorat et d'aide met en lien des étudiants autour de créneaux d'entraide."
          <br /> "Inscris-toi ci-dessous : tu recevras un email pour confirmer ton adresse."
        </p>
      </div>
    </header>
  }
}
