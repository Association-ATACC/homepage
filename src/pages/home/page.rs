//! View of the home page.

use leptos::prelude::*;

use super::components::{Header, RegistrationForm, WeeklySchedule};

/// Home page: header, registration form, and the weekly schedule.
#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <Header />
    <RegistrationForm />
    <WeeklySchedule />
  }
}
