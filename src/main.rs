use slint::PlatformError;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

mod password;

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    // Use Rc<RefCell<T>> to allow shared mutable state
    let len_val = Rc::new(RefCell::new(String::new()));
    let num_val = Rc::new(RefCell::new(false));
    let upper_val = Rc::new(RefCell::new(false));
    let symbol_val = Rc::new(RefCell::new(false));

    // Event handler for setting password settings
    ui.on_request_set_password_settings({
        let len_val = Rc::clone(&len_val);
        let num_val = Rc::clone(&num_val);
        let upper_val = Rc::clone(&upper_val);
        let symbol_val = Rc::clone(&symbol_val);
        
        move |len, upper, num, symbols| {
            // Update the values
            *len_val.borrow_mut() = len.to_string();
            *num_val.borrow_mut() = num;
            *upper_val.borrow_mut() = upper;
            *symbol_val.borrow_mut() = symbols;
        }
    });

    // Event handler for generating password
    ui.on_request_generate_password({
        let ui_handle = ui.as_weak();
        let len_val = Rc::clone(&len_val);
        let num_val = Rc::clone(&num_val);
        let upper_val = Rc::clone(&upper_val);
        let symbol_val = Rc::clone(&symbol_val);
        
        move || {
            let ui = ui_handle.unwrap();
            let len = len_val.borrow().parse::<i32>().unwrap(); // Parse length
            let num = *num_val.borrow();  // Access bool value
            let upper = *upper_val.borrow();  // Access bool value
            let symbol = *symbol_val.borrow();  // Access bool value

            let pass = password::create_password(len, num, upper, symbol);

            ui.set_pass(pass.into());
        }
    });

    // Run the UI event loop
    ui.run()?;

    Ok(())
}
