use std::{str,fmt};


use crate::utils::{dv_utils, fator_vencimento_to_date, u8_array_to_u16};
use crate::BoletoError;
use crate::instituicoes_bancarias::InfoBanco;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum CodigoMoeda {
    Real,
    Outras,
}

pub struct Cobranca {
    pub cod_barras: [u8; 44],
    pub linha_digitavel: [u8; 47],
    pub cod_banco: u16,
    pub info_banco: Option<&'static InfoBanco>,
    pub cod_moeda: CodigoMoeda,
    pub digito_verificador: u8,
    pub fator_vencimento: u16,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: Option<f64>,
}

impl fmt::Debug for Cobranca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arrecadacao")
            .field("cod_barras", &str::from_utf8(&self.cod_barras).unwrap())
            .field("linha_digitavel", &str::from_utf8(&self.linha_digitavel).unwrap())
            .field("cod_banco", &self.cod_banco)
            .field("info_banco", &self.info_banco)
            .field("cod_moeda", &self.cod_moeda)
            .field("digito_verificador", &(self.digito_verificador - b'0'))
            .field("fator_vencimento", &self.fator_vencimento)
            .field("data_vencimento", &self.data_vencimento)
            .field("valor", &self.valor)
            .finish()
    }
}


impl Cobranca {
    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        let _ = Self::validate(value)?;

        let (cod_barras,linha_digitavel) = match value.len() {
            44 => {
                let mut barcode = [0_u8; 44];
                barcode.copy_from_slice(value);

                (barcode, Self::cod_barras_to_linha_digitavel(&barcode)?)
            },
            47 => {
                let mut digitable_line = [0_u8; 47];
                digitable_line.copy_from_slice(value);

                (Self::linha_digitavel_to_cod_barras(&digitable_line)?, digitable_line)
            },
            _ => return Err(BoletoError::InvalidLength),
        };

        let cod_banco: u16 = u8_array_to_u16(&cod_barras[0..3]);

        let cod_moeda = match cod_barras[3] {
            b'9' => CodigoMoeda::Real,
            b'0' => CodigoMoeda::Outras,
            _ => return Err(BoletoError::InvalidCodigoMoeda),
        };

        let fator_vencimento: u16 = u8_array_to_u16(&cod_barras[5..9]);

        let valor = {
            let x = unsafe { str::from_utf8_unchecked(&cod_barras[9..19]) };
            match  x.parse::<f64>().unwrap()
            {
                x if x.is_normal() => Some(x / 100.00),
                _ => None,
            }
        };

        let digito_verificador: u8 = cod_barras[4];

        if digito_verificador != Self::calculate_digito_verificador(&cod_barras)? {
            return Err(BoletoError::InvalidDigitoVerificador);
        }

