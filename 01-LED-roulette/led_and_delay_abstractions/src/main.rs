#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut ping_period = 100_u16;
    let v_ping_period = Volatile::new(&mut ping_period);

    loop {
        leds[0].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[1].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[2].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[3].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[4].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[5].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[6].on().ok();
        delay.delay_ms(v_ping_period.read());
        leds[7].on().ok();
        delay.delay_ms(v_ping_period.read());

        leds[0].off().ok();
        leds[1].off().ok();
        leds[2].off().ok();
        leds[3].off().ok();
        leds[4].off().ok();
        leds[5].off().ok();
        leds[6].off().ok();
        leds[7].off().ok();
    }
}
