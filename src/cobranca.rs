use crate::utils::{dv_utils, fator_vencimento_to_date, u8_array_to_u16};
use crate::BoletoError;
use crate::instituicoes_bancarias::InfoBanco;
use chrono::NaiveDate;

pub struct CodBarras([u8; Cobranca::COD_BARRAS_LENGTH]);

impl CodBarras {
    pub fn new(input: &[u8]) -> Result<Self, BoletoError> {
        if input.first() == Some(&b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        if input.len() != Cobranca::COD_BARRAS_LENGTH {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = input.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        let mut cod_barras = [0u8; Cobranca::COD_BARRAS_LENGTH];
        cod_barras.copy_from_slice(input);

        Ok(Self(cod_barras))
    }

    pub fn calculate_digito_verificador(&self) -> Result<u8, BoletoError> {
        // Cria um iterator que itera sobre os caracteres do código de barras
        // exceto o dígito verificador
        let iterator_without_dv = self[..4]
            .iter()
            .chain(self[5..].iter());

        Ok(
            dv_utils::mod_11(iterator_without_dv).unwrap_or(b'1')
        )
    }
}

impl From<&LinhaDigitavel> for CodBarras {
    fn from(linha_digitavel: &LinhaDigitavel) -> Self {

        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV

        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        let LinhaDigitavel(src) = linha_digitavel;
        let mut barcode = [0_u8; Cobranca::COD_BARRAS_LENGTH];

        barcode[0..4].copy_from_slice(&src[0..4]);
        barcode[4..19].copy_from_slice(&src[32..47]);
        barcode[19..24].copy_from_slice(&src[4..9]);
        barcode[24..34].copy_from_slice(&src[10..20]);
        barcode[34..44].copy_from_slice(&src[21..31]);

        Self(barcode)
    }
}

impl std::fmt::Debug for CodBarras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CodBarras")
            .field(unsafe { &std::str::from_utf8_unchecked(&self.0)})
            .finish()
    }
}

impl std::ops::Deref for CodBarras {
    type Target = [u8; Cobranca::COD_BARRAS_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct LinhaDigitavel([u8; Cobranca::LINHA_DIGITAVEL_LENGTH]);

impl LinhaDigitavel {
    pub fn new(input: &[u8]) -> Result<Self, BoletoError> {
        if input.first() == Some(&b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        if input.len() != Cobranca::LINHA_DIGITAVEL_LENGTH {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = input.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        let mut linha_digitavel = [0u8; Cobranca::LINHA_DIGITAVEL_LENGTH];
        linha_digitavel.copy_from_slice(input);

        Ok(Self(linha_digitavel))
    }
}

impl From<&CodBarras> for LinhaDigitavel {
    fn from(cod_barras: &CodBarras) -> Self {
        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV
        // 75691.43436 01033.723402 00149.330011 6 90380000250000

        let CodBarras(src) = cod_barras;

        let mut digitable_line = [0_u8; Cobranca::LINHA_DIGITAVEL_LENGTH];

        // Campo 1
        digitable_line[0..4].copy_from_slice(&src[0..4]);
        digitable_line[4..9].copy_from_slice(&src[19..24]);
        digitable_line[9] = dv_utils::mod_10(digitable_line[0..9].iter());

        // Campo 2
        digitable_line[10..20].copy_from_slice(&src[24..34]);
        digitable_line[20] = dv_utils::mod_10(digitable_line[10..20].iter());

        // Campo 3
        digitable_line[21..31].copy_from_slice(&src[34..44]);
        digitable_line[31] = dv_utils::mod_10(digitable_line[21..31].iter());

        // DV
        digitable_line[32] = src[4];

        // Campo 4
        digitable_line[33..47].copy_from_slice(&src[5..19]);

        Self(digitable_line)
    }
}

impl std::fmt::Debug for LinhaDigitavel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LinhaDigitavel")
            .field(unsafe { &std::str::from_utf8_unchecked(&self.0)})
            .finish()
    }
}

impl std::ops::Deref for LinhaDigitavel {
    type Target = [u8; Cobranca::LINHA_DIGITAVEL_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug)]
pub enum CodigoMoeda {
    Real,
    Outras,
}
#[derive(Debug)]
pub struct Cobranca {
    pub cod_barras: CodBarras,
    pub linha_digitavel: LinhaDigitavel,
    pub cod_banco: u16,
    pub info_banco: Option<&'static InfoBanco>,
    pub cod_moeda: CodigoMoeda,
    pub digito_verificador: u8,
    pub fator_vencimento: u16,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: Option<f64>,
}


impl Cobranca {
    const COD_BARRAS_LENGTH: usize = 44;
    const LINHA_DIGITAVEL_LENGTH: usize = 47;

    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        let (cod_barras, linha_digitavel): (CodBarras, LinhaDigitavel) = match value.len() {
            Self::COD_BARRAS_LENGTH => {
                let cod_barras = CodBarras::new(value)?;
                let linha_digitavel = LinhaDigitavel::from(&cod_barras);
                (cod_barras, linha_digitavel)
            },
            Self::LINHA_DIGITAVEL_LENGTH => {
                let linha_digitavel = LinhaDigitavel::new(value)?;
                ((&linha_digitavel).into(), linha_digitavel)
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

        if fator_vencimento > 0 && fator_vencimento < 1000 {
            return Err(BoletoError::InvalidCodigoMoeda)
        }

        let valor = {
            let x = unsafe { std::str::from_utf8_unchecked(&cod_barras[9..19]) };
            match  x.parse::<f64>().unwrap()
            {
                x if x.is_normal() => Some(x / 100.00),
                _ => None,
            }
        };

        let digito_verificador: u8 = cod_barras[4];

        if digito_verificador != cod_barras.calculate_digito_verificador()? {
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
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::cobranca::LinhaDigitavel;

    use super::{Cobranca, CodigoMoeda, CodBarras};

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
                Some(NaiveDate::from_ymd(2025, 2, 22)),
            ),
            (
                b"11191100255555555556666666666666666666666666",
                1002,
                Some(NaiveDate::from_ymd(2025, 2, 24)),
            ),
            (
                b"11196166755555555556666666666666666666666666",
                1667,
                Some(NaiveDate::from_ymd(2026, 12, 21)),
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
                CodBarras::new(*barcode).unwrap().calculate_digito_verificador().unwrap(),
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
                LinhaDigitavel::from(&CodBarras::new(*barcode).unwrap()).0,
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
                CodBarras::from(&LinhaDigitavel::new(*linha_digitavel).unwrap()).0,
                **barcode,
            );
        }
    }
}
