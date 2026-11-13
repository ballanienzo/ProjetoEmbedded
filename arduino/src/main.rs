#![no_std]
#![no_main]

use arduino_hal::{pac::TC1, prelude::{_embedded_hal_serial_Read, _unwrap_infallible_UnwrapInfallible}};
use commands::Command;
use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, commands::BAUD_RATE);
    let mut buzzer_pin = pins.d11.into_output();
    let timer1 = dp.TC1;
    
    let mut buffer = [0u8; 64];
    let mut pos = 0;

    ufmt::uwriteln!(&mut serial, "READY").unwrap_infallible();

    loop {
        let byte = nb::block!(serial.read()).unwrap();

        if pos < buffer.len() {
            buffer[pos] = byte;
            pos += 1;   
        } else {
            pos = 0;
            continue;
        };

        if byte == 0x00 {
            let cmd = postcard::from_bytes_cobs::<Command>(&mut buffer[..pos]).unwrap();
            
            match cmd {
                Command::PlayNote(freq) => play_note(&timer1, freq),
                Command::StopSound => stop_sound(&timer1, &mut buzzer_pin),
                Command::Sleep(ms) => arduino_hal::delay_ms(ms.min(u32::MAX as usize) as u32),
            }
            pos = 0;
        }
    }
}

fn play_note(timer: &TC1, freq: u32) {
    if freq == 0 {
        return;
    }

    let counts = (16_000_000u32 / (2 * freq)).clamp(1, u16::MAX as u32 + 1);
    let top = (counts - 1) as u16;

    timer.tccr1b().write(|w| w.cs1().no_clock());
    timer.ocr1a().write(|w| w.set(top));
    timer.tccr1a().write(|w| w.wgm1().set(0).com1a().set(0b01));
    timer.tccr1b().write(|w| w.wgm1().set(0b01).cs1().set(0b001));
}

fn stop_sound(timer: &TC1, pin: &mut arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Output, arduino_hal::hal::port::PB5>) {
    timer.tccr1b().write(|w| w.cs1().no_clock());
    timer.tccr1a().write(|w| w.wgm1().set(0).com1a().disconnected());
    pin.set_low();
}
