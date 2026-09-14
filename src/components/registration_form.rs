use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{register_user, RegistrationOutcome};

#[derive(Clone)]
enum FormStatus {
    Idle,
    Submitting,
    Success(String),
    Error(String),
}

#[component]
pub fn RegistrationForm() -> impl IntoView {
    let (first_names, set_first_names) = signal(String::new());
    let (last_names, set_last_names) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (student_number, set_student_number) = signal(String::new());
    let status = RwSignal::new(FormStatus::Idle);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let first = first_names.get_untracked();
        let last = last_names.get_untracked();
        let mail = email.get_untracked();
        let ph = phone.get_untracked();
        let num = student_number.get_untracked();

        if first.trim().is_empty() || last.trim().is_empty() || mail.trim().is_empty() {
            status.set(FormStatus::Error(
                "Merci de renseigner au moins ton prénom, ton nom et ton email.".to_string(),
            ));
            return;
        }

        status.set(FormStatus::Submitting);

        spawn_local(async move {
            let phone_opt = (!ph.trim().is_empty()).then_some(ph);
            let student_opt = (!num.trim().is_empty()).then_some(num);

            match register_user(first, last, mail, phone_opt, student_opt).await {
                Ok(RegistrationOutcome::EmailSent) => {
                    status.set(FormStatus::Success(
                        "Un email de confirmation vient de t'être envoyé. Vérifie ta boîte de réception (et tes spams) pour valider ton inscription.".to_string(),
                    ));
                    set_first_names.set(String::new());
                    set_last_names.set(String::new());
                    set_email.set(String::new());
                    set_phone.set(String::new());
                    set_student_number.set(String::new());
                }
                Ok(RegistrationOutcome::AlreadyVerified) => {
                    status.set(FormStatus::Success(
                        "Cette adresse est déjà inscrite et confirmée : pas besoin de refaire quoi que ce soit !".to_string(),
                    ));
                }
                Err(err) => {
                    status.set(FormStatus::Error(err.to_string()));
                }
            }
        });
    };

    view! {
        <section class="registration" id="inscription">
            <h2>"Inscription"</h2>
            <form on:submit=on_submit novalidate=true>
                <div class="field-row">
                    <label class="field">
                        <span>"Prénom(s)"</span>
                        <input
                            type="text"
                            autocomplete="given-name"
                            prop:value=move || first_names.get()
                            on:input=move |ev| set_first_names.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="field">
                        <span>"Nom(s)"</span>
                        <input
                            type="text"
                            autocomplete="family-name"
                            prop:value=move || last_names.get()
                            on:input=move |ev| set_last_names.set(event_target_value(&ev))
                        />
                    </label>
                </div>

                <label class="field">
                    <span>"Email (universitaire ou personnel)"</span>
                    <input
                        type="email"
                        autocomplete="email"
                        prop:value=move || email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                </label>

                <div class="field-row">
                    <label class="field">
                        <span>"Téléphone " <em>"(optionnel)"</em></span>
                        <input
                            type="tel"
                            autocomplete="tel"
                            prop:value=move || phone.get()
                            on:input=move |ev| set_phone.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="field">
                        <span>"Numéro étudiant " <em>"(optionnel)"</em></span>
                        <input
                            type="text"
                            prop:value=move || student_number.get()
                            on:input=move |ev| set_student_number.set(event_target_value(&ev))
                        />
                    </label>
                </div>

                <button
                    type="submit"
                    disabled=move || matches!(status.get(), FormStatus::Submitting)
                >
                    {move || if matches!(status.get(), FormStatus::Submitting) {
                        "Envoi en cours…"
                    } else {
                        "S'inscrire"
                    }}
                </button>

                {move || match status.get() {
                    FormStatus::Success(msg) => {
                        view! { <p class="form-message form-message-success">{msg}</p> }.into_any()
                    }
                    FormStatus::Error(msg) => {
                        view! { <p class="form-message form-message-error">{msg}</p> }.into_any()
                    }
                    FormStatus::Idle | FormStatus::Submitting => ().into_any(),
                }}
            </form>
        </section>
    }
}
