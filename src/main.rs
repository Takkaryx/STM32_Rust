#![no_main]
#![no_std]

// extern crate panic_halt;
use panic_semihosting as _;

mod audio;
mod display;
mod packet;

use crate::audio::Audio;
use crate::display::Display;

use cortex_m_rt::entry;

use crate::hal::{
    gpio::{Output, PA8, PB10},
    i2c::I2c,
    interrupt, pac,
    prelude::*,
    rcc::{Clocks, Rcc},
    timer::{pwm, CounterUs, Event},
};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use hdlc_modem::Decoder;
use stm32f4xx_hal as hal;

const SAMPLE_RATE: u32 = 9600;

static G_AUDIO_IN: Mutex<RefCell<Option<Audio>>> = Mutex::new(RefCell::new(None));
static G_PTT: Mutex<RefCell<Option<PB10<Output>>>> = Mutex::new(RefCell::new(None));

fn setup_clocks(rcc: Rcc) -> Clocks {
    rcc.cfgr
        .hclk(48u32.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze()
}

fn setup_timer(clocks: &Clocks, tim1: pac::TIM1, debug_pin: PA8) {
    // let mut timer = tim1.counter_hz(clocks);
    // timer.start(SAMPLE_RATE.Hz()).unwrap();

    /*
    let mut timer = tim2.counter_us(clocks);
    timer.start((1_000_000_000 / SAMPLE_RATE).nanos()).unwrap();
    timer.listen(Event::Update);
     */

    let debug_pin = debug_pin.into_alternate();
    let mut pwm = tim1.pwm_hz(debug_pin, SAMPLE_RATE.Hz(), clocks).split();
    let max_duty = pwm.get_max_duty();
    pwm.set_duty(max_duty / 2);
    pwm.enable();
}

#[entry]
fn main() -> ! {}
