use atsame54_xpro as bsp;

use bsp::hal::{
    pac::{CorePeripherals, Peripherals},
    clock::GenericClockController,
};

use rtic_monotonics::systick::*;

use crate::app::{init, blink, Shared, Local};

pub fn init(cx: init::Context) -> (Shared, Local) {
    let core: CorePeripherals = cx.core;
    let mut device: Peripherals = cx.device;

    let mut _clocks: GenericClockController = GenericClockController::with_external_32kosc(
        device.GCLK,
        &mut device.MCLK,
        &mut device.OSC32KCTRL,
        &mut device.OSCCTRL,
        &mut device.NVMCTRL,
    );

    let systick_token = rtic_monotonics::create_systick_token!();
    Systick::start(core.SYST, 120_000_000, systick_token);

    let pins = bsp::Pins::new(device.PORT);
    let led = bsp::pin_alias!(pins.led).into_push_pull_output();

    blink::spawn().ok();

    (Shared {}, Local {led})
}