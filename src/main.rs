use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use glib::clone;
use std::{cell::Cell, rc::Rc, cell::RefCell};
use rand::Rng;

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
        let letter_count: gtk::Label = builder.get_object("count_label").unwrap();
        let health_count: gtk::Label = builder.get_object("health_label").unwrap();
        let word_label_clone = word_label.clone();
        let letter_count_clone = letter_count.clone();


        // Define score
        let score = Rc::new(Cell::new(0));

        // Define number of taking letter
        let give_letter_number = Rc::new(Cell::new(4));
        let give_letter_number_clone = give_letter_number.clone();

        // Define health
        let health_point = Rc::new(Cell::new(4));

        // Define word list
        let word_list = vec![
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
            "buff".to_string(),
            "dizzying".to_string(),
            "vodka".to_string(),
            "cozy".to_string(),
            "bikini".to_string(),
            "dwarves".to_string(),
            "kiosk".to_string(),
            "buzzing".to_string(),
            "klutz".to_string(),
            "swivel".to_string(),
            "vex".to_string(),
            "embezzle".to_string(),
            "avenue".to_string(),
            "disavow".to_string(),
            "keyhole".to_string(),
            "voodoo".to_string(),
            "razzmatazz".to_string(),
            "curacao".to_string(),
            "gabby".to_string(),
            "bookworm".to_string(),
        ];

        let word_count = word_list.len();

        let test = Word {
            word_list,
        };

        let words = Rc::new(RefCell::new(test.get_word(word_count)));
        let words_borrowed = words.clone();
        let words_borrowed_clone = words_borrowed.clone();
    
        // Set word label as selected word
        word_label.set_text(&words.borrow()[0]);

        check_button.connect_clicked(clone!(@strong score, @strong health_point,@strong give_letter_number_clone  => move |check_button| {

            // If there is no health left, reset the application
            if health_point.get() == 0 {
                score.set(0);
                health_point.set(4);
                give_letter_number_clone.set(4);

                hangman_image.set_from_file("./images/hangmanAlive.png");
                
                text_entry.set_text("");
                text_entry.set_placeholder_text(Some("Enter your guess!"));

                point_label.set_label(&score.get().to_string());
                letter_count_clone.set_text(&give_letter_number_clone.get().to_string());
                health_count.set_text(&health_point.get().to_string());

                *words_borrowed.borrow_mut() = test.get_word(word_count); 
                word_label.set_text(&words_borrowed.borrow()[0]);

                check_button.set_label("Check");
                text_entry.show();

            } else {
                // Check if the score reached, if so, reset the application
                if score.get()/10 == 20{
                    score.set(0);
                    health_point.set(4);
                    give_letter_number_clone.set(4);

                    hangman_image.set_from_file("./images/hangmanAlive.png");
                    
                    text_entry.set_text("");
                    text_entry.set_placeholder_text(Some("Enter your guess!"));

                    point_label.set_label(&score.get().to_string());
                    letter_count_clone.set_text(&give_letter_number_clone.get().to_string());
                    health_count.set_text(&health_point.get().to_string());

                    *words_borrowed.borrow_mut() = test.get_word(word_count); 
                    word_label.set_text(&words_borrowed.borrow()[0]);

                    check_button.set_label("Check");
                    text_entry.show();
                } else {
                    // Get the text in entry
                    let message_gstring = text_entry.get_text();

                    // Check if it is empty
                    if message_gstring.as_str().is_empty() {
                        text_entry.set_placeholder_text(Some("Please enter a guess!"));
                    } else {

                        // Check if the text in entry matches with the word
                        if &message_gstring.to_string() == &words_borrowed.borrow()[1] {
                            score.set(score.get() + 10);

                            point_label.set_label(&score.get().to_string());

                            // Get a new word from the list
                            *words_borrowed.borrow_mut() = test.get_word(word_count); 
                            word_label.set_text(&words_borrowed.borrow()[0]);

                            // Reset the entry
                            text_entry.set_text("");
                            text_entry.set_placeholder_text(Some("Enter your guess!"));

                            hangman_image.set_from_file("./images/hangmanAlive.png");

                            // Check if the score reached, if so, change the UI
                            if score.get()/10 == 20{
                                hangman_image.set_from_file("./images/hangmanAlive.png");
                                score_label.set_text("YOU WON!");
                                word_label.set_text("Press the button to restart");
                                check_button.set_label("Restart");
                                text_entry.hide();
                            }

                        } else {

                            // In case of wrong answer, decrease the health
                            health_point.set(health_point.get() - 1);
                            health_count.set_text(&health_point.get().to_string());

                            // Reset the entry
                            text_entry.set_text("");
                            text_entry.set_placeholder_text(Some("Try again!"));

                            // If health has reached to 0, change the UI
                            if health_point.get() == 0 {
                                hangman_image.set_from_file("./images/hangmanDead.png");
                                score_label.set_text("YOU LOSE!");
                                word_label.set_text("Press the button to restart");
                                check_button.set_label("Restart");
                                text_entry.hide();
                            }
                        }
                    };
                }
            }
            
        }));

        letter_box.connect_changed(clone!(@strong give_letter_number => move |letter_box| {

            // Check if there is a letter left
            if give_letter_number.get() > 0 {
                let tree_iter = letter_box.get_active_iter().unwrap();
                let model = letter_box.get_model().unwrap();

                // Create the alphabet
                let selection = match &*model.get_string_from_iter(&tree_iter).unwrap().to_string() {
                    "0" => "a",
                    "1" => "b",
                    "2" => "c",
                    "3" => "d",
                    "4" => "e",
                    "5" => "f",
                    "6" => "g",
                    "7" => "h",
                    "8" => "i",
                    "9" => "j",
                    "10" => "k",
                    "11" => "l",
                    "12" => "m",
                    "13" => "n",
                    "14" => "o",
                    "15" => "p",
                    "16" => "q",
                    "17" => "r",
                    "18" => "s",
                    "19" => "t",
                    "20" => "u",
                    "21" => "v",
                    "22" => "w",
                    "23" => "x",
                    "24" => "y",
                    "25" => "z",
                    _ => "",
                };
    
                // Get the original word which is not with '_' character
                let original_word = &words_borrowed_clone.borrow()[1];

                // Get the word that showed on the application
                let mut showed_word = word_label_clone.get_text().to_string();
    
                let mut changed = false;
    
                for (i, letter) in original_word.char_indices() {
                    // Check if the word includes the letter that selected by user 
                    //if letter.to_string().eq(selection) { -- Not sure if this is more efficient or not
                    if showed_word.chars().nth(i).unwrap().to_string().eq("_") && letter.to_string().eq(selection) {
                        showed_word.replace_range(i..(i+1), selection);
                        changed = true;
                    }
                }
    
                // If any '_' letter found, change
                if changed {
                    word_label_clone.set_text(&showed_word);
                } else {
                    println!("'{}' could not found in '{}'", selection, original_word);  
                }
    
                // Decrease the letter number
                give_letter_number.set(give_letter_number.get() - 1);
                letter_count.set_text(&give_letter_number.get().to_string());
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
    pub fn get_word(&self, word_list_len: usize) -> [String; 2]{
        //Get the first word from vector
        let mut rng = rand::thread_rng();
        let mut selected_word = self.word_list[rng.gen_range(0..word_list_len)].clone();
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
