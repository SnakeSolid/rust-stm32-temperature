#![deny(unsafe_code)]
#![deny(warnings)]
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
use cortex_m::asm::wfi;
use cortex_m::prelude::*;
use cortex_m_rtic_macros::app;
use stm32g0xx_hal::gpio::gpioa::PA11;
use stm32g0xx_hal::gpio::gpioa::PA12;
use stm32g0xx_hal::gpio::gpiob::PB0;
use stm32g0xx_hal::gpio::GpioExt;
use stm32g0xx_hal::gpio::OpenDrain;
use stm32g0xx_hal::gpio::Output;
use stm32g0xx_hal::gpio::PushPull;
use stm32g0xx_hal::i2c::Config as I2CConfig;
use stm32g0xx_hal::i2c::I2c;
use stm32g0xx_hal::i2c::I2cExt;
use stm32g0xx_hal::prelude::OutputPin;
use stm32g0xx_hal::prelude::PinState;
use stm32g0xx_hal::rcc::RccExt;
use stm32g0xx_hal::serial::BasicConfig;
use stm32g0xx_hal::serial::SerialExt;
use stm32g0xx_hal::serial::Tx;
use stm32g0xx_hal::stm32::I2C2;
use stm32g0xx_hal::stm32::TIM17;
use stm32g0xx_hal::stm32::USART2;
use stm32g0xx_hal::time::U32Ext;
use stm32g0xx_hal::timer::delay::DelayExt;
use stm32g0xx_hal::timer::Timer;
use stm32g0xx_hal::timer::TimerExt;

type Timer17 = Timer<TIM17>;

type LedPin = PB0<Output<PushPull>>;

type UsartTx = Tx<USART2, BasicConfig>;

type I2C = I2c<I2C2, PA12<Output<OpenDrain>>, PA11<Output<OpenDrain>>>;

#[app(device = stm32g0xx_hal::stm32, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        timer: Timer17,
        led: LedPin,
        usart_tx: UsartTx,
        i2c: I2C,
        bme280: Bme280,
    }

    #[init]
    fn init(context: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = context.device.RCC.constrain();
        let gpioa = context.device.GPIOA.split(&mut rcc);
        let gpiob = context.device.GPIOB.split(&mut rcc);
        let led = gpiob.pb0.into_push_pull_output();
        let tx = gpioa.pa2;
        let rx = gpioa.pa3;
        let sda = gpioa.pa12.into_open_drain_output_in_state(PinState::High);
        let scl = gpioa.pa11.into_open_drain_output_in_state(PinState::High);
        let (mut usart_tx, _usart_rx) = context
            .device
            .USART2
            .usart(
                tx,
                rx,
                BasicConfig::default().baudrate(9_600.bps()),
                &mut rcc,
            )
            .unwrap()
            .split();
        let mut i2c = context
            .device
            .I2C2
            .i2c(sda, scl, I2CConfig::new(400.khz()), &mut rcc);
        let bme280 = Bme280::address_low(&mut i2c).expect("Failed to initialize BME280 sensor");
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

        let mut delay = context.device.TIM17.delay(&mut rcc);
        delay.delay(100.ms()); // Wait for BME280 first measurement.

        let mut timer = delay.release().timer(&mut rcc);
        timer.start(60.seconds());
        timer.listen();

        let _ = writeln!(usart_tx, "Ready.\r");

        (
            Shared {},
            Local {
                timer,
                led,
                usart_tx,
                i2c,
                bme280,
            },
            init::Monotonics(),
        )
    }

    #[idle(local = [ led, usart_tx, i2c, bme280 ])]
    fn idle(context: idle::Context) -> ! {
        loop {
            let _ = context.local.led.set_low();
            let temperature = context
                .local
                .bme280
                .temperature(context.local.i2c)
                .expect("Failed to read temperature")
                .c();
            let pressure = context
                .local
                .bme280
                .pressure(context.local.i2c)
                .expect("Failed to read pressure")
                .mmhg();
            let humidity = context
                .local
                .bme280
                .humidity(context.local.i2c)
                .expect("Failed to read humidity")
                .percent();
            let _ = writeln!(
                context.local.usart_tx,
                "T: {}.{:02} C, P: {}.{:01} mmHg, H: {}.{:03}%\r",
                temperature.0, temperature.1, pressure.0, pressure.1, humidity.0, humidity.1,
            );
            let _ = context.local.led.set_high();

            wfi();
        }
    }

    #[task(binds = TIM17, local = [ timer ])]
    fn timer_tick(context: timer_tick::Context) {
        context.local.timer.clear_irq();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
