use atsame54_xpro as bsp;

use bsp::{
    devices::usb_allocator,
    hal::{
        clock::GenericClockController,
        pac::{CorePeripherals, Peripherals},
        usb::usb_device::class_prelude::*,
        usb::usb_device::prelude::*,
    },
};
use usbd_audio::AudioClass;

use crate::app::{blink, init, stream, Local, Shared};
use rtic_monotonics::systick::*;
use rtt_target::{rtt_init_print, rprintln};

pub static mut USB_ALLOCATOR: Option<UsbBusAllocator<bsp::hal::usb::UsbBus>> = None;
pub static mut USB_AUDIO: Option<AudioClass<bsp::hal::usb::UsbBus>> = None;
pub static mut USB_DEVICE: Option<UsbDevice<bsp::hal::usb::UsbBus>> = None;

pub fn init(cx: init::Context) -> (Shared, Local) {
    rtt_init_print!();
    rprintln!("Init!");

    let core: CorePeripherals = cx.core;
    let mut device: Peripherals = cx.device;

    let mut clocks: GenericClockController = GenericClockController::with_external_32kosc(
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

    // USB Audio class setup
    let usb_dm = bsp::pin_alias!(pins.usb_dm);
    let usb_dp = bsp::pin_alias!(pins.usb_dp);

    // Why is this unsafe necessary?
    // RTIC's Local struct requires that all variables within have a static lifetime, but we cannot
    // automatically declare usb_allocator to have a static lifetime,
    // as it depends on the internal lifetime of the UsbBus contained within, which is not 'static.
    //
    // To fulfill the requirement, we create global static variables, USB_ALLOCATOR and such, wrapped
    // by an Option<_>, which is initalized to None and modified to Some() when the USB bus resources are created.
    let usb_allocator = unsafe {
        USB_ALLOCATOR = Some(usb_allocator(
            device.USB,
            &mut clocks,
            &mut device.MCLK,
            usb_dm,
            usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_AUDIO = Some(
            usbd_audio::AudioClassBuilder::new()
                .input(
                    usbd_audio::StreamConfig::new_discrete(
                        usbd_audio::Format::S24le,
                        1,
                        &[96000],
                        usbd_audio::TerminalType::InDesktopMicrophone,
                    )
                    .unwrap(),
                )
                .build(usb_allocator)
                .unwrap(),
        );
    }

    unsafe {
        USB_DEVICE = Some(UsbDeviceBuilder::new(usb_allocator, UsbVidPid(0x16c0, 0x27dd))
        .max_packet_size_0(64)
        .manufacturer("John Little")
        .product("Desktop Microphone")
        .serial_number("42")
        .max_power(500)
        .device_class(0x10)
        .device_sub_class(0x03)
        .build());
    }

    rprintln!("Done initializing");

    blink::spawn().ok();
    stream::spawn().ok();

    (Shared {}, Local { led })
}
