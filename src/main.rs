use cursive::menu::MenuTree;
use cursive::views::Dialog;
use cursive::{Cursive, With};
use rustacean_review::screens::{
    chapter_choice, recommend_review_screen, search_screen, summary_screen,
};
use rustacean_review::state::deserialize_state;
use rustacean_review::{State, CHAPTERS};
use std::collections::HashMap;

#[macro_use]
extern crate ferris_print;

fn main() {
    let result: State = {
        let mut cursive = Cursive::default();

        /*
        let mut palette = Palette::default();

        palette[PaletteColor::Background] = Color::Rgb(59, 104, 55);
        palette[PaletteColor::View] = Color::Rgb(247, 246, 230);
        palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Red);

        cursive.set_theme(Theme { shadow: true, borders: BorderStyle::Simple, palette });
        */

        // Attempt to deserialize from state file if exists
        match deserialize_state() {
            Ok(state) => {
                cursive.set_user_data(state);
            }
            Err(_) => {
                // If this is first run, create newly minted vector
                cursive.set_user_data(State {
                    reviews: HashMap::new(),
                });
            }
        }

        cursive
            .menubar()
            .add_subtree(
                "Chapters",
                MenuTree::new()
                    .leaf("View Summary", |cursive| {
                        summary_screen(cursive);
                    })
                    .leaf("Recommend Review", |cursive| {
                        recommend_review_screen(cursive);
                    })
                    .leaf("Search for Chapter", |cursive| {
                        search_screen(cursive);
                    })
                    .delimiter()
                    .subtree(
                        "Chapters",
                        MenuTree::new().with(|tree| {
                            for (index, chapter) in CHAPTERS.iter().enumerate() {
                                tree.add_leaf(chapter.human_readable, move |cursive| {
                                    chapter_choice(cursive, index)
                                });
                            }
                        }),
                    ),
            )
            .add_delimiter()
            .add_leaf("About", |cursive| {
                cursive.add_layer(Dialog::info("A t11s project. First Rust project."))
            })
            .add_leaf("Quit", |cursive| cursive.quit());

        cursive.set_autohide_menu(false);

        cursive.add_layer(Dialog::text("Click on the menubar!"));

        cursive.run();

        // On Quit
        cursive.take_user_data().unwrap()
    };

    // After cursive event loop closes
    ferrisprint!("Come back soon! :D");
}
