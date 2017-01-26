extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, LinearLayout};
use cursive::traits::Identifiable;

fn main() {
    let mut siv = Cursive::new();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new("testtext").with_id("word_field"))
            .child(EditView::new().on_edit(|s, text, _| {
                s.find_id::<TextView>("word_field").unwrap().set_content(text)
            })))
        .title("Keyurses")
        .button("Quit", |s| s.quit()));

    siv.run();
}
