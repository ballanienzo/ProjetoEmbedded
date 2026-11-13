use std::{
    error::Error,
    io::{self, BufRead, BufReader, Write},
    thread,
    time::Duration,
};

use commands::Command;
use notes::Note;
use serialport::{
    SerialPort, SerialPortInfo,
    SerialPortType::{self},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let avaliable_ports: Vec<SerialPortInfo> = serialport::available_ports().unwrap_or_default();
    let usb_ports: Vec<SerialPortInfo> = avaliable_ports
        .into_iter()
        .filter(|port| match &port.port_type {
            SerialPortType::UsbPort(info) => info
                .product
                .as_deref()
                .is_some_and(|m| !m.trim().is_empty()),
            _ => false,
        })
        .collect();

    let selected_port = if usb_ports.len() == 1 {
        usb_ports.first().unwrap()
    } else {
        usb_ports.iter().enumerate().for_each(|(i, port)| {
            println!(
                "{} : {}",
                match &port.port_type {
                    SerialPortType::UsbPort(info) => info.product.clone().unwrap(),
                    _ => "".to_string(),
                },
                port.port_name
            );
        });

        usb_ports.first().unwrap()
    };

    let mut serial = serialport::new(selected_port.port_name.clone(), commands::BAUD_RATE)
        .timeout(Duration::from_millis(500))
        .open()?;

    let mut reader = BufReader::new(serial.try_clone()?);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(n) if n > 0 && line.trim() == "READY" => break,
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
    }

    println!("Conected");

    // Note sequence supplied by the user; durations are in milliseconds.
    let melody = [
        ("D5", 120),
        ("D5", 120),
        ("D6", 240),
        ("A5", 240),
        ("A#5", 120),
        ("A5", 120),
        ("G#5", 120),
        ("G5", 120),
        ("F5", 120),
        ("D5", 120),
        ("F5", 120),
        ("G5", 240),
        ("C5", 120),
        ("C5", 120),
        ("D6", 240),
        ("A5", 240),
        ("A#5", 120),
        ("A5", 120),
        ("G#5", 120),
        ("G5", 120),
        ("F5", 120),
        ("D5", 120),
        ("F5", 120),
        ("G5", 240),
    ];

    for (note, duration_ms) in melody {
        let frequency = Note::from(note).unwrap().frequency();
        send_command(&mut serial, &Command::PlayNote(frequency))?;
        thread::sleep(Duration::from_millis(duration_ms));
    }
    send_command(&mut serial, &Command::StopSound)?;

    Ok(())
}

fn send_command(port: &mut Box<dyn SerialPort>, msg: &Command) -> io::Result<()> {
    let bytes =
        postcard::to_stdvec_cobs(msg).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    port.write_all(&bytes)?;
    port.flush()?;

    println!("{:?}", msg);
    Ok(())
}
