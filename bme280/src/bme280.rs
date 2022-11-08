use cortex_m::prelude::_embedded_hal_blocking_i2c_Write as Write;
use cortex_m::prelude::_embedded_hal_blocking_i2c_WriteRead as WriteRead;

const BME280_ADDRESS_LOW: u8 = 0x76;
const BME280_ADDRESS_HIGH: u8 = 0x77;
const BME280_RESET_VALUE: u8 = 0xB6;

const REGISTER_DIG_T1: u8 = 0x88;
const REGISTER_DIG_T2: u8 = 0x8A;
const REGISTER_DIG_T3: u8 = 0x8C;
const REGISTER_DIG_P1: u8 = 0x8E;
const REGISTER_DIG_P2: u8 = 0x90;
const REGISTER_DIG_P3: u8 = 0x92;
const REGISTER_DIG_P4: u8 = 0x94;
const REGISTER_DIG_P5: u8 = 0x96;
const REGISTER_DIG_P6: u8 = 0x98;
const REGISTER_DIG_P7: u8 = 0x9A;
const REGISTER_DIG_P8: u8 = 0x9C;
const REGISTER_DIG_P9: u8 = 0x9E;
const REGISTER_DIG_H1: u8 = 0xA1;
const REGISTER_DIG_H2: u8 = 0xE1;
const REGISTER_DIG_H3: u8 = 0xE3;
const REGISTER_DIG_H4: u8 = 0xE4;
const REGISTER_DIG_H5: u8 = 0xE5;
const REGISTER_DIG_H6: u8 = 0xE7;

const REGISTER_ID: u8 = 0xD0;
const REGISTER_RESET: u8 = 0xE0;
const REGISTER_STATUS: u8 = 0xF3;
const REGISTER_PRESSURE: u8 = 0xF7;
const REGISTER_TEMPERATURE: u8 = 0xFA;
const REGISTER_HUMIDITY: u8 = 0xFD;
const REGISTER_HUMIDITY_CONTROL: u8 = 0xF2;
const REGISTER_MEASUREMENT_CONTROL: u8 = 0xF4;

#[derive(Debug)]
#[repr(u8)]
pub enum HumiditySampling {
    Skipped = 0b00000_000,
    Sampling1 = 0b00000_001,
    Sampling2 = 0b00000_010,
    Sampling4 = 0b00000_011,
    Sampling8 = 0b00000_100,
    Sampling16 = 0b00000_101,
}

#[derive(Debug)]
#[repr(u8)]
pub enum TemperatireSampling {
    Skipped = 0b000_000_00,
    Sampling1 = 0b001_000_00,
    Sampling2 = 0b010_000_00,
    Sampling4 = 0b011_000_00,
    Sampling8 = 0b100_000_00,
    Sampling16 = 0b101_000_00,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PressureSampling {
    Skipped = 0b000_000_00,
    Sampling1 = 0b000_001_00,
    Sampling2 = 0b000_010_00,
    Sampling4 = 0b000_011_00,
    Sampling8 = 0b000_100_00,
    Sampling16 = 0b000_101_00,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SensorMode {
    Sleep = 0b000000_00,
    Foeced = 0b000000_01,
    Normal = 0b000000_11,
}

#[derive(Debug)]
pub struct BmeStatus {
    measuring: bool,
    im_update: bool,
}

impl BmeStatus {
    pub fn new(measuring: bool, im_update: bool) -> BmeStatus {
        BmeStatus {
            measuring,
            im_update,
        }
    }

    pub fn measuring(&self) -> bool {
        self.measuring
    }

    pub fn im_update(&self) -> bool {
        self.im_update
    }
}

#[derive(Debug)]
pub struct Bme280 {
    address: u8,
    temprerature_fine: i32,
    compensation_data: CompensationData,
}

impl Bme280 {
    pub fn address_low<I2C>(i2c: &mut I2C) -> Result<Bme280, I2C::Error>
    where
        I2C: WriteRead,
    {
        let address = BME280_ADDRESS_LOW;
        let mut buffer = [0; 1];

        i2c.write_read(address, &[REGISTER_ID], &mut buffer)?;

        let compensation_data = CompensationData::read(i2c, address)?;

        Ok(Bme280 {
            address,
            temprerature_fine: 0,
            compensation_data,
        })
    }

    pub fn address_high<I2C>(i2c: &mut I2C) -> Result<Bme280, I2C::Error>
    where
        I2C: WriteRead,
    {
        let address = BME280_ADDRESS_HIGH;
        let compensation_data = CompensationData::read(i2c, address)?;

        Ok(Bme280 {
            address,
            temprerature_fine: 0,
            compensation_data,
        })
    }

    pub fn id<I2C>(&self, i2c: &mut I2C) -> Result<u8, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 1];

        i2c.write_read(self.address, &[REGISTER_ID], &mut buffer)?;

        Ok(buffer[0])
    }

