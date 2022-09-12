use std::ops::Range;

use crate::utils::{dv_utils, fator_vencimento_to_date};
use crate::BoletoError;
use crate::instituicoes_bancarias::InfoBanco;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum CodigoMoeda {
    Real,
    Outras,
}

#[derive(Debug)]
pub struct Cobranca {
    pub cod_banco: u16,
    pub info_banco: Option<&'static InfoBanco>,
    pub cod_moeda: CodigoMoeda,
    pub digito_verificador: u8,
    pub fator_vencimento: u16,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: Option<f64>,
}

impl Cobranca {
    pub fn new(barcode: &str) -> Result<Self, BoletoError> {
        let _ = Self::validate(barcode)?;

        const ID_BANCO: Range<usize> = 0..3;
        const COD_MOEDA: Range<usize> = 3..4;
        const DIG_VERIF: Range<usize> = 4..5;
        const FATOR_VENC: Range<usize> = 5..9;
        const VALOR: Range<usize> = 9..19;
        // const CAMPO_LIVRE: Range<usize> = 19..44;

        let cod_banco = barcode[ID_BANCO]
            .parse()
            .expect("cod_banco deve ser numérico");

        let cod_moeda = match barcode[COD_MOEDA]
            .parse()
            .expect("cod_banco deve ser numérico")
        {
            9 => CodigoMoeda::Real,
            0 => CodigoMoeda::Outras,
            _ => return Err(BoletoError::InvalidCodigoMoeda),
        };

        let fator_vencimento: u16 = barcode[FATOR_VENC].parse().unwrap();

        let valor = match barcode[VALOR].parse::<f64>().unwrap() {
            x if x.is_normal() => Some(x / 100.00),
            _ => None,
        };

        let digito_verificador: u8 = barcode[DIG_VERIF].parse().unwrap();

        if digito_verificador != Self::calculate_digito_verificador(barcode)? {
            return Err(BoletoError::InvalidDigitoVerificador);
        }

        Ok(Self {
            cod_banco,
            info_banco: InfoBanco::get_by_id(cod_banco),
            cod_moeda,
            fator_vencimento,
            digito_verificador,
            data_vencimento: fator_vencimento_to_date(fator_vencimento),
            valor,
        })
    }

