slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    
    // Clone the weak reference for use inside callbacks
    let weak = main_window.as_weak();
    
    // Handle numeric and decimal point button clicks
    main_window.on_button_clicked(move |value| {
        let main_window = weak.unwrap();
        let current = main_window.get_display();
        let start_new = main_window.get_start_new_input();
        
        // If we're starting a new input, replace the display
        // Otherwise append to the current display
        if start_new {
            if value == "." {
                // If starting with decimal point, prepend a zero
                main_window.set_display("0.".into());
            } else {
                main_window.set_display(value.into());
            }
            main_window.set_start_new_input(false);
        } else {
            // Don't allow multiple decimal points
            if value == "." && current.contains('.') {
                return;
            }
            
            // Don't allow leading zeros
            if current == "0" && value != "." {
                main_window.set_display(value.into());
            } else {
                main_window.set_display(format!("{}{}", current, value));
            }
        }
    });
    
    // Handle operation buttons (+, -, *, /)
    let weak = main_window.as_weak();
    main_window.on_operation_clicked(move |operation| {
        let main_window = weak.unwrap();
        
        // Store the current value
        match main_window.get_display().parse::<f64>() {
            Ok(value) => {
                // If there was a previous operation pending, perform it first
                if !main_window.get_current_operation().is_empty() {
                    let result = perform_operation(
                        main_window.get_stored_value(),
                        value,
                        &main_window.get_current_operation()
                    );
                    main_window.set_stored_value(result);
                    main_window.set_display(format_result(result));
                } else {
                    main_window.set_stored_value(value);
                }
                
                // Set the new operation
                main_window.set_current_operation(operation.into());
                main_window.set_start_new_input(true);
            },
            Err(_) => {
                // Handle parsing error
                main_window.set_display("Error".into());
                main_window.set_start_new_input(true);
            }
        }
    });
    
    // Handle equals button
    let weak = main_window.as_weak();
    main_window.on_equals_clicked(move || {
        let main_window = weak.unwrap();
        
        if main_window.get_current_operation().is_empty() {
            return; // No operation pending
        }
        
        match main_window.get_display().parse::<f64>() {
            Ok(value) => {
                let result = perform_operation(
                    main_window.get_stored_value(),
                    value,
                    &main_window.get_current_operation()
                );
                main_window.set_display(format_result(result));
                main_window.set_stored_value(0.0);
                main_window.set_current_operation("".into());
                main_window.set_start_new_input(true);
            },
            Err(_) => {
                // Handle parsing error
                main_window.set_display("Error".into());
                main_window.set_start_new_input(true);
            }
        }
    });
    
    // Handle clear button
    let weak = main_window.as_weak();
    main_window.on_clear_clicked(move || {
        let main_window = weak.unwrap();
        main_window.set_display("0".into());
        main_window.set_stored_value(0.0);
        main_window.set_current_operation("".into());
        main_window.set_start_new_input(true);
    });
    
    main_window.run()
}

// Helper function to perform the actual calculation
fn perform_operation(a: f64, b: f64, operation: &str) -> f64 {
    match operation {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                f64::NAN // Handle division by zero
            } else {
                a / b
            }
        },
        _ => b, // Default case, should not happen
    }
}

// Helper function to format the result
fn format_result(result: f64) -> String {
    if result.is_nan() {
        return "Error".into();
    }
    
    if result.fract() == 0.0 {
        // No decimal part, display as integer
        return format!("{}", result as i64);
    } else {
        // Remove trailing zeros
        let mut s = format!("{}", result);
        while s.ends_with('0') && s.contains('.') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
        return s;
    }
}