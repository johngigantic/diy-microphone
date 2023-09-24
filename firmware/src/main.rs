#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device=atsame54p, peripherals=true)]
mod app {
    #[local]
    pub struct Local {}

    #[shared]
    pub struct Shared {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }
}
