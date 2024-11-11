// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

slint::include_modules!();

mod password;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let mut len = "16";
    let mut num = false;
    let mut upper = false;
    let mut symbol = false;


    ui.on_request_set_password_settings({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            len = &ui.get_len();
            num = ui.get_num();
            upper = ui.get_upper();
            symbol = ui.get_symbol();
        }
    });

    ui.on_request_generate_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let pass = password::create_password(len.parse::<i32>().unwrap(), num, upper, symbol);

            ui.set_pass(pass.into());
        }
    });

    ui.run()?;

    Ok(())
}
