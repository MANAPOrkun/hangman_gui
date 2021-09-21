use gio::prelude::*;
use gtk::prelude::*;
use rand::Rng;
use std::env;

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

        // let score_label: gtk::Label = builder.get_object("score_label").unwrap();
        let point_label: gtk::Label = builder.get_object("score_point").unwrap();
        let word_label: gtk::Label = builder.get_object("word_label").unwrap();
        let check_button: gtk::Button = builder.get_object("check_button").unwrap();
        let text_entry: gtk::Entry = builder.get_object("text_entry").unwrap();
        let hangman_image: gtk::Image = builder.get_object("hangman_image").unwrap();
        let hangman_image_clone = hangman_image.clone();

        let word_list = vec![
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

        let mut rng = rand::thread_rng();
        word_label.set_text(&word_list[rng.gen_range(0..10)]);

        check_button.connect_clicked(move |_| {
            let mut point = point_label.get_text().parse::<u32>().unwrap();
            let mut world_list_clone = word_list.clone();

            let message_gstring = text_entry.get_text();
            let message = if message_gstring.as_str().is_empty() {
                "Please enter a guess!"
            } else {
                message_gstring.as_str()
            };

            if message_gstring == word_label.get_text() {
                point = &point + 10;
                point_label.set_label(&point.to_string());
                hangman_image_clone.set_from_file("./images/hangmanAlive.png");

                let mut rng = rand::thread_rng();
                world_list_clone.remove(
                    world_list_clone
                        .iter()
                        .position(|x| *x == word_label.get_text())
                        .unwrap(),
                );

                for i in &world_list_clone {
                    println!("{}", i)
                }

                word_label.set_text(&world_list_clone[rng.gen_range(0..10)]);
            } else {
                hangman_image_clone.set_from_file("./images/hangmanDead.png");
            }
            text_entry.set_placeholder_text(Some(message));
            println!("{}", point.to_string());
        });

        window.show_all();
    });
    app.run(&env::args().collect::<Vec<_>>());
}
