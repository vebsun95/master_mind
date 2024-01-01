fn main() {
    let application_state = engine::platform::ApplicationState::new(0, 0, 600, 800, &String::from("Master mind"));

    loop {
        if application_state.pump_messages() && false {
            break;
        }
    }

    println!("Actually completed.");
}