use cursive::Cursive;
use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
#[macro_use]
extern crate ferris_print;

struct State {
    reviews: Vec<u8>
}

fn profiency_prompt(cursive: &mut Cursive) {
    let count = cursive.user_data::<State>().unwrap().reviews.len() + 1;

    let mut select = SelectView::new();
    for i in 1..11 {
        select.add_item(i.to_string(), i.to_string());
    }
    select.set_on_submit(show_next_window);

    cursive.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10))).title(format!(
            "What's your confidence level on Chapter {}?",
            count
        )),
    );
}

fn main() {
    let result: State = {
        let mut cursive = Cursive::default();

        // Theme
        let mut palette = Palette::default();

        palette[PaletteColor::Background] = Color::Rgb(59, 104, 55);
        palette[PaletteColor::View] = Color::Rgb(247, 246, 230);
        palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Red);

        cursive.set_theme(Theme { shadow: true, borders: BorderStyle::Simple, palette });

        // Actual start
        cursive.set_user_data(State {
            reviews: Vec::new(),
        });

        profiency_prompt(&mut cursive);

        cursive.run();

       cursive.take_user_data().unwrap()
    };

    if (result.reviews.len() > 0) {
        println!("{} is your ranking for the first chapter!", result.reviews.get(0).unwrap());
    } else {
        ferrisprint!("Come back soon! :D");
    }

}

fn show_next_window(cursive: &mut Cursive, result: &str) {
    cursive.pop_layer();

    let mut chapters_reviewed = 0;

    cursive.with_user_data(|state: &mut State| {
        state.reviews.push(result.parse().unwrap());
        chapters_reviewed = state.reviews.len();
    });

    if chapters_reviewed != 10 {
        cursive.add_layer(
            Dialog::text(format!("{} is pretty good for a beginner!", result))
                .button("Continue", |button_cursive| {
                    profiency_prompt(button_cursive);
                })
                .button("Quit", |s| s.quit()),
        );
    } else {
        let final_state = cursive.user_data::<State>().unwrap();

        let mut select = SelectView::new();

        for (chap_num, score) in final_state.reviews.iter().enumerate() {
            select.add_item(format!("Chapter {}: {}/10", chap_num + 1, score), "");
        }

        cursive.add_layer(
            Dialog::around(select).title("Here's your breakdown of each Chapter!")
                .button("Next Cycle", |button_cursive| {}),
        );
    }
}