    pub fn reset<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        i2c.write(self.address, &[REGISTER_RESET, BME280_RESET_VALUE])?;

        Ok(())
    }

    pub fn status<I2C>(&self, i2c: &mut I2C) -> Result<BmeStatus, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 1];

        i2c.write_read(self.address, &[REGISTER_STATUS], &mut buffer)?;

        Ok(BmeStatus::new(
            buffer[0] & 0b0000_1000 == 0,
            buffer[0] & 0b0000_0001 == 0,
        ))
    }

    pub fn sampling<I2C>(
        &self,
        i2c: &mut I2C,
        humidity: HumiditySampling,
        temperatire: TemperatireSampling,
        pressure: PressureSampling,
        mode: SensorMode,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        i2c.write(self.address, &[REGISTER_HUMIDITY_CONTROL, humidity as u8])?;
        i2c.write(
            self.address,
            &[
                REGISTER_MEASUREMENT_CONTROL,
                temperatire as u8 | pressure as u8 | mode as u8,
            ],
        )?;

        Ok(())
    }

    pub fn temperature<I2C>(&mut self, i2c: &mut I2C) -> Result<i32, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 3];
        i2c.write_read(self.address, &[REGISTER_TEMPERATURE], &mut buffer)?;

        let adc_temperature =
            ((buffer[0] as i32) << 12) | ((buffer[1] as i32) << 4) | ((buffer[2] as i32) >> 4);

        if adc_temperature == 0x080000 {
            return Ok(0);
        }

        let dig_t1 = self.compensation_data.dig_t1 as i32;
        let dig_t2 = self.compensation_data.dig_t2 as i32;
        let dig_t3 = self.compensation_data.dig_t3 as i32;

        let var1 = (((adc_temperature >> 3) - (dig_t1 << 1)) * dig_t2) >> 11;
        let var2 = (((((adc_temperature >> 4) - dig_t1) * ((adc_temperature >> 4) - dig_t1))
            >> 12)
            * dig_t3)
            >> 14;

        self.temprerature_fine = var1 + var2;

        let temperature = (self.temprerature_fine * 5 + 128) >> 8;

        Ok(temperature)
    }

    pub fn pressure<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 3];
        i2c.write_read(self.address, &[REGISTER_PRESSURE], &mut buffer)?;

        let adc_pressure =
            ((buffer[0] as i32) << 12) | ((buffer[1] as i32) << 4) | ((buffer[2] as i32) >> 4);

        if adc_pressure == 0x080000 {
            return Ok(0);
        }

        let dig_p1 = self.compensation_data.dig_p1 as i64;
        let dig_p2 = self.compensation_data.dig_p2 as i64;
        let dig_p3 = self.compensation_data.dig_p3 as i64;
        let dig_p4 = self.compensation_data.dig_p4 as i64;
        let dig_p5 = self.compensation_data.dig_p5 as i64;
        let dig_p6 = self.compensation_data.dig_p6 as i64;

        let var1 = self.temprerature_fine as i64 - 128000;
        let var2 = var1 * var1 * dig_p6;
        let var2 = var2 + ((var1 * dig_p5) << 17);
        let var2 = var2 + ((dig_p4) << 35);
        let var1 = ((var1 * var1 * dig_p3) >> 8) + ((var1 * dig_p2) << 12);
        let var1 = ((1 << 47) + var1) * (dig_p1) >> 33;

        if var1 == 0 {
            return Ok(0); // avoid exception caused by division by zero
        }

        let dig_p9 = self.compensation_data.dig_p9 as i64;
        let dig_p8 = self.compensation_data.dig_p8 as i64;
        let dig_p7 = self.compensation_data.dig_p7 as i64;

        let pressure = 1048576 - adc_pressure as i64;
        let pressure = (((pressure << 31) - var2) * 3125) / var1;
        let var1 = ((dig_p9) * (pressure >> 13) * (pressure >> 13)) >> 25;
        let var2 = ((dig_p8) * pressure) >> 19;
        let pressure = ((pressure + var1 + var2) >> 8) + (dig_p7 << 4);

        Ok(pressure as u32)
    }

    pub fn humidity<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 2];
        i2c.write_read(self.address, &[REGISTER_HUMIDITY], &mut buffer)?;

        let adc_humidity = ((buffer[0] as i32) << 8) | (buffer[1] as i32);

        if adc_humidity == 0x8000 {
            return Ok(0);
        }

        let dig_h1 = self.compensation_data.dig_h1 as i32;
        let dig_h2 = self.compensation_data.dig_h2 as i32;
        let dig_h3 = self.compensation_data.dig_h3 as i32;
        let dig_h4 = self.compensation_data.dig_h4 as i32;
        let dig_h5 = self.compensation_data.dig_h5 as i32;
        let dig_h6 = self.compensation_data.dig_h6 as i32;

        let v_x1_u32r = self.temprerature_fine - 76800;
        let v_x1_u32r = ((((adc_humidity << 14) - (dig_h4 << 20) - dig_h5 * v_x1_u32r) + 16384)
            >> 15)
            * (((((((v_x1_u32r * dig_h6) >> 10) * (((v_x1_u32r * dig_h3) >> 11) + 32768)) >> 10)
                + 2097152)
                * dig_h2
                + 8192)
                >> 14);
        let v_x1_u32r =
            v_x1_u32r - (((((v_x1_u32r >> 15) * (v_x1_u32r >> 15)) >> 7) * dig_h1) >> 4);
        let v_x1_u32r = if v_x1_u32r < 0 { 0 } else { v_x1_u32r };
        let v_x1_u32r = if v_x1_u32r > 419430400 {
            419430400
        } else {
            v_x1_u32r
        };

        Ok((v_x1_u32r >> 12) as u32)
    }
}