        Ok(Self {
            cod_barras,
            linha_digitavel,
            cod_banco,
            info_banco: InfoBanco::get_by_id(cod_banco),
            cod_moeda,
            fator_vencimento,
            digito_verificador,
            data_vencimento: fator_vencimento_to_date(fator_vencimento),
            valor,
        })
    }

    pub fn validate(barcode: &[u8]) -> Result<(), BoletoError> {
        let only_numbers = barcode.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        if barcode.first() == Some(&b'8') {
            return Err(BoletoError::InvalidCobrancaBarcode);
        }

        Ok(())
    }

    pub fn linha_digitavel_to_cod_barras(digitable_line: &[u8; 47]) -> Result<[u8; 44], BoletoError> {
        let _ = Self::validate(digitable_line)?;

        let mut barcode = [0_u8; 44];

        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV

        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        barcode[0..4].copy_from_slice(&digitable_line[0..4]);
        barcode[4..19].copy_from_slice(&digitable_line[32..47]);
        barcode[19..24].copy_from_slice(&digitable_line[4..9]);
        barcode[24..34].copy_from_slice(&digitable_line[10..20]);
        barcode[34..44].copy_from_slice(&digitable_line[21..31]);

        Ok(barcode)
    }

    pub fn cod_barras_to_linha_digitavel(barcode: &[u8; 44]) -> Result<[u8; 47], BoletoError> {
        let _ = Self::validate(barcode)?;

        let mut digitable_line = [0_u8; 47];

        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        // 75696903800002500001434301033723400014933001


        // 75691 4343X 01033 72340X 00149 33001X 6 90380000250000
        // 21212 1212


        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV
        // 75691.43436 01033.723402 00149.330011 6 90380000250000

        // Campo 1
        digitable_line[0..4].copy_from_slice(&barcode[0..4]);
        digitable_line[4..9].copy_from_slice(&barcode[19..24]);
        digitable_line[9] = dv_utils::mod_10(digitable_line[0..9].iter());

        // Campo 2
        digitable_line[10..20].copy_from_slice(&barcode[24..34]);
        digitable_line[20] = dv_utils::mod_10(digitable_line[10..20].iter());

        // Campo 3
        digitable_line[21..31].copy_from_slice(&barcode[34..44]);
        digitable_line[31] = dv_utils::mod_10(digitable_line[21..31].iter());

        // DV
        digitable_line[32] = barcode[4];

        // Campo 4
        digitable_line[33..47].copy_from_slice(&barcode[5..19]);

        Ok(digitable_line)
    }

    fn calculate_digito_verificador(barcode: &[u8; 44]) -> Result<u8, BoletoError> {
        let _ = Self::validate(barcode)?;

        // Cria um iterator que itera sobre os caracteres do código de barras
        // exceto o dígito verificador
        let iterator_without_dv = barcode[..4]
            .iter()
            .chain(barcode[5..].iter());

        Ok(
            dv_utils::mod_11(iterator_without_dv).unwrap_or(b'1')
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{Cobranca, CodigoMoeda};

    #[test]
    fn get_cod_banco_correctly() {
        let barcodes: [(&[u8], u16); 5] = [
            (b"11191444455555555556666666666666666666666666", 111),
            (b"99996444455555555556666666666666666666666666", 999),
            (b"12395444455555555556666666666666666666666666", 123),
            (b"66691444455555555556666666666666666666666666", 666),
            (b"00091444455555555556666666666666666666666666", 0),
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
        match Cobranca::new(b"11191444455555555556666666666666666666666666") {
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

        match Cobranca::new(b"11105444455555555556666666666666666666666666") {
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
            (b"11196000055555555556666666666666666666666666", 0_u16, None),
            (
                b"11199100055555555556666666666666666666666666",
                1000,
                Some(NaiveDate::from_ymd(2000, 7, 3)),
            ),
            (
                b"11191100255555555556666666666666666666666666",
                1002,
                Some(NaiveDate::from_ymd(2000, 7, 5)),
            ),
            (
                b"11196166755555555556666666666666666666666666",
                1667,
                Some(NaiveDate::from_ymd(2002, 5, 1)),
            ),
            (
                b"11198478955555555556666666666666666666666666",
                4789,
                Some(NaiveDate::from_ymd(2010, 11, 17)),
            ),
            (
                b"11193999955555555556666666666666666666666666",
                9999,
                Some(NaiveDate::from_ymd(2025, 2, 21)),
            ),
            (
                b"75696903800002500001434301033723400014933001",
                9038,
                Some(NaiveDate::from_ymd(2022, 7, 6)),
            ),
            (
                b"00191667900002434790000002656973019362470618",
                6679,
                Some(NaiveDate::from_ymd(2016, 1, 20)),
            ),
            (
                b"00195586200000773520000002464206011816073018",
                5862,
                Some(NaiveDate::from_ymd(2013, 10, 25)),
            ),
            (
                b"75592896700003787000003389850761252543475984",
                8967,
                Some(NaiveDate::from_ymd(2022, 4, 26)),
            ),
            (
                b"23791672000003249052028269705944177105205220",
                6720,
                Some(NaiveDate::from_ymd(2016, 3, 1)),
            ),
            (
                b"23791672000003097902028060007024617500249000",
                6720,
                Some(NaiveDate::from_ymd(2016, 3, 1)),
            ),
        ];
        for (barcode, expected_fator, expected_date) in barcodes.iter() {
            match Cobranca::new(barcode.as_slice()) {
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
            match Cobranca::new(barcode.as_slice()) {
                Err(e) => {
                    panic!("Barcode should be considered valid. ({:?}): {:?}", e, barcode);
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
            (b"11191444455555555556666666666666666666666666", b'1'),
            (b"10499898100000214032006561000100040099726390", b'9'),
            (b"75696903800002500001434301033723400014933001", b'6'),
            (b"00191667900002434790000002656973019362470618", b'1'),
            (b"00195586200000773520000002464206011816073018", b'5'),
            (b"75592896700003787000003389850761252543475984", b'2'),
            (b"23791672000003249052028269705944177105205220", b'1'),
            (b"23791672000003097902028060007024617500249000", b'1'),
            (b"11191100255555555556666666666666666666666666", b'1'),
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
                b"75691434360103372340200149330011690380000250000",
                b"75696903800002500001434301033723400014933001",
            ),
            (
                b"00190000090265697301993624706185166790000243479",
                b"00191667900002434790000002656973019362470618",
            ),
            (
                b"00190000090246420601618160730182558620000077352",
                b"00195586200000773520000002464206011816073018",
            ),
            (
                b"75590003318985076125825434759848289670000378700",
                b"75592896700003787000003389850761252543475984",
            ),
            (
                b"23792028296970594417671052052207167200000324905",
                b"23791672000003249052028269705944177105205220",
            ),
            (
                b"23792028036000702461975002490003167200000309790",
                b"23791672000003097902028060007024617500249000",
            ),
        ];

        for (linha_digitavel, barcode) in barcodes.iter() {
            assert_eq!(
                Cobranca::cod_barras_to_linha_digitavel(barcode).unwrap(),
                **linha_digitavel,
            );
        }
    }

    #[test]
    fn validate_converting_linha_digitavel_to_barcode() {
        let barcodes = [
            (
                b"75691434360103372340200149330011690380000250000",
                b"75696903800002500001434301033723400014933001",
            ),
            (
                b"00190000090265697301993624706185166790000243479",
                b"00191667900002434790000002656973019362470618",
            ),
            (
                b"00190000090246420601618160730182558620000077352",
                b"00195586200000773520000002464206011816073018",
            ),
            (
                b"75590003318985076125825434759848289670000378700",
                b"75592896700003787000003389850761252543475984",
            ),
            (
                b"23792028296970594417671052052207167200000324905",
                b"23791672000003249052028269705944177105205220",
            ),
            (
                b"23792028036000702461975002490003167200000309790",
                b"23791672000003097902028060007024617500249000",
            ),
        ];

        for (linha_digitavel, barcode) in barcodes.iter() {
            assert_eq!(
                Cobranca::linha_digitavel_to_cod_barras(linha_digitavel).unwrap(),
                **barcode,
            );
        }
    }
}
