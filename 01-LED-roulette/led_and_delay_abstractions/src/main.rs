#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 500_u16;

    loop {
        leds[0].on().ok();
        leds[1].off().ok();
        leds[2].on().ok();
        leds[3].off().ok();
        leds[4].on().ok();
        leds[5].off().ok();
        leds[6].on().ok();
        leds[7].off().ok();
        delay.delay_ms(half_period);

        leds[0].off().ok();
        leds[1].on().ok();
        leds[2].off().ok();
        leds[3].on().ok();
        leds[4].off().ok();
        leds[5].on().ok();
        leds[6].off().ok();
        leds[7].on().ok();
        delay.delay_ms(half_period);
    }
}
