#![allow(non_snake_case, non_upper_case_globals)]

use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Button,
};

fn main()
{
    let app = Application::builder()
                .application_id("nemesisx00.dice-roller-rust.gtk")
                .build();
    app.connect_activate(onActivate);
    app.run();
}

fn onActivate(application: &Application)
{
    let window = ApplicationWindow::new(application);
    let button = Button::with_label("Hello World");
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}
