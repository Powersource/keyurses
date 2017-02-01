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
            .child(TextView::new(words.rand_word()).with_id("target_field"))
            .child(EditView::new()
                .on_edit(move |s, input, _| typed_some(s, input, &words))
                .with_id("input_field")))
        .title("Keyurses")
        .button("Quit", |s| s.quit()));

    siv.run();
}

fn typed_some(siv: &mut Cursive, input: &str, words: &Words) {
    // See https://github.com/gyscos/Cursive/issues/102
    // for discussion on this mess

    let mut reset_input = false;
    {
       let target_word = siv.find_id::<TextView>("target_field").unwrap();
       if target_word.get_content() == input {
           target_word.set_content(words.rand_word());
           reset_input = true;
       }
    }
    if reset_input {
       siv.find_id::<EditView>("input_field").unwrap().set_content("");
    }
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