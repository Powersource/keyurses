extern crate cursive;
extern crate rand;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, LinearLayout};
use cursive::traits::Identifiable;
use rand::Rng;

fn main() {
    // This really messes with stdout. Seems to disable it by default but when
    // siv is running println prints in random places on the screen.
    let mut siv = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());

    let words = Words::new();
    

    siv.add_layer(Dialog::around(LinearLayout::vertical()
                .child(TextView::new("testtext").with_id("word_field"))
                .child(EditView::new().on_edit(move |s, _, _| {
                    s.find_id::<TextView>("word_field").unwrap().set_content(words.rand_word())
                })))
            .title("Keyurses")
            .button("Quit", |s| s.quit()));

    siv.run();
}

#[derive(Debug)]
struct Words {
    list: Vec<&'static str>,
}

impl Words {
    fn new() -> Self {
        Words { list: include_str!("google-10000-english-usa.txt").lines().collect() }
    }

    fn rand_word(&self) -> &str {
        let mut rng = rand::thread_rng();
        rng.choose(&self.list).unwrap()
    }
}