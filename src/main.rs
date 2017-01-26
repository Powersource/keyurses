extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, LinearLayout};

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new("testtext"))
            .child(EditView::new()))
        .title("Cursive")
        .button("Quit", |s| s.quit()));

    siv.run();
}
