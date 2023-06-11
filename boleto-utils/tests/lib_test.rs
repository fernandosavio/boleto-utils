#[cfg(test)]
mod tests {
    use boleto_utils::{Boleto, BoletoError, cobranca::CodigoMoeda};
    use chrono::NaiveDate;

    #[test]
    fn invalid_input_error() {
        let input = b"A".repeat(44);
        let result = Boleto::new(&input);

        assert!(matches!(result, Err(BoletoError::NumbersOnly)));
    }

    #[test]
    fn cod_moeda_validation() {
        let invalid_inputs = [
            b"11111444455555555556666666666666666666666666",
            b"11121444455555555556666666666666666666666666",
            b"11131444455555555556666666666666666666666666",
            b"11141444455555555556666666666666666666666666",
            b"11151444455555555556666666666666666666666666",
            b"11161444455555555556666666666666666666666666",
            b"11171444455555555556666666666666666666666666",
            b"11181444455555555556666666666666666666666666",
        ];

        for input in invalid_inputs {
            let result = Boleto::new(input);
            assert!(matches!(result, Err(BoletoError::InvalidCodigoMoeda)));
        }

        let boleto = Boleto::new(b"11191444455555555556666666666666666666666666").unwrap();

        let Boleto::Cobranca(cob) = boleto else {
            panic!("Não é cobrança válida");
        };

        assert!(
            matches!(cob.cod_moeda, CodigoMoeda::Real),
            "cod_moeda should be 'Real'",
        );

        let boleto = Boleto::new(b"11105444455555555556666666666666666666666666").unwrap();

        let Boleto::Cobranca(cob) = boleto else {
            panic!("Não é cobrança válida");
        };

        assert!(
            matches!(cob.cod_moeda, CodigoMoeda::Outras),
            "cod_moeda should be 'Outras'",
        );

    }

    #[test]
    fn cod_banco_validation() {
        let barcodes: [(&[u8], u16); 5] = [
            (b"11191444455555555556666666666666666666666666", 111),
            (b"99996444455555555556666666666666666666666666", 999),
            (b"12395444455555555556666666666666666666666666", 123),
            (b"66691444455555555556666666666666666666666666", 666),
            (b"00091444455555555556666666666666666666666666", 0),
        ];

        for (barcode, expected) in barcodes.iter() {
            let boleto = Boleto::new(barcode).unwrap();

            let Boleto::Cobranca(result) = boleto else {
                panic!("Barcode should be considered valid.");
            };

            assert_eq!(result.cod_banco, *expected);
        }
    }

    #[test]
    fn fator_vencimento_validation() {
        let barcodes = [
            (b"11196000055555555556666666666666666666666666", 0_u16, None),
            (
                b"11199100055555555556666666666666666666666666",
                1000,
                Some(NaiveDate::from_ymd_opt(2025, 2, 22).unwrap()),
            ),
            (
                b"11191100255555555556666666666666666666666666",
                1002,
                Some(NaiveDate::from_ymd_opt(2025, 2, 24).unwrap()),
            ),
            (
                b"11196166755555555556666666666666666666666666",
                1667,
                Some(NaiveDate::from_ymd_opt(2026, 12, 21).unwrap()),
            ),
            (
                b"11198478955555555556666666666666666666666666",
                4789,
                Some(NaiveDate::from_ymd_opt(2010, 11, 17).unwrap()),
            ),
            (
                b"11193999955555555556666666666666666666666666",
                9999,
                Some(NaiveDate::from_ymd_opt(2025, 2, 21).unwrap()),
            ),
            (
                b"75696903800002500001434301033723400014933001",
                9038,
                Some(NaiveDate::from_ymd_opt(2022, 7, 6).unwrap()),
            ),
            (
                b"00191667900002434790000002656973019362470618",
                6679,
                Some(NaiveDate::from_ymd_opt(2016, 1, 20).unwrap()),
            ),
            (
                b"00195586200000773520000002464206011816073018",
                5862,
                Some(NaiveDate::from_ymd_opt(2013, 10, 25).unwrap()),
            ),
            (
                b"75592896700003787000003389850761252543475984",
                8967,
                Some(NaiveDate::from_ymd_opt(2022, 4, 26).unwrap()),
            ),
            (
                b"23791672000003249052028269705944177105205220",
                6720,
                Some(NaiveDate::from_ymd_opt(2016, 3, 1).unwrap()),
            ),
            (
                b"23791672000003097902028060007024617500249000",
                6720,
                Some(NaiveDate::from_ymd_opt(2016, 3, 1).unwrap()),
            ),
        ];
        for (barcode, expected_fator, expected_date) in barcodes {
            let boleto = Boleto::new(barcode.as_slice()).unwrap();

            let Boleto::Cobranca(result) = boleto else {
                panic!("Barcode should be considered valid.");
            };

            assert_eq!(result.fator_vencimento, expected_fator);
            assert_eq!(result.data_vencimento, expected_date);
        }

        assert!(
            matches!(
                Boleto::new(b"11196000155555555556666666666666666666666666".as_slice()),
                Err(BoletoError::InvalidFatorVencimento),
            )
        );

        assert!(
            matches!(
                Boleto::new(b"11196099955555555556666666666666666666666666".as_slice()),
                Err(BoletoError::InvalidFatorVencimento),
            )
        );
    }

    #[test]
    fn valor_validation() {
        let barcodes = [
            (
                b"11191444455555555556666666666666666666666666",
                Some(55555555.55_f64),
            ),
            (
                b"11196444499999999996666666666666666666666666",
                Some(99999999.99),
            ),
            (b"11193444400000000006666666666666666666666666", None),
        ];
        for (barcode, expected) in barcodes.iter() {
            let boleto = Boleto::new(barcode.as_slice()).unwrap();
            let Boleto::Cobranca(result) = boleto else {
                panic!("Barcode should be considered valid.");
            };
            assert_eq!(result.valor, *expected);
        }
    }

    #[test]
    fn digito_verificador_validation() {
        let barcodes = [
            (b"11191444455555555556666666666666666666666666", 1_u8),
            (b"10499898100000214032006561000100040099726390", 9_u8),
            (b"75696903800002500001434301033723400014933001", 6_u8),
            (b"00191667900002434790000002656973019362470618", 1_u8),
            (b"00195586200000773520000002464206011816073018", 5_u8),
            (b"75592896700003787000003389850761252543475984", 2_u8),
            (b"23791672000003249052028269705944177105205220", 1_u8),
            (b"23791672000003097902028060007024617500249000", 1_u8),
            (b"11191100255555555556666666666666666666666666", 1_u8),
        ];

        for (barcode, expected) in barcodes.iter() {
            let boleto = Boleto::new(barcode.as_slice()).unwrap();
            let Boleto::Cobranca(result) = boleto else {
                panic!("Barcode should be considered valid.");
            };
            assert_eq!(result.digito_verificador, *expected);
        }
    }

    #[test]
    fn invalid_length_error() {
        let invalid_lengths = [1, 10, 20, 30, 40, 43, 46, 45, 49, 50];

        for i in invalid_lengths {
            let input = b"0".repeat(i);
            let result = Boleto::new(&input);

            assert!(matches!(result, Err(BoletoError::InvalidLength)));
       }
    }
}