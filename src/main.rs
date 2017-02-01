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

    let mut words = WordBar::new();

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new(words.update_and_get_bar()).with_id("target_field"))
            .child(EditView::new()
                .on_edit(move |s, input, _| words.typed_some(s, input))
                .with_id("input_field")))
        .title("Keyurses")
        .button("Quit", |s| s.quit()));

    siv.run();
}


type WordList = Vec<&'static str>;

#[derive(Debug)]
struct WordBar {
    words: WordList,
    target_list: WordList,
}

impl WordBar {
    fn new() -> Self {
        WordBar {
            words: include_str!("google-10000-english-usa.txt").lines().collect(),
            target_list: vec!["foo"],
        }
    }

    fn typed_some(&mut self, siv: &mut Cursive, input: &str) {
        // See https://github.com/gyscos/Cursive/issues/102
        // for discussion on this mess

        let mut reset_input = false;
        {
            let target_word = siv.find_id::<TextView>("target_field").unwrap();
            if target_word.get_content() == input {
                target_word.set_content(self.update_and_get_bar());
                reset_input = true;
            }
        }
        if reset_input {
            siv.find_id::<EditView>("input_field").unwrap().set_content("");
        }
    }

    fn rand_word(&self) -> &'static str {
        let mut rng = rand::thread_rng();
        rng.choose(&self.words).unwrap()
    }

    fn update_and_get_bar(&mut self) -> String {
        if self.target_list.len() > 0 {
            self.target_list.pop();
        }
        while self.target_list.len() < 5 {
            let new_word = self.rand_word();
            self.target_list.push(new_word);
        }
        let mut bar_text: String = "".to_string();
        for word in &self.target_list {
            if bar_text == "" {
                bar_text = word.to_string();
            } else {
                bar_text.push_str(" ");
                bar_text.push_str(word);
            }
        }
        bar_text
    }
}