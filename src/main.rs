use std::cell::{Cell, RefCell};
use std::process::Command;
use std::rc::Rc;
use std::string::String;

extern crate wifi_rs;
extern crate wifiscanner;
use wifi_rs::prelude::*;
use wifi_rs::WiFi;

use gtk::builders::ButtonBuilder;
use gtk::glib::clone;
use gtk::{glib, prelude::*, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow};
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    // get wifi
    let networks = get_wifi_networks();
    println!("Found {} networks", networks.len());
    // list
    let list_box = ListBox::new();
    if networks.is_empty() {
        let label = Label::new(Some("No networks found"));
        list_box.append(&label);
    } else {
        for network in networks {
            let label = Label::new(Some(&network.ssid));
            list_box.append(&label);
        }
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(800)
        .min_content_height(600)
        .child(&list_box)
        .build();

    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}

fn get_wifi_networks() -> Vec<WifiNetwork> {
    let output = Command::new("nmcli")
        .args(["dev", "wifi"])
        .output()
        .expect("Failed to execute command");

    let mut networks = Vec::new();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.split('\n').collect();

        for line in lines.iter().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let ssid = parts[1].to_string();

                let network = WifiNetwork {
                    ssid,
                    // Populate with other values as needed
                };

                networks.push(network);
            }
        }

        println!("{:#?}", networks);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
    }

    networks
}

#[derive(Debug)]
struct WifiNetwork {
    ssid: String,
    // Add more fields as needed
}
