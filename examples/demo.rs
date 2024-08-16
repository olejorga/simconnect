use simconnect::{Name, Unit, Variable};

fn main() {
    let mut client = simconnect::Client::new();

    match client.open("") {
        Ok(_) => println!("Connected to simulator"),
        Err(_) => println!("Failed to connect to simulator"),
    }

    let variable = Variable {
        name: Name("AIRSPEED INDICATED".to_string()),
        unit: Unit("Knots".to_string()),
        value: 0.0,
    };

    match client.observe(variable) {
        Ok(_) => println!("Observing to indicated airspeed"),
        Err(_) => println!("Failed to observe indicated airspeed"),
    }

    loop {
        match client.receive() {
            Some(msg) => {
                match msg {
                    simconnect::Message::Open => println!("OPEN"),
                    simconnect::Message::Quit => println!("QUIT"),
                    simconnect::Message::Exception(exception) => println!("EXCEPTION: {}", exception),
                    simconnect::Message::Variable(variable) => println!("{}", variable.value)
                }
            },
            None => (),
        }

        std::thread::sleep(std::time::Duration::from_millis(16))
    }
}
