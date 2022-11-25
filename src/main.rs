#![no_std]
#![no_main]

use cortex_m_rt::entry;
use hal::{gpio::Level, pac, prelude::*, spi::Spi};
use nrf52840_hal as hal;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::digital::v2::OutputPin;
use smart_leds::colors;
use smart_leds::SmartLedsWrite;
use ws2812_blocking_spi::Ws2812BlockingWriter;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let _clocks = hal::clocks::Clocks::new(dp.CLOCK).enable_ext_hfosc();

    let mut delay = hal::Delay::new(p.SYST);

    let mut timer = hal::timer::Timer::periodic(dp.TIMER4);
    timer.start(1u32);

    let port0 = hal::gpio::p0::Parts::new(dp.P0);

    let spiclk = port0.p0_15.into_push_pull_output(Level::Low).degrade();
    let spimosi = port0.p0_16.into_push_pull_output(Level::Low).degrade();
    let spimiso = port0.p0_17.into_floating_input().degrade();

    let pins = hal::spi::Pins {
        sck: Some(spiclk),
        miso: Some(spimiso),
        mosi: Some(spimosi),
    };
    let spi = Spi::new(dp.SPI0, pins, hal::spi::Frequency::M4, hal::spi::MODE_0);

    let mut neopixel = Ws2812BlockingWriter::new(spi);

    // does the data write

    let p1 = hal::gpio::p1::Parts::new(dp.P1);
    let mut led = p1.p1_15.into_push_pull_output(Level::Low);

    let mut led_state = false;

    let mut data = colors::RED;
    neopixel.write([data].iter().cloned()).unwrap();
    loop {
        if led_state {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        led_state = !led_state;
        data.r = data.r.wrapping_add(4);
        data.g = data.g.wrapping_add(8);
        data.b = data.b.wrapping_add(16);

        neopixel.write([data].iter().cloned()).unwrap();

        rprintln!("Step");
        delay.delay_ms(500u16);
    }
}
