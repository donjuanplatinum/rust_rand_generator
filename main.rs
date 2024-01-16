use gtk::prelude::*;
use gtk::{Label, Orientation, Window, WindowType};
use rand::Rng;
use std::env::args;

fn generate_random_numbers(min: u32, max: u32, count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(min, max + 1)).collect()
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Random Number Generator");
    window.set_default_size(400, 300);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let vbox = gtk::Box::new(Orientation::Vertical, 5);
    window.add(&vbox);

    let min_label = Label::new(Some("Min:"));
    vbox.add(&min_label);

    let min_entry = gtk::Entry::new();
    vbox.add(&min_entry);

    let max_label = Label::new(Some("Max:"));
    vbox.add(&max_label);

    let max_entry = gtk::Entry::new();
    vbox.add(&max_entry);

    let count_label = Label::new(Some("Count:"));
    vbox.add(&count_label);

    let count_entry = gtk::Entry::new();
    vbox.add(&count_entry);

    let generate_button = gtk::Button::with_label("Generate");
    vbox.add(&generate_button);

    let result_label = Label::new(None);
    vbox.add(&result_label);

    generate_button.connect_clicked(move |_| {
        let min = min_entry
            .get_text()
            .parse()
            .expect("Invalid min value, please enter an integer.");
        let max = max_entry
            .get_text()
            .parse()
            .expect("Invalid max value, please enter an integer.");
        let count = count_entry
            .get_text()
            .parse()
            .expect("Invalid count value, please enter an integer.");

        let result = generate_random_numbers(min, max, count);
        let result_text = result
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        result_label.set_text(&result_text);
    });

    window.show_all();

    gtk::main();
}
