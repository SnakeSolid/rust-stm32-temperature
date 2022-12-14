#[allow(non_snake_case)]
pub trait TemperatureExt {
    fn c(self) -> (i8, u8);
    fn f(self) -> (i8, u8);
}

const CELSIUS: i32 = 100;
const FAHRENHEIT: i32 = 100;

impl TemperatureExt for i32 {
    fn c(self) -> (i8, u8) {
        let integer = self / CELSIUS;
        let fractional = (100 * (self.abs() % CELSIUS)) / CELSIUS;

        (integer as i8, fractional as u8)
    }

    fn f(self) -> (i8, u8) {
        let fahrenheit = self * 9 / 5 + 3_200;
        let integer = fahrenheit / FAHRENHEIT;
        let fractional = (100 * (fahrenheit.abs() % FAHRENHEIT)) / FAHRENHEIT;

        (integer as i8, fractional as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius() {
        assert_eq!(5123.c(), (51, 23));
        assert_eq!((-5123).c(), (-51, 23));
    }

    #[test]
    fn fahrenheit() {
        assert_eq!(5123.f(), (124, 21));
        assert_eq!((-5123).f(), (-60, 21));
    }
}