#[derive(Debug)]
struct CompensationData {
    dig_t1: u16, // Temperature compensation value.
    dig_t2: i16, // Temperature compensation value.
    dig_t3: i16, // Temperature compensation value.

    dig_p1: u16, // Pressure compensation value.
    dig_p2: i16, // Pressure compensation value.
    dig_p3: i16, // Pressure compensation value.
    dig_p4: i16, // Pressure compensation value.
    dig_p5: i16, // Pressure compensation value.
    dig_p6: i16, // Pressure compensation value.
    dig_p7: i16, // Pressure compensation value.
    dig_p8: i16, // Pressure compensation value.
    dig_p9: i16, // Pressure compensation value.

    dig_h1: u8,  // Humidity compensation value.
    dig_h2: i16, // Humidity compensation value.
    dig_h3: u8,  // Humidity compensation value.
    dig_h4: i16, // Humidity compensation value.
    dig_h5: i16, // Humidity compensation value.
    dig_h6: i8,  // Humidity compensation value.
}

impl CompensationData {
    pub fn read<I2C>(i2c: &mut I2C, address: u8) -> Result<CompensationData, I2C::Error>
    where
        I2C: WriteRead,
    {
        let dig_t1 = CompensationData::read_u16(i2c, address, REGISTER_DIG_T1)?;
        let dig_t2 = CompensationData::read_i16(i2c, address, REGISTER_DIG_T2)?;
        let dig_t3 = CompensationData::read_i16(i2c, address, REGISTER_DIG_T3)?;
        let dig_p1 = CompensationData::read_u16(i2c, address, REGISTER_DIG_P1)?;
        let dig_p2 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P2)?;
        let dig_p3 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P3)?;
        let dig_p4 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P4)?;
        let dig_p5 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P5)?;
        let dig_p6 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P6)?;
        let dig_p7 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P7)?;
        let dig_p8 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P8)?;
        let dig_p9 = CompensationData::read_i16(i2c, address, REGISTER_DIG_P9)?;
        let dig_h1 = CompensationData::read_u8(i2c, address, REGISTER_DIG_H1)?;
        let dig_h2 = CompensationData::read_i16(i2c, address, REGISTER_DIG_H2)?;
        let dig_h3 = CompensationData::read_u8(i2c, address, REGISTER_DIG_H3)?;
        let dig_h4 = CompensationData::read_h4(i2c, address, REGISTER_DIG_H4)?;
        let dig_h5 = CompensationData::read_h5(i2c, address, REGISTER_DIG_H5)?;
        let dig_h6 = CompensationData::read_i8(i2c, address, REGISTER_DIG_H6)?;

        Ok(CompensationData {
            dig_t1,
            dig_t2,
            dig_t3,
            dig_p1,
            dig_p2,
            dig_p3,
            dig_p4,
            dig_p5,
            dig_p6,
            dig_p7,
            dig_p8,
            dig_p9,
            dig_h1,
            dig_h2,
            dig_h3,
            dig_h4,
            dig_h5,
            dig_h6,
        })
    }

    fn read_i8<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<i8, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 1];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok(i8::from_le_bytes(buffer))
    }

    fn read_u8<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<u8, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 1];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok(u8::from_le_bytes(buffer))
    }

    fn read_i16<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<i16, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 2];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok(i16::from_le_bytes(buffer))
    }

    fn read_u16<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<u16, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 2];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok(u16::from_le_bytes(buffer))
    }

    fn read_h4<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<i16, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 2];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok((buffer[0] as i16) << 4 | (buffer[1] & 0x0f) as i16)
    }

    fn read_h5<I2C>(i2c: &mut I2C, address: u8, register: u8) -> Result<i16, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut buffer = [0; 2];

        i2c.write_read(address, &[register], &mut buffer)?;

        Ok((((buffer[0] & 0xf0) as i16) << 4) | (buffer[1] as i16))
    }
}
