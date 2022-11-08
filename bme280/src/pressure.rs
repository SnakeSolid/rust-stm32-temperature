#[allow(non_snake_case)]
pub trait PressureExt {
    fn pa(self) -> (u32, u8);
    fn hpa(self) -> (u16, u16);
    fn mmhg(self) -> (u16, u8);
}

const PASCAL: u32 = 256;
const HECTOPASCAL: u32 = 25_600;
const MILLIMETRE_MERCURY: u32 = 34130;

impl PressureExt for u32 {
    fn pa(self) -> (u32, u8) {
        let integer = self / PASCAL;
        let fractional = (100 * (self % PASCAL)) / PASCAL;

        (integer, fractional as u8)
    }

    fn hpa(self) -> (u16, u16) {
        let integer = self / HECTOPASCAL;
        let fractional = (1000 * (self % HECTOPASCAL)) / HECTOPASCAL;

        (integer as u16, fractional as u16)
    }

    fn mmhg(self) -> (u16, u8) {
        let integer = self / MILLIMETRE_MERCURY;
        let fractional = (10 * (self % MILLIMETRE_MERCURY)) / MILLIMETRE_MERCURY;

        (integer as u16, fractional as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascals() {
        assert_eq!(24674867.pa(), (96386, 19));
    }

    #[test]
    fn hectopascals() {
        assert_eq!(24674867.hpa(), (963, 861));
    }

    #[test]
    fn millimetre_mercury() {
        assert_eq!(24674867.mmhg(), (722, 9));
    }
}
