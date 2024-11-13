use slint::PlatformError;

slint::include_modules!();

mod password;

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_generate_password({
        let ui_handle = ui.as_weak();
        
        move |len, upper, num, symbols| {
            let ui = ui_handle.unwrap();

            let pass = password::create_password(len.parse::<i32>().unwrap(), num, upper, symbols);

            ui.set_pass(pass.into());
        }
    });

    ui.run()?;

    Ok(())
}
