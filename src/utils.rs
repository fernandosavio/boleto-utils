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

pub mod barcode_utils {
    pub fn barcode_to_digitable_line(barcode: &str) -> String {
        String::from(barcode)
    }

    pub fn digitable_line_to_barcode(digitable_line: &str) -> String {
        String::from(digitable_line)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::utils::{date_to_fator_vencimento, fator_vencimento_to_date};

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
}
