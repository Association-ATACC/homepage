use leptos::prelude::*;

/// A single tutoring slot in the weekly schedule.
///
/// `jour` is one of the values in [`JOURS`]. This type (and
/// [`creneaux_actuels`] below) is intentionally the *only* place that needs
/// to change once the schedule becomes dynamic: swap the body of
/// `creneaux_actuels` for a call to a `#[server]` function backed by the
/// database, and this component keeps working as-is.
#[derive(Clone)]
struct Creneau {
    jour: &'static str,
    debut: &'static str,
    fin: &'static str,
    matiere: String,
}

const JOURS: [&str; 7] = [
    "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
];

/// Returns the slots to display. Empty for now — no tutoring slots have
/// been scheduled yet — so this is a plain static list. Replace this with
/// a database-backed lookup once slots exist.
fn creneaux_actuels() -> Vec<Creneau> {
    vec![
        Creneau {
            jour: "Mardi",
            debut: "11h15",
            fin: "11h30",
            matiere: "Présentation de l'ATACC".to_owned(),
        },
        Creneau {
            jour: "Vendredi",
            debut: "14h00",
            fin: "18h00",
            matiere: "Permanance".to_owned(),
        },
    ]
}

#[component]
pub fn WeeklySchedule() -> impl IntoView {
    let creneaux = creneaux_actuels();
    let has_creneaux = !creneaux.is_empty();

    view! {
        <section class="schedule" id="emploi-du-temps">
            <h2>"Emploi du temps"</h2>
            <Show
                when=move || has_creneaux
                fallback=|| view! {
                    <p class="schedule-empty">
                        "Aucun créneau n'est publié pour l'instant. Les prochaines séances de tutorat apparaîtront ici dès qu'elles seront programmées."
                    </p>
                }
            >
                <div class="schedule-grid">
                    {JOURS.iter().map(|jour| {
                        let slots: Vec<Creneau> = creneaux
                            .iter()
                            .filter(|c| c.jour == *jour)
                            .cloned()
                            .collect();

                        view! {
                            <div class="schedule-day">
                                <h3>{*jour}</h3>
                                <div class="schedule-slots">
                                    {slots.into_iter().map(|c| view! {
                                        <div class="schedule-slot">
                                            <span class="schedule-time">{c.debut}" – "{c.fin}</span>
                                            <span class="schedule-subject">{c.matiere.clone()}</span>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </Show>
        </section>
    }
}
