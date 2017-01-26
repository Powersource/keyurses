extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Dialog::around(TextView::new("Hello World"))
    .title("Cursive")
    .button("Quit", |s| s.quit()));

    siv.run();
}
