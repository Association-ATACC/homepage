//! Registration form component: collects the visitor's details, validates
//! them client-side and calls the `register_user` server function.

use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::pages::home::api::{RegistrationOutcome, register_user};

/// Submission state of the registration form.
#[derive(Clone)]
enum FormStatus {
  /// Nothing has been submitted yet.
  Idle,
  /// A request is currently in flight.
  Submitting,
  /// The request succeeded; holds the message shown to the visitor.
  Success(String),
  /// Validation or server error; holds the message shown to the visitor.
  Error(String),
}

/// Signals backing the form inputs.
#[derive(Clone, Copy)]
struct FormFields {
  /// First name(s).
  first_names: RwSignal<String>,
  /// Last name(s).
  last_names: RwSignal<String>,
  /// Email address.
  email: RwSignal<String>,
  /// Phone number.
  phone: RwSignal<String>,
  /// Student number.
  student_number: RwSignal<String>,
}

impl FormFields {
  /// Creates a set of empty fields.
  fn new() -> Self {
    Self {
      first_names: RwSignal::new(String::new()),
      last_names: RwSignal::new(String::new()),
      email: RwSignal::new(String::new()),
      phone: RwSignal::new(String::new()),
      student_number: RwSignal::new(String::new()),
    }
  }

  /// Returns every field signal, in display order.
  const fn all(self) -> [RwSignal<String>; 5] {
    [
      self.first_names,
      self.last_names,
      self.email,
      self.phone,
      self.student_number,
    ]
  }

  /// Returns `true` when no field is empty or whitespace-only.
  fn is_complete(self) -> bool {
    self
      .all()
      .iter()
      .all(|field| !field.get_untracked().trim().is_empty())
  }

  /// Resets every field to an empty string.
  fn clear(self) {
    for field in self.all() {
      field.set(String::new());
    }
  }
}

/// Sends the registration request and maps the outcome to a form status.
async fn send_registration(fields: FormFields) -> FormStatus {
  let result = register_user(
    fields.first_names.get_untracked(),
    fields.last_names.get_untracked(),
    fields.email.get_untracked(),
    fields.phone.get_untracked(),
    fields.student_number.get_untracked(),
  )
  .await;

  match result {
    Ok(RegistrationOutcome::EmailSent) => {
      fields.clear();
      FormStatus::Success(
        "Un email de confirmation vient de t'être envoyé. Vérifie ta boîte de réception (et tes spams) pour valider ton inscription.".to_string(),
      )
    }
    Ok(RegistrationOutcome::AlreadyVerified) => FormStatus::Success(
      "Cette adresse est déjà inscrite et confirmée : pas besoin de refaire quoi que ce soit !".to_string(),
    ),
    Err(err) => FormStatus::Error(err.to_string()),
  }
}

/// Labelled text input bound to a signal.
#[component]
fn Field(
  /// Text displayed above the input.
  label: &'static str,
  /// HTML `type` attribute of the input.
  input_type: &'static str,
  /// Optional HTML `autocomplete` hint.
  #[prop(optional)]
  autocomplete: Option<&'static str>,
  /// Signal holding the current value.
  value: RwSignal<String>,
) -> impl IntoView {
  view! {
    <label class="field">
      <span>{label}</span>
      <input
        type=input_type
        autocomplete=autocomplete
        prop:value=move || value.get()
        on:input=move |ev| value.set(event_target_value(&ev))
      />
    </label>
  }
}

/// Registration form with client-side validation and status feedback.
#[component]
pub fn RegistrationForm() -> impl IntoView {
  let fields = FormFields::new();
  let status = RwSignal::new(FormStatus::Idle);

  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    if !fields.is_complete() {
      status.set(FormStatus::Error(
        "Merci de renseigner tous les champs.".to_string(),
      ));
      return;
    }

    status.set(FormStatus::Submitting);
    spawn_local(async move {
      status.set(send_registration(fields).await);
    });
  };

  view! {
    <section class="registration" id="inscription">
      <h2>"Inscription"</h2>
      <form on:submit=on_submit novalidate=true>
        <div class="field-row">
          <Field
            label="Prénom(s)"
            input_type="text"
            autocomplete="given-name"
            value=fields.first_names
          />
          <Field
            label="Nom(s)"
            input_type="text"
            autocomplete="family-name"
            value=fields.last_names
          />
        </div>

        <Field
          label="Email (universitaire ou personnel)"
          input_type="email"
          autocomplete="email"
          value=fields.email
        />

        <div class="field-row">
          <Field label="Téléphone" input_type="tel" autocomplete="tel" value=fields.phone />
          <Field label="Numéro étudiant" input_type="text" value=fields.student_number />
        </div>

        <button type="submit" disabled=move || matches!(status.get(), FormStatus::Submitting)>
          {move || {
            if matches!(status.get(), FormStatus::Submitting) {
              "Envoi en cours…"
            } else {
              "S'inscrire"
            }
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
