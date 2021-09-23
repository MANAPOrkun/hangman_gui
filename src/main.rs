use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use glib::clone;
use std::{cell::Cell, rc::Rc};
use rand::Rng;
use glib::clone::Downgrade;

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
        let letter_box: gtk::ComboBox = builder.get_object("letter_box").unwrap();
        // let hangman_image_clone = hangman_image.clone();

        // Define score
        let score = Rc::new(Cell::new(0));

        // Define number of taking letter
        let give_letter = Rc::new(Cell::new(3));

        // Define word list
        let word_list:Vec<String> = vec![
            "opposed".to_string(),
            "consider".to_string(),
            "relationship".to_string(),
            "tired".to_string(),
            "flag".to_string(),
            "peasant".to_string(),
            "relate".to_string(),
            "loop".to_string(),
            "tragedy".to_string(),
            "wonder".to_string(),
        ];
        
        let mut test = Word{
            word_list,
        };

        let a = &test.get_word();
        let selected_word = a[0].clone();
        let copy_word = a[1].clone();

        

        // Set word label as selected word
        word_label.set_text(&selected_word);

        check_button.connect_clicked(clone!(@strong score, @strong selected_word, @strong copy_word => move |_| {

            // Check if there is any word left in vector
            if 0 == 1 {
                score_label.set_text("YOU WON!");
                text_entry.hide();
            } else {
                let message_gstring = text_entry.get_text();

                if message_gstring.as_str().is_empty() {
                    text_entry.set_placeholder_text(Some("Please enter a guess!"));
                } else {
                    if message_gstring == copy_word {
                        score.set(score.get() + 10);
    
                        point_label.set_label(&score.get().to_string());
    
                        //let mut rng = rand::thread_rng();
                        //word_label.set_text(&word_list[rng.gen_range(0..word_list.len())]);
    
                        let words = &test.get_word();
                        selected_word. = words[0].clone();
                        let copy_word = words[1].clone();

                        word_label.set_text(&selected_word.clone());

                        hangman_image.set_from_file("./images/hangmanAlive.png");
                    } else {
                        hangman_image.set_from_file("./images/hangmanDead.png");
                        text_entry.set_text("");
                        text_entry.set_placeholder_text(Some("Try again!"));
                    }
                };
            }
        }));

        window.show_all();
    });
    app.run(&env::args().collect::<Vec<_>>());
}

struct Word {
    word_list: Vec<String>,
}

impl Word {
    pub fn get_word(&self) -> [String; 2]{
        //Get the first word from vector
        let mut rng = rand::thread_rng();
        let mut selected_word = self.word_list[rng.gen_range(0..10)].clone();
        let copy_word = selected_word.clone();
        let word_length = selected_word.len();
        

        // Number of missing letters from a word
        let mut missing_letter_number:usize = rng.gen_range(1..word_length - 1);

        println!("selected_word = {}, missing_letter_number = {}, word_len = {}", 
            selected_word, missing_letter_number, selected_word.len());
        
        // Replace random characters with _
        while missing_letter_number > 0 {
            // Get random number between 1 and length of the selected word
            let random_number = rng.gen_range(1..word_length);

            // Replace character which in random_number - 1, random_number range
            selected_word.replace_range(random_number - 1..random_number, "_");
            missing_letter_number = missing_letter_number - 1;
        }
        
        // Return changed and original word
        [selected_word, copy_word] 
    }
}
