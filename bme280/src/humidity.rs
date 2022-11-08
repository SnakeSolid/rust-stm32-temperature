#[allow(non_snake_case)]
pub trait HumidityExt {
    fn percent(self) -> (u8, u16);
}

const FACTOR: u32 = 1024;

impl HumidityExt for u32 {
    fn percent(self) -> (u8, u16) {
        let integer = self / FACTOR;
        let fractional = (1_000 * (self % FACTOR)) / FACTOR;

        (integer as u8, fractional as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent() {
        assert_eq!(47445.percent(), (46, 333));
    }
}
