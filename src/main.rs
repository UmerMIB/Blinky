#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u16;
    loop {
        for curr in 0..8 {
            leds[curr].on();
            delay.delay_ms(ms);
        }

        for curr in 0..8{
            leds[curr].off();
            delay.delay_ms(ms);
        }
        for curr in 0..8 {
            leds[curr].on();
            delay.delay_ms(ms);
        }

        for curr in (0..8).rev(){
            leds[curr].off();
            delay.delay_ms(ms);
        }
        
    }
}

