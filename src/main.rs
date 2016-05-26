//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the dialog with an imported glade file

extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Button, Label, MessageDialog, Window};
    use std::env;

    pub fn get_cnt_label_val(b: &Builder) -> i32 {
        let cnt_label: Label = b.get_object("count_label").unwrap();

        let label_text_opt_int: Option<i32> = cnt_label.get_label().unwrap().trim().parse::<i32>().ok();

        match label_text_opt_int {
            Some(label_text_as_int) => label_text_as_int,
            None => {
                println!("Could not parse labels field. Will cnt_label let \"0\".");
                0
            }
        }
    }

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("dialog.glade");
        let dialog = Builder::new_from_string(glade_src);

        let window: Window = dialog.get_object("window1").unwrap();
        //window.set_title(format!("Dialog: {}", env::current_dir().unwrap().display()));

        let cnt_up_btn: Button = dialog.get_object("count_up_btn").unwrap();
        let cnt_down_btn: Button = dialog.get_object("count_down_btn").unwrap();

        let cnt_label: Label = dialog.get_object("count_label").unwrap();

        let close_btn: Button = dialog.get_object("close_btn").unwrap();

        cnt_up_btn.connect_clicked(move |_| {
            let mut label_text_as_int = get_cnt_label_val(&dialog);

            label_text_as_int += 1;

            println!("{:?}", label_text_as_int);
            println!("Click, click.");
            cnt_label.set_label(label_text_as_int.to_string().as_str());

        });

        close_btn.connect_clicked(move |_| {
            gtk::main_quit();
        });

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example only work with GTK 3.10 and later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}

