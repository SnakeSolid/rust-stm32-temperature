# Bluetooth Temperature Sensor

Read temperature, humidity and pressure from BME280 and write measurements to Bluetooth serial port. Serial output
example:

```
T: 23:87 C, P: 749.6 mmHg, H: 35.844%
T: 24:76 C, P: 749.6 mmHg, H: 34.101%
T: 24.70 C, P: 749.7 mmHg, H: 33.334%
```

Rows are separated with `\r\n` new line sequence.

### Wiring Diagram

Sensor BME280 use interface I2C2 (PA12, PA11), Bluetooth use interface USART2 (PA2, PA3). By default BME280 must be
available on address 0x76 (SDO connected to GND).

```
                                 STM32G030Fx (TSSOP20)
                        /-------------------------------------+
                      --| PB7/PB8           20PB3/PB4/PB5/PB6 |--
                      --| PB9/PC14-OSC32_IN   PA15/PA14-BOOT0 |--      to BME280
                      --| PC15-OSC32_OUT                 PA13 |--
                      --| VDD/VDDA                 PA12[PA10] |-- I2C2_SDA
                      --| VSS/VSSA                  PA11[PA9] |-- I2C2_SCL
                      --| NRST                PB0/PB1/PB2/PA8 |--
   to Bluetooth       --| PA0                             PA7 |--
                      --| PA1                             PA6 |--
            USART2_TX --| PA2                             PA5 |--
            USART2_RX --| PA3                             PA4 |--
                        +-------------------------------------+
```

### Build

By default cargo will use native build target. To build firmware for STM32G030 following command can be used:

```sh
cargo build --target thumbv6m-none-eabi --release
```

Command to connect to microcontroller using JTAG:

```sh
openocd -f openocd.cfg
```

Version of `openocd` utility must be greater or equals to `0.11.0`, earlier versions does not support STM32G030
microcontrollers. To flash and debug firmware use following command:

```sh
gdb-multiarch target/thumbv6m-none-eabi/release/stm32-bme280-temperature --command openocd.gdb
```

File `openocd.gdb` already contains commands to establish connection with `openocd`, enable semihosting, load firmware
and set breakpoints to `DefaultHandler` and `HardFault` handlers. After flashing firmware will be paused, command
`continue` will start firmware execution.

### License

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
