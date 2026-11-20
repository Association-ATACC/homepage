//! Building blocks of the home page: the header, the registration form and the
//! weekly schedule.

mod header;
mod registration_form;
mod weekly_schedule;

pub use header::Header;
pub use registration_form::RegistrationForm;
pub use weekly_schedule::WeeklySchedule;
