fn main() {
    let mut client = simconnect::Client::new();

    match client.open("") {
        Ok(_) => println!("Connected to simulator"),
        Err(_) => println!("Failed to connect to simulator"),
    }

    loop {
        match client.receive() {
            Some(message) => {
                match message {
                    simconnect::Message::Open => print!("MESSAGE"),
                }
            },
            None => (),
        }

        std::thread::sleep(std::time::Duration::from_millis(16))
    }
}
