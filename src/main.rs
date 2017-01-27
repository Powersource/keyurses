extern crate cursive;
extern crate rand;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, LinearLayout};
use cursive::traits::Identifiable;
use cursive::view::Finder;
use rand::Rng;

fn main() {
    // This really messes with stdout. Seems to disable it by default but when
    // siv is running println prints in random places on the screen.
    let mut siv = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());

    let words = Words::new();

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new("testtext").with_id("target_field"))
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

    // This one would be optimal but pisses off the borrow checker about siv
    /*let target_word = siv.find_id::<TextView>("target_field").unwrap();
    let input_word = siv.find_id::<EditView>("input_field").unwrap();
    if target_word.get_content() == input {
        target_word.set_content(words.rand_word());
        input_word.set_content("");
    }*/
    
    // This one doesn't crash but is ugly
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

    // This one crashes due to the last find_id not finding anything. Why?
    // Does it only look for children?
    /*let target_word = siv.find_id::<TextView>("target_field").unwrap();
    if target_word.get_content() == input {
        target_word.set_content(words.rand_word());
        target_word.find_id::<EditView>("input_field")
            .unwrap()
            .set_content("");
    }*/

    // This one works but is verbose
    /*if siv.find_id::<TextView>("target_field").unwrap().get_content() == input {
        siv.find_id::<TextView>("target_field").unwrap().set_content(words.rand_word());
        siv.find_id::<EditView>("input_field")
            .unwrap()
            .set_content("");
    }*/

    // This one feels reasonable but TextView doesn't implement content() (EditView does.
    // Intentional?)
    /*let target_word = siv.find_id::<TextView>("target_field").unwrap();
    if target_word.get_content() == input {
        target_word.content(words.rand_word())
            .find_id::<EditView>("input_field")
            .unwrap()
            .set_content("");
    }*/
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