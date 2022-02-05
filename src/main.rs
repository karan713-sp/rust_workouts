// Program to blink four LEDs present on STM32F407VGTx in 1 second orderly fashion
// Crate used: stm32f4xx_hal

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
	hprintln!("STM32F407VGTx Board").unwrap();
	
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LEDs. Four LEDs have been mapped as per STM32F407VGTx.
        let gpiod = dp.GPIOD.split();
		let mut led1 = gpiod.pd12.into_push_pull_output();
		let mut led2 = gpiod.pd13.into_push_pull_output();
		let mut led3 = gpiod.pd14.into_push_pull_output();
        let mut led4 = gpiod.pd15.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

        loop {
            // On for 1s, off for 1s.
            led1.set_high();	// green
            delay.delay_ms(1000_u32);
            led2.set_high();    // orange
            delay.delay_ms(1000_u32);
			led3.set_high();    // red
            delay.delay_ms(1000_u32);
            led4.set_high();    // blue
            delay.delay_ms(1000_u32);
			
            led4.set_low();
            delay.delay_ms(1000_u32);
            led3.set_low();
            delay.delay_ms(1000_u32);
            led2.set_low();
            delay.delay_ms(1000_u32);
            led1.set_low();
            delay.delay_ms(1000_u32);			
        }
    }

    loop {}
}
