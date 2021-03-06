#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

extern crate nucleo_f401re as board;

#[macro_use(block)]
extern crate nb;

use board::hal::prelude::*;
use board::hal::stm32;

use board::hal::serial::{Serial, config::Config};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpioa = p.GPIOA.split();
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let tx = gpioa.pa2.into_alternate_af7();
        let rx = gpioa.pa3.into_alternate_af7();

        let config = Config::default()
            .baudrate(115200.bps());

        let serial = Serial::usart2(p.USART2, (tx, rx), config, clocks).unwrap();

        let (mut tx, mut rx) = serial.split();

        loop {
            // Read character and echo it back
            let received = block!(rx.read()).unwrap();
            block!(tx.write(received)).ok();
        }
    }

    loop {}
}
