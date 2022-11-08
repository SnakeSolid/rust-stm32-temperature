// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use bme280::Bme280;
use bme280::HumidityExt;
use bme280::HumiditySampling;
use bme280::PressureExt;
use bme280::PressureSampling;
use bme280::SensorMode;
use bme280::TemperatireSampling;
use bme280::TemperatureExt;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::prelude::_embedded_hal_timer_CountDown;
use cortex_m_rt::entry;
use nb::block;
use stm32g0xx_hal::gpio::GpioExt;
use stm32g0xx_hal::i2c::Config as I2CConfig;
use stm32g0xx_hal::i2c::I2cExt;
use stm32g0xx_hal::prelude::OutputPin;
use stm32g0xx_hal::prelude::PinState;
use stm32g0xx_hal::rcc::RccExt;
use stm32g0xx_hal::serial::BasicConfig;
use stm32g0xx_hal::serial::SerialExt;
use stm32g0xx_hal::stm32::Peripherals;
use stm32g0xx_hal::time::U32Ext;
use stm32g0xx_hal::timer::TimerExt;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb0.into_push_pull_output();
    let tx = gpioa.pa2;
    let rx = gpioa.pa3;
    let sda = gpioa.pa12.into_open_drain_output_in_state(PinState::High);
    let scl = gpioa.pa11.into_open_drain_output_in_state(PinState::High);
    let (mut usart_tx, _usart_rx) = dp
        .USART2
        .usart(
            tx,
            rx,
            BasicConfig::default().baudrate(9_600.bps()),
            &mut rcc,
        )
        .unwrap()
        .split();
    let mut i2c = dp.I2C2.i2c(sda, scl, I2CConfig::new(400.khz()), &mut rcc);
    let mut timer = dp.TIM17.timer(&mut rcc);
    let mut bme280 = Bme280::address_low(&mut i2c).expect("Failed to initialize BME280 sensor");
    bme280
        .reset(&mut i2c)
        .expect("Failed to reset BME280 sensor");
    bme280
        .sampling(
            &mut i2c,
            HumiditySampling::Sampling16,
            TemperatireSampling::Sampling16,
            PressureSampling::Sampling16,
            SensorMode::Normal,
        )
        .expect("Failed to set sampling for BME280 sensor");

    timer.start(1.hz());
    timer.listen();
    timer.clear_irq();

    writeln!(usart_tx, "Ready.\r").unwrap();

    let _ = led.set_high();

    loop {
        let temperature = bme280
            .temperature(&mut i2c)
            .expect("Failed to read temperature")
            .C();
        let pressure = bme280
            .pressure(&mut i2c)
            .expect("Failed to read pressure")
            .mmHg();
        let humidity = bme280
            .humidity(&mut i2c)
            .expect("Failed to read humidity")
            .percent();
        let _ = writeln!(
            usart_tx,
            "T: {}:{:02}°C, P: {}.{:01} mmHg, H: {}.{:03}%",
            temperature.0, temperature.1, pressure.0, pressure.1, humidity.0, humidity.1,
        );

        block!(timer.wait()).unwrap();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
