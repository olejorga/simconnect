use simconnect::{Name, Unit, Variable};

fn main() {
    let mut client = simconnect::Client::new();

    match client.open("") {
        Ok(_) => println!("Connected to simulator"),
        Err(_) => println!("Failed to connect to simulator"),
    }

    let var = Variable {
        name: Name("PLANE ALTITUDE".to_string()),
        unit: Unit("Feet".to_string()),
        value: 0.0,
    };

    client.listen(var);

    loop {
        match client.receive() {
            Some(msg) => {
                match msg {
                    simconnect::Message::Open => print!("MESSAGE"),
                    simconnect::Message::Quit => print!("QUIT"),
                    simconnect::Message::Exception(excep) => print!("EXCEPTION: {}", excep)
                }
            },
            None => (),
        }

        std::thread::sleep(std::time::Duration::from_millis(16))
    }
}
