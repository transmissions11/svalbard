//! Has functions to display different screens to the user of the cursive TUI.
use crate::state::serialize_state;
use crate::{find_chapter_index, smallest_value_in_hashmap, State, CHAPTERS};
use cursive::traits::{Boxable, Scrollable};
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

/// Lets a user choice an action to perform on a given chapter: View study resources or review the chapter
pub fn chapter_choice(cursive: &mut Cursive, chapter_num: usize) {
    cursive.add_layer(
        Dialog::text("What would you like to do?")
            .button("Review Chapter", move |cursive| {
                cursive.pop_layer();
                review_chapter_screen(cursive, chapter_num);
            })
            .button("Study Resources", move |cursive| {
                cursive.pop_layer();
                study_resources_screen(cursive, chapter_num);
            }),
    )
}
/// Shows a screen
pub fn review_chapter_screen(cursive: &mut Cursive, chapter_num: usize) {
    let mut select = SelectView::new();
    for i in 1..11 {
        select.add_item(i.to_string(), i.to_string());
    }

    select.set_on_submit(move |cursive, result: &str| {
        cursive.pop_layer();

        let state = cursive.user_data::<State>().unwrap();

        state.reviews.insert(chapter_num, result.parse().unwrap());
        serialize_state(state);
    });

    cursive.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10))).title(format!(
            "What's your confidence level on Chapter {}?",
            CHAPTERS[chapter_num].human_readable
        )),
    );
}

/// Shows all the reviews the user has left for each chapter
pub fn summary_screen(cursive: &mut Cursive) {
    let state = cursive.user_data::<State>().unwrap();

    let mut select = SelectView::new();

    for (index, chapter) in CHAPTERS.iter().enumerate() {
        if let Some(review) = state.reviews.get(&index) {
            select.add_item(format!("{}: {}/10", chapter.human_readable, review), index);
        }
    }
    cursive.add_layer(
        Dialog::around(select.on_submit(|cursive, chapter_index| {
            cursive.pop_layer();
            chapter_choice(cursive, *chapter_index);
        }))
        .title("Here's a summary of your learning:")
        .button("Ok", |button_cursive| {
            button_cursive.pop_layer();
        }),
    );
}

/// Shows a screen recommending which chapter to review.
pub fn recommend_review_screen(cursive: &mut Cursive) {
    let state = cursive.user_data::<State>().unwrap();

    if let Some(recommended_chapter_index) = smallest_value_in_hashmap(&state.reviews) {
        cursive.add_layer(
            Dialog::info(format!(
                "You should brush up on Chapter {}",
                &CHAPTERS[recommended_chapter_index].human_readable
            ))
            .button("Study Resources", move |cursive| {
                cursive.pop_layer();
                study_resources_screen(cursive, recommended_chapter_index);
            }),
        )
    } else {
        cursive.add_layer(Dialog::info("Review some chapters first!"));
    }
}

pub fn study_resources_screen(cursive: &mut Cursive, chap_num: usize) {
    let recommended_chapter = &CHAPTERS[chap_num];
    cursive.add_layer(Dialog::info(
        recommended_chapter.chapter_resources_string(),
    ));
}

/// Shows a prompt where user can type in the name of a chapter. Will redirect user to `profiency_prompt()` if successful. Will not panic if no results are found, shows dialog instead.
pub fn search_screen(cursive: &mut Cursive) {
    cursive.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(|cursive, result| {
                    cursive.pop_layer();

                    if let Some(chapter_num) = find_chapter_index(result, CHAPTERS) {
                        chapter_choice(cursive, chapter_num)
                    } else {
                        cursive.add_layer(Dialog::info("No results found."));
                    }
                })
                .fixed_width(20),
        )
        .button("Ok", |cursive| {
            cursive.pop_layer();
        })
        .title("Which chapter do you want to edit?"),
    );
}
