pub mod fator_vencimento {
    use chrono::{NaiveDate, Duration};
    use lazy_static::lazy_static;

    fn get_base_date() -> NaiveDate {
        lazy_static! {
            static ref BASE_DATE: NaiveDate = NaiveDate::from_ymd(1997, 10, 7);
        }
        *BASE_DATE
    }

    pub fn to_date(fator: u32) -> NaiveDate {
        get_base_date() + Duration::days(fator.into())
    }

    pub fn from_date(date: NaiveDate) -> Option<u32> {
        let diff_days = date - get_base_date();
        diff_days.num_days().try_into().ok()
    }
}

pub mod barcode_utils {
    pub fn barcode_to_digitable_line(barcode: &str) -> String {
        todo!();
    }

    pub fn digitable_line_to_barcode(digitable_line: &str) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::utils::fator_vencimento;

    
    #[test]
    fn convert_fator_vencimento_to_naive_date_correctly() {
        
        assert_eq!(
            fator_vencimento::to_date(1000),
            NaiveDate::from_ymd(2000, 7, 3),
        );
        assert_eq!(
            fator_vencimento::to_date(1002),
            NaiveDate::from_ymd(2000, 7, 5),
        );
        assert_eq!(
            fator_vencimento::to_date(1667),
            NaiveDate::from_ymd(2002, 5, 1),
        );
        assert_eq!(
            fator_vencimento::to_date(4789),
            NaiveDate::from_ymd(2010, 11, 17),
        );
        assert_eq!(
            fator_vencimento::to_date(9999),
            NaiveDate::from_ymd(2025, 2, 21),
        );
    }

    #[test]
    fn convert_naive_date_to_fator_vencimento_correctly() {
        
        assert_eq!(
            fator_vencimento::from_date(NaiveDate::from_ymd(2000, 7, 3)),
            Some(1000),
        );
        assert_eq!(
            fator_vencimento::from_date(NaiveDate::from_ymd(2000, 7, 5)),
            Some(1002),
        );
        assert_eq!(
            fator_vencimento::from_date(NaiveDate::from_ymd(2002, 5, 1)),
            Some(1667),
        );
        assert_eq!(
            fator_vencimento::from_date(NaiveDate::from_ymd(2010, 11, 17)),
            Some(4789),
        );
        assert_eq!(
            fator_vencimento::from_date(NaiveDate::from_ymd(2025, 2, 21)),
            Some(9999),
        );
    }
}