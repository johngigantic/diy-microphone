//! Audio streaming task
//!
//! Outputs audio data once each millisecond

use rtic_monotonics::systick::*;
use rtt_target::rprintln;

use crate::{app::stream, init::USB_AUDIO};

pub async fn stream(_cx: stream::Context<'_>) -> ! {
    let sine_table = [
        0i16, 4276, 8480, 12539, 16383, 19947, 23169, 25995, 28377, 30272, 31650, 32486, 32767,
        32486, 31650, 30272, 28377, 25995, 23169, 19947, 16383, 12539, 8480, 4276, 0, -4276, -8480,
        -12539, -16383, -19947, -23169, -25995, -28377, -30272, -31650, -32486, -32767, -32486,
        -31650, -30272, -28377, -25995, -23169, -19947, -16383, -12539, -8480, -4276,
    ];
    let sine_wave = unsafe { &*(&sine_table as *const _ as *const [u8; 96]) };

    loop {
        unsafe {
            if let Some(usb_audio) = USB_AUDIO.as_mut() {
                rprintln!("Sine!");
                usb_audio.write(sine_wave).ok();
            };
        }
        Systick::delay(1000.millis()).await;
    }
}
