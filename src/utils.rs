use std::convert::TryInto;

use chrono::{Duration, NaiveDate};

fn get_base_date() -> NaiveDate {
    NaiveDate::from_ymd(1997, 10, 7)
}

pub fn fator_vencimento_to_date(fator: u16) -> Option<NaiveDate> {
    if fator > 0 {
        Some(get_base_date() + Duration::days(fator.into()))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn date_to_fator_vencimento(date: NaiveDate) -> Option<u16> {
    (date - get_base_date()).num_days().try_into().ok()
}

pub fn u8_array_to_u16(slice: &[u8]) -> u16 {
    slice.iter()
        .rev()
        .enumerate()
        .map(|(i, n)| {
            10_u16.pow(i.try_into().unwrap()) * (*n - b'0') as u16
        })
        .sum()
}

pub mod dv_utils {
    pub fn mod_10<'a, I>(values: I) -> u8
    where
        I: DoubleEndedIterator<Item = &'a u8>
    {
        let soma: u8 = values.rev()
            .zip([2, 1].iter().cycle())
            .map(|(n, i)| {
                match (n - b'0') * i {
                    x if x > 9 => (x / 10) + (x % 10),
                    x => x,
                }
            })
            .sum();

        ((10 - (soma % 10)) % 10) + b'0'
    }

    pub fn mod_11<'a, I>(values: I) -> Option<u8>
    where
        I: DoubleEndedIterator<Item = &'a u8>
    {
        let soma: u32 = values.rev()
            .zip((2..=9).cycle())
            .map(|(n, i)| (n - b'0') as u32 * i)
            .sum();

        match 11 - (soma % 11) {
            10 | 11 => None,
            dv => Some((dv as u8) + b'0'),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::utils::{date_to_fator_vencimento, fator_vencimento_to_date, u8_array_to_u16};
    use crate::utils::dv_utils::{mod_10, mod_11};

    #[test]
    fn convert_fator_vencimento_to_naive_date_correctly() {
        assert_eq!(
            fator_vencimento_to_date(1000),
            Some(NaiveDate::from_ymd(2000, 7, 3)),
        );
        assert_eq!(
            fator_vencimento_to_date(1002),
            Some(NaiveDate::from_ymd(2000, 7, 5)),
        );
        assert_eq!(
            fator_vencimento_to_date(1667),
            Some(NaiveDate::from_ymd(2002, 5, 1)),
        );
        assert_eq!(
            fator_vencimento_to_date(4789),
            Some(NaiveDate::from_ymd(2010, 11, 17)),
        );
        assert_eq!(
            fator_vencimento_to_date(9999),
            Some(NaiveDate::from_ymd(2025, 2, 21)),
        );
    }

    #[test]
    fn convert_naive_date_to_fator_vencimento_correctly() {
        assert_eq!(
            date_to_fator_vencimento(NaiveDate::from_ymd(2000, 7, 3)),
            Some(1000),
        );
        assert_eq!(
            date_to_fator_vencimento(NaiveDate::from_ymd(2000, 7, 5)),
            Some(1002),
        );
        assert_eq!(
            date_to_fator_vencimento(NaiveDate::from_ymd(2002, 5, 1)),
            Some(1667),
        );
        assert_eq!(
            date_to_fator_vencimento(NaiveDate::from_ymd(2010, 11, 17)),
            Some(4789),
        );
        assert_eq!(
            date_to_fator_vencimento(NaiveDate::from_ymd(2025, 2, 21)),
            Some(9999),
        );
    }

    #[test]
    fn calculate_mod_10_correctly() {
        assert_eq!(mod_10(b"01230067896".iter()), b'3');
        assert_eq!(mod_10(b"01230167896".iter()), b'2');
        assert_eq!(mod_10(b"01230267896".iter()), b'1');
        assert_eq!(mod_10(b"01230367896".iter()), b'0');
        assert_eq!(mod_10(b"01230467896".iter()), b'9');
        assert_eq!(mod_10(b"01230567896".iter()), b'8');
        assert_eq!(mod_10(b"01230667896".iter()), b'7');
        assert_eq!(mod_10(b"01230767896".iter()), b'6');
        assert_eq!(mod_10(b"01230867896".iter()), b'5');
        assert_eq!(mod_10(b"01230967896".iter()), b'4');
    }

    #[test]
    fn calculate_mod_11_correctly() {
        assert_eq!(mod_11(b"8220000215048200974123220154098290108605940".iter()), None);
        assert_eq!(mod_11(b"01230067896".iter()), None);
        assert_eq!(mod_11(b"31230067896".iter()), None);
        assert_eq!(mod_11(b"01231068896".iter()), Some(b'9'));
        assert_eq!(mod_11(b"01230267896".iter()), Some(b'8'));
        assert_eq!(mod_11(b"01231167896".iter()), Some(b'7'));
        assert_eq!(mod_11(b"01232067896".iter()), Some(b'6'));
        assert_eq!(mod_11(b"01241067896".iter()), Some(b'5'));
        assert_eq!(mod_11(b"01250067896".iter()), Some(b'4'));
        assert_eq!(mod_11(b"02232067896".iter()), Some(b'3'));
        assert_eq!(mod_11(b"01250067897".iter()), Some(b'2'));
        assert_eq!(mod_11(b"01230367896".iter()), Some(b'1'));
    }

    #[test]
    fn convert_u8_array_to_u32_correctly() {
        assert_eq!(u8_array_to_u16(b"001"), 1);
        assert_eq!(u8_array_to_u16(b"010"), 10);
        assert_eq!(u8_array_to_u16(b"100"), 100);
        assert_eq!(u8_array_to_u16(b"999"), 999);
        assert_eq!(u8_array_to_u16(b"9999"), 9999);
    }
}
