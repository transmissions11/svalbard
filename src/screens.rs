//! Has functions to display different screens to the user of the cursive TUI.
use crate::state::serialize_state;
use crate::{find_chapter_index, State, CHAPTERS, smallest_value_in_hashmap};
use cursive::traits::{Boxable, Scrollable};
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

/// Asks a user to rate their confidence on a specific chapter
pub fn proficiency_prompt(cursive: &mut Cursive, chapter_num: usize) {
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
            CHAPTERS[chapter_num]
        )),
    );
}

/// Shows all the reviews the user has left for each chapter
pub fn summary_screen(cursive: &mut Cursive) {
    let state = cursive.user_data::<State>().unwrap();

    let mut select = SelectView::new();

    for (index, chapter) in CHAPTERS.iter().enumerate() {
        if let Some(review) = state.reviews.get(&index) {
            select.add_item(format!("{}: {}/10", chapter, review), "");
        }
    }

    cursive.add_layer(
        Dialog::around(select)
            .title("Here's a summary of your learning:")
            .button("Ok", |button_cursive| {
                button_cursive.pop_layer();
            }),
    );
}

/// Shows a screen recommending which chapter to review.
pub fn recommend_review(cursive: &mut Cursive) {
    let state = cursive.user_data::<State>().unwrap();

    if let Some(recommended_chapter_index) = smallest_value_in_hashmap(&state.reviews) {
        let recommended_chapter_name = CHAPTERS[recommended_chapter_index];

        cursive.add_layer(Dialog::info(format!(
            "You should brush up on Chapter {}\n\nMake sure to look at the Rust by Example online book for some examples!",
            recommended_chapter_name
        )));
    } else {
        cursive.add_layer(Dialog::info("Review some chapters first!"));
    }
}

/// Shows a prompt where user can type in the name of a chapter. Will redirect user to `profiency_prompt()` if successful. Will not panic if no results are found, shows dialog instead.
pub fn search_screen(cursive: &mut Cursive) {
    cursive.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(|cursive, result| {
                    cursive.pop_layer();

                    if let Some(chapter_num) = find_chapter_index(result, CHAPTERS) {
                        proficiency_prompt(cursive, chapter_num)
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
