//! LED Blinky task

use rtic_monotonics::systick::*;

use atsame54_xpro as bsp;
use bsp::hal::prelude::*;
use rtt_target::rprintln;

use crate::app::blink;

pub async fn blink(cx: blink::Context<'_>) -> ! {
    loop {
        cx.local.led.toggle().unwrap();
        rprintln!("Blink!");
        Systick::delay(500.millis()).await;
    }
}
