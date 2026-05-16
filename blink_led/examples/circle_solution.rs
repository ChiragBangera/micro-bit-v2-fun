#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use nrf52833_hal::{pac, timer};
use panic_halt as _;

enum Port {
    P0,
    P1,
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let leds: [((usize, Port), (usize, Port)); 16] = [
        ((21, Port::P0), (28, Port::P0)),
        ((21, Port::P0), (11, Port::P0)),
        ((21, Port::P0), (31, Port::P0)),
        ((21, Port::P0), (5, Port::P1)),
        ((21, Port::P0), (30, Port::P0)),
        ((22, Port::P0), (30, Port::P0)),
        ((15, Port::P0), (30, Port::P0)),
        ((24, Port::P0), (30, Port::P0)),
        ((19, Port::P0), (30, Port::P0)),
        ((19, Port::P0), (5, Port::P0)),
        ((19, Port::P0), (31, Port::P0)),
        ((19, Port::P0), (11, Port::P0)),
        ((19, Port::P0), (28, Port::P0)),
        ((24, Port::P0), (28, Port::P0)),
        ((15, Port::P0), (28, Port::P0)),
        ((22, Port::P0), (28, Port::P0)),
    ];

    for led in &leds {
        //handling row outputs
        p.P0.pin_cnf[led.0.0].write(|w| w.dir().output());

        // handling column led outputs
        match led.1.1 {
            Port::P0 => set_p0_port_as_output(&p.P0, led.1.0),
            Port::P1 => set_p1_port_as_output(&p.P1, led.1.0),
        }
    }

    let mut timer0 = timer::Timer::new(p.TIMER0);

    loop {
        for led in &leds {
            //turning on led
            //row high
            set_p0_pin(&p.P0, led.0.0, true);

            //set col low
            match led.1.1 {
                Port::P0 => set_p0_pin(&p.P0, led.1.0, false),
                Port::P1 => set_p1_pin(&p.P1, led.1.0, false),
            }

            //turn off delay
            timer0.delay_ms(100);

            //turning off led
            // row low
            set_p0_pin(&p.P0, led.0.0, false);

            // set col low
            match led.1.1 {
                Port::P0 => set_p0_pin(&p.P0, led.1.0, true),
                Port::P1 => set_p1_pin(&p.P1, led.1.0, true),
            }
        }
    }
}

fn set_p0_port_as_output(p: &pac::P0, pin: usize) -> () {
    p.pin_cnf[pin].write(|w| w.dir().output());
}

fn set_p1_port_as_output(p: &pac::P1, pin: usize) -> () {
    p.pin_cnf[pin].write(|w| w.dir().output());
}

fn set_p0_pin(p: &pac::P0, pin: usize, high: bool) -> () {
    p.out.modify(|r, w| unsafe {
        if high {
            w.bits(r.bits() | (1 << pin as u32))
        } else {
            w.bits(r.bits() & !(1 << pin as u32))
        }
    });
}

fn set_p1_pin(p: &pac::P1, pin: usize, high: bool) -> () {
    p.out.modify(|r, w| unsafe {
        if high {
            w.bits(r.bits() | (1 << pin as u32))
        } else {
            w.bits(r.bits() & !(1 << pin as u32))
        }
    });
}
