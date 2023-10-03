#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use panic_halt as _;

mod init;
mod tasks;
mod test;

#[rtic::app(device=atsame54p, peripherals=true, dispatchers=[EVSYS_0])]
mod app {
    use crate::init::init;
    use crate::tasks::blink::blink;
    use crate::tasks::stream::stream;
    use atsame54_xpro as bsp;

    #[local]
    pub struct Local {
        pub led: bsp::Led,
    }

    #[shared]
    pub struct Shared {}

    extern "Rust" {
        #[init]
        fn init(cx: init::Context) -> (Shared, Local);

        #[task(local=[led], priority=2)]
        async fn blink(cx: blink::Context);

        #[task(local=[], priority=2)]
        async fn stream(cx: stream::Context);
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
