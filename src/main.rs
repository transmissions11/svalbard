use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::view::IntoBoxedView;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

struct State {
    chaptersReviewed: u32,
}

fn profiencyPrompt(cursive: &mut Cursive) {
    let value = cursive.user_data::<State>().unwrap().chaptersReviewed;

    let mut select = SelectView::new();
    for i in 1..10 {
        select.add_item(i.to_string(), i.to_string());
    }
    select.set_on_submit(show_next_window);

    cursive.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10))).title(format!(
            "What's your confidence level on the Chapter {}?",
            value
        )),
    );
}

fn main() {
    let mut cursive = Cursive::default();

    cursive.set_user_data(State {
        chaptersReviewed: 1,
    });

    profiencyPrompt(&mut cursive);

    cursive.run();
}

fn show_next_window(cursive: &mut Cursive, result: &str) {
    cursive.pop_layer();

    let text = format!("{} is pretty good for a beginner!", result);

    let value = cursive.user_data::<State>().unwrap();

    value.chaptersReviewed += 1;

    let text = format!("{} is pretty good for a beginner!", value.chaptersReviewed);

    if value.chaptersReviewed == 10 {

        cursive.add_layer(Dialog::text("Ayy!"));
    } else {
        cursive.add_layer(
            Dialog::text(text)
                .button("Continue", |button_cursive| {
                    profiencyPrompt(button_cursive);
                })
                .button("Quit", |s| s.quit()),
        );
    }
}
