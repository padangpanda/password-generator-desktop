use slint::PlatformError;

slint::include_modules!();

mod password;

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;
    

    ui.on_slider_toggled({
        let ui_handle = ui.as_weak();
        move |state| {
            let ui = ui_handle.unwrap();

            ui.set_len_int(state.round() as i32);
        }
    });
    ui.on_request_generate_password({
        let ui_handle = ui.as_weak();
        move |len, upper, num, symbols| {
            let ui = ui_handle.unwrap();

            let len_to_int = len.parse::<u32>().unwrap();

            let pass = password::create_password(len_to_int, num, upper, symbols);

            ui.set_pass(pass.into());
        }
    });

    ui.run()?;

    Ok(())
}