    pub fn validate(barcode: &str) -> Result<(), BoletoError> {
        if barcode.len() != 44 && barcode.len() != 47 {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = barcode.chars().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        if barcode.bytes().next() == Some(b'8') {
            return Err(BoletoError::InvalidCobrancaBarcode);
        }

        Ok(())
    }

    pub fn linha_digitavel_to_cod_barras(digitable_line: &str) -> Result<String, BoletoError> {
        let _ = Self::validate(digitable_line)?;

        if digitable_line.len() != 47 {
            return Err(BoletoError::InvalidLength);
        }

        let mut barcode = String::with_capacity(44);

        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV

        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        barcode.push_str(&digitable_line[0..4]);
        barcode.push_str(&digitable_line[32..47]);
        barcode.push_str(&digitable_line[4..9]);
        barcode.push_str(&digitable_line[10..20]);
        barcode.push_str(&digitable_line[21..31]);

        Ok(barcode)
    }

    pub fn cod_barras_to_linha_digitavel(barcode: &str) -> Result<String, BoletoError> {
        let _ = Self::validate(barcode)?;

        if barcode.len() != 44 {
            return Err(BoletoError::InvalidLength);
        }

        let mut digitable_line = String::with_capacity(47);
        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV

        // Campo 1
        digitable_line.push_str(&barcode[0..4]);
        digitable_line.push_str(&barcode[19..24]);

        let mut dv_campo = dv_utils::mod_10_alternating_2_1(digitable_line[0..9].bytes());
        digitable_line.push_str(&dv_campo.to_string());

        // Campo 2
        digitable_line.push_str(&barcode[24..34]);
        dv_campo = dv_utils::mod_10_alternating_2_1(digitable_line[10..20].bytes());
        digitable_line.push_str(&dv_campo.to_string());

        // Campo 3
        digitable_line.push_str(&barcode[34..44]);
        dv_campo = dv_utils::mod_10_alternating_2_1(digitable_line[21..31].bytes());
        digitable_line.push_str(&dv_campo.to_string());

        // DV
        digitable_line.push_str(&barcode[4..5]);

        // Campo 4
        digitable_line.push_str(&barcode[5..19]);

        Ok(digitable_line)
    }

    fn calculate_digito_verificador(barcode: &str) -> Result<u8, BoletoError> {
        let _ = Self::validate(barcode)?;

        // Cria um iterator que itera sobre os caracteres do código de barras
        // exceto o dígito verificador
        let iterator_without_dv = barcode[..4]
            .as_bytes()
            .iter()
            .chain(barcode[5..].as_bytes().iter())
            .map(|x| *x);

        Ok(
            dv_utils::mod_11_alternating_2_to_9(iterator_without_dv, 1)
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{Cobranca, CodigoMoeda};

    #[test]
    fn get_cod_banco_correctly() {
        let barcodes: [(&str, u16); 5] = [
            ("11191444455555555556666666666666666666666666", 111),
            ("99996444455555555556666666666666666666666666", 999),
            ("12395444455555555556666666666666666666666666", 123),
            ("66691444455555555556666666666666666666666666", 666),
            ("00091444455555555556666666666666666666666666", 0),
        ];
        for (barcode, expected) in barcodes.iter() {
            match Cobranca::new(barcode) {
                Err(e) => {
                    panic!("Barcode should be considered valid. ({:?})", e);
                }
                Ok(result) => {
                    assert_eq!(result.cod_banco, *expected)
                }
            };
        }
    }

    #[test]
    fn get_cod_moeda_correctly() {
        match Cobranca::new("11191444455555555556666666666666666666666666") {
            Ok(result) => {
                assert!(
                    matches!(result.cod_moeda, CodigoMoeda::Real),
                    "cod_moeda should be 'Real'",
                );
            }
            Err(e) => {
                panic!("Barcode should be considered valid. ({:?})", e);
            }
        };

        match Cobranca::new("11105444455555555556666666666666666666666666") {
            Ok(result) => {
                assert!(
                    matches!(result.cod_moeda, CodigoMoeda::Outras),
                    "cod_moeda should be 'Outras'",
                );
            }
            Err(e) => {
                panic!("Barcode should be considered valid. ({:?})", e);
            }
        };
    }

    #[test]
    fn get_fator_vencimento_correctly() {
        let barcodes = [
            ("11196000055555555556666666666666666666666666", 0_u16, None),
            (
                "11199100055555555556666666666666666666666666",
                1000,
                Some(NaiveDate::from_ymd(2000, 7, 3)),
            ),
            (
                "11191100255555555556666666666666666666666666",
                1002,
                Some(NaiveDate::from_ymd(2000, 7, 5)),
            ),
            (
                "11196166755555555556666666666666666666666666",
                1667,
                Some(NaiveDate::from_ymd(2002, 5, 1)),
            ),
            (
                "11198478955555555556666666666666666666666666",
                4789,
                Some(NaiveDate::from_ymd(2010, 11, 17)),
            ),
            (
                "11193999955555555556666666666666666666666666",
                9999,
                Some(NaiveDate::from_ymd(2025, 2, 21)),
            ),
            (
                "75696903800002500001434301033723400014933001",
                9038,
                Some(NaiveDate::from_ymd(2022, 7, 6)),
            ),
            (
                "00191667900002434790000002656973019362470618",
                6679,
                Some(NaiveDate::from_ymd(2016, 1, 20)),
            ),
            (
                "00195586200000773520000002464206011816073018",
                5862,
                Some(NaiveDate::from_ymd(2013, 10, 25)),
            ),
            (
                "75592896700003787000003389850761252543475984",
                8967,
                Some(NaiveDate::from_ymd(2022, 4, 26)),
            ),
            (
                "23791672000003249052028269705944177105205220",
                6720,
                Some(NaiveDate::from_ymd(2016, 3, 1)),
            ),
            (
                "23791672000003097902028060007024617500249000",
                6720,
                Some(NaiveDate::from_ymd(2016, 3, 1)),
            ),
        ];
        for (barcode, expected_fator, expected_date) in barcodes.iter() {
            match Cobranca::new(barcode) {
                Err(e) => {
                    panic!("Barcode should be considered valid. ({:?})", e);
                }
                Ok(result) => {
                    assert_eq!(result.fator_vencimento, *expected_fator);
                    assert_eq!(result.data_vencimento, *expected_date);
                }
            };
        }
    }

    #[test]
    fn get_valor_correctly() {
        let barcodes = [
            (
                "11191444455555555556666666666666666666666666",
                Some(55555555.55_f64),
            ),
            (
                "11196444499999999996666666666666666666666666",
                Some(99999999.99),
            ),
            ("11193444400000000006666666666666666666666666", None),
        ];
        for (barcode, expected) in barcodes.iter() {
            match Cobranca::new(barcode) {
                Err(e) => {
                    panic!("Barcode should be considered valid. ({:?}): {}", e, barcode);
                }
                Ok(result) => {
                    assert_eq!(result.valor, *expected);
                }
            };
        }
    }

    #[test]
    fn validate_digito_verificador_correctly() {
        let barcodes = [
            ("11191444455555555556666666666666666666666666", 1),
            ("10499898100000214032006561000100040099726390", 9),
            ("75696903800002500001434301033723400014933001", 6),
            ("00191667900002434790000002656973019362470618", 1),
            ("00195586200000773520000002464206011816073018", 5),
            ("75592896700003787000003389850761252543475984", 2),
            ("23791672000003249052028269705944177105205220", 1),
            ("23791672000003097902028060007024617500249000", 1),
            ("11191100255555555556666666666666666666666666", 1),
        ];

        for (barcode, expected) in barcodes.iter() {
            assert_eq!(
                Cobranca::calculate_digito_verificador(barcode).unwrap(),
                *expected,
            );
        }
    }

    #[test]
    fn validate_converting_barcode_to_linha_digitavel() {
        let barcodes = [
            (
                "75691434360103372340200149330011690380000250000",
                "75696903800002500001434301033723400014933001",
            ),
            (
                "00190000090265697301993624706185166790000243479",
                "00191667900002434790000002656973019362470618",
            ),
            (
                "00190000090246420601618160730182558620000077352",
                "00195586200000773520000002464206011816073018",
            ),
            (
                "75590003318985076125825434759848289670000378700",
                "75592896700003787000003389850761252543475984",
            ),
            (
                "23792028296970594417671052052207167200000324905",
                "23791672000003249052028269705944177105205220",
            ),
            (
                "23792028036000702461975002490003167200000309790",
                "23791672000003097902028060007024617500249000",
            ),
        ];

        for (linha_digitavel, barcode) in barcodes.iter() {
            assert_eq!(
                Cobranca::cod_barras_to_linha_digitavel(barcode).unwrap(),
                *linha_digitavel,
            );
        }
    }

    #[test]
    fn validate_converting_linha_digitavel_to_barcode() {
        let barcodes = [
            (
                "75691434360103372340200149330011690380000250000",
                "75696903800002500001434301033723400014933001",
            ),
            (
                "00190000090265697301993624706185166790000243479",
                "00191667900002434790000002656973019362470618",
            ),
            (
                "00190000090246420601618160730182558620000077352",
                "00195586200000773520000002464206011816073018",
            ),
            (
                "75590003318985076125825434759848289670000378700",
                "75592896700003787000003389850761252543475984",
            ),
            (
                "23792028296970594417671052052207167200000324905",
                "23791672000003249052028269705944177105205220",
            ),
            (
                "23792028036000702461975002490003167200000309790",
                "23791672000003097902028060007024617500249000",
            ),
        ];

        for (linha_digitavel, barcode) in barcodes.iter() {
            assert_eq!(
                Cobranca::linha_digitavel_to_cod_barras(linha_digitavel).unwrap(),
                *barcode,
            );
        }
    }
}
