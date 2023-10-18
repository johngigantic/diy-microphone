//! LED Blinky task

use rtic_monotonics::systick::*;

use atsame54_xpro as bsp;
use bsp::hal::prelude::*;

use crate::app::blink;

pub async fn blink(cx: blink::Context<'_>) -> ! {
    loop {
        cx.local.led.toggle().unwrap();
        Systick::delay(500.millis()).await;
    }
}