//! Weekly schedule component: lists the tutoring slots of the week, grouped
//! by day, or an explanatory message when no slot is published.

use leptos::prelude::*;

/// Day of the week.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Day {
  /// Monday.
  Monday,
  /// Tuesday.
  Tuesday,
  /// Wednesday.
  Wednesday,
  /// Thursday.
  Thursday,
  /// Friday.
  Friday,
  /// Saturday.
  Saturday,
  /// Sunday.
  Sunday,
}

impl Day {
  /// French label displayed in the schedule.
  const fn label(self) -> &'static str {
    match self {
      Self::Monday => "Lundi",
      Self::Tuesday => "Mardi",
      Self::Wednesday => "Mercredi",
      Self::Thursday => "Jeudi",
      Self::Friday => "Vendredi",
      Self::Saturday => "Samedi",
      Self::Sunday => "Dimanche",
    }
  }
}

/// Days of the week, in display order.
const DAYS: [Day; 7] = [
  Day::Monday,
  Day::Tuesday,
  Day::Wednesday,
  Day::Thursday,
  Day::Friday,
  Day::Saturday,
  Day::Sunday,
];

/// A single tutoring slot in the weekly schedule.
#[derive(Clone)]
struct Slot {
  /// Day of the week on which the slot takes place.
  day: Day,
  /// Start time, formatted for display (e.g. `"14h00"`).
  start: &'static str,
  /// End time, formatted for display (e.g. `"18h00"`).
  end: &'static str,
  /// Subject or title of the slot.
  subject: String,
}

/// Returns the slots to display.
fn current_slots() -> Vec<Slot> {
  vec![
    Slot {
      day: Day::Tuesday,
      start: "11h15",
      end: "11h30",
      subject: "Présentation de l'ATACC".to_owned(),
    },
    Slot {
      day: Day::Thursday,
      start: "14h00",
      end: "18h00",
      subject: "Permanence".to_owned(),
    },
  ]
}

/// Weekly schedule grid, one column per day, with a fallback message when
/// there are no slots.
#[component]
pub fn WeeklySchedule() -> impl IntoView {
  let slots = current_slots();
  let has_slots = !slots.is_empty();

  view! {
    <section class="schedule" id="schedule">
      <h2>"Emploi du temps"</h2>
      <Show
        when=move || has_slots
        fallback=|| {
          view! {
            <p class="schedule-empty">
              "Aucun créneau n'est publié pour l'instant. Les prochaines séances de tutorat apparaîtront ici dès qu'elles seront programmées."
            </p>
          }
        }
      >
        <div class="schedule-grid">
          {DAYS
            .into_iter()
            .map(|day| {
              let day_slots = slots.iter().filter(|slot| slot.day == day).cloned();

              view! {
                <div class="schedule-day">
                  <h3>{day.label()}</h3>
                  <div class="schedule-slots">
                    {day_slots
                      .map(|slot| {
                        view! {
                          <div class="schedule-slot">
                            <span class="schedule-time">{slot.start}" – "{slot.end}</span>
                            <span class="schedule-subject">{slot.subject}</span>
                          </div>
                        }
                      })
                      .collect::<Vec<_>>()}
                  </div>
                </div>
              }
            })
            .collect::<Vec<_>>()}
        </div>
      </Show>
    </section>
  }
}
