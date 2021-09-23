use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use glib::clone;
use std::{cell::RefCell, rc::Rc};
use rand::{thread_rng, Rng};

fn main() {
    let app = gtk::Application::new(
        Some("com.orkunmanap.hangman_gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK.");

    app.connect_activate(|app| {
        let glade_src = include_str!("../layout.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window: gtk::Window = builder.get_object("application_window").unwrap();
        window.set_application(Some(app));

        // Get UI elements
        let score_label: gtk::Label = builder.get_object("score_label").unwrap();
        let point_label: gtk::Label = builder.get_object("score_point").unwrap();
        let word_label: gtk::Label = builder.get_object("word_label").unwrap();
        let check_button: gtk::Button = builder.get_object("check_button").unwrap();
        let text_entry: gtk::Entry = builder.get_object("text_entry").unwrap();
        let hangman_image: gtk::Image = builder.get_object("hangman_image").unwrap();
        // let hangman_image_clone = hangman_image.clone();

        // Define word list
        /*
        let mut word_list = vec![
            "opposed",
            "consider",
            "relationship",
            "tired",
            "flag",
            "peasant",
            "relate",
            "loop",
            "tragedy",
            "wonder",
        ];
        */
        let x = Rc::new(RefCell::new(vec![
            "opposed",
            "consider",
            "relationship",
            "tired",
            "flag",
            "peasant",
            "relate",
            "loop",
            "tragedy",
            "wonder",
        ]));

        //Get the first word from vector
        //word_label.set_text(return_word(word_list.as_mut_slice()));
        let mut rng = rand::thread_rng();
        word_label.set_text(x.borrow()[rng.gen_range(0..10)]);

        check_button.connect_clicked(clone!(@strong x => move |_| {
            //let test = remove_word(word_list.as_mut_slice());

            // Check if there is any word left in vector
            if x.borrow().len() == 0 {
                score_label.set_text("YOU WON!");
                text_entry.hide();
            } else {
                let message_gstring = text_entry.get_text();
                let message = if message_gstring.as_str().is_empty() {
                    "Please enter a guess!"
                } else {
                    message_gstring.as_str()
                };

                if message_gstring == word_label.get_text() {
                    let current_score = point_label.get_text().parse::<usize>().unwrap();

                    point_label.set_label(
                        &increase_score(current_score).to_string(),
                    );

                    // Get a new word, if there is no word left in vector, it will return "q"
                    //let new_word = return_word(word_list.as_mut_slice());
                    //word_label.set_text(new_word);
                    let mut rng = rand::prelude::ThreadRng();
                    let mut a = x.borrow_mut().clone();
                    rng.shuffle(&mut a);

                    a.remove(
                        a.iter()
                            .position(|x| *x == word_label.get_text())
                            .unwrap(),
                    );

                    let mut rng = rand::thread_rng();
                    word_label.set_text(x.borrow()[rng.gen_range(0..10-current_score/10)]);

                    hangman_image.set_from_file("./images/hangmanAlive.png");

                    /*
                                        let mut rng = rand::thread_rng();
                                        world_list_clone.remove(
                                            world_list_clone
                                                .iter()
                                                .position(|x| *x == word_label.get_text())
                                                .unwrap(),
                                        );

                                        for i in &world_list_clone {
                                            println!("{}, {}", i, world_list_clone.len())
                                        }
                    */
                } else {
                    hangman_image.set_from_file("./images/hangmanDead.png");
                }
                text_entry.set_placeholder_text(Some(message));
            }
        }));

        window.show_all();
    });
    app.run(&env::args().collect::<Vec<_>>());
}

fn increase_score(point: usize) -> usize {
    point + 10
}

/*
fn return_word<'a>(word_list: &'a mut [&str]) -> &'a str {
    let mut rng = rand::thread_rng();
    let word = word_list[rng.gen_range(0..10)];

    if word_list.len() == 0 {
        return " ";
    } else {
        //word_list.remove(word_list.iter().position(|x| *x == word).unwrap());
        word
    }
}
*/
/*
fn remove_word<'a>(word_list: &'a mut [&str]) -> &'a mut [&'a str] {
    let mut clone = word_list.clone();
    let mut rng = rand::thread_rng();

    let len_list = clone.len();

    let word = clone[rng.gen_range(0..len_list)];
    clone.remove(clone.iter().position(|x| *x == word).unwrap());
    clone
}
*/
