use std::convert::TryFrom;
use std::str;

use crate::utils::dv_utils;
use crate::BoletoError;
// use crate::instituicoes_bancarias::InfoBanco;
// use chrono::NaiveDate;


#[derive(Debug)]
pub enum Segmento {
    Prefeituras,
    Saneamento,
    EnergiaEletricaEGas,
    Telecomunicacoes,
    OrgaosGovernamentais,
    Carnes,
    MultasTransito,
    ExclusivoDoBanco,
}

impl TryFrom<u8> for Segmento {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'1' => Ok(Self::Prefeituras),
            b'2' => Ok(Self::Saneamento),
            b'3' => Ok(Self::EnergiaEletricaEGas),
            b'4' => Ok(Self::Telecomunicacoes),
            b'5' => Ok(Self::OrgaosGovernamentais),
            b'6' => Ok(Self::Carnes),
            b'7' => Ok(Self::MultasTransito),
            b'9' => Ok(Self::ExclusivoDoBanco),
            _ => Err(()),
        }
    }
}


#[derive(Debug)]
pub enum TipoValor {
    ValorReaisMod10,
    QtdeMoedaMod10,
    ValorReaisMod11,
    QtdeMoedaMod11,
}

impl TryFrom<u8> for TipoValor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'6' => Ok(Self::ValorReaisMod10),
            b'7' => Ok(Self::QtdeMoedaMod10),
            b'8' => Ok(Self::ValorReaisMod11),
            b'9' => Ok(Self::QtdeMoedaMod11),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Convenio {
    Carne([u8; 8]),
    Outros([u8; 4]),
}

#[derive(Debug)]
pub struct Arrecadacao {
    pub cod_barras: [u8; 44],
    pub linha_digitavel: [u8; 48],
    pub segmento: Segmento,
    pub tipo_valor: TipoValor,
    pub digito_verificador: u8,
    pub valor: Option<f64>,
    pub convenio: Convenio,
}

impl Arrecadacao {
    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        let _ = Self::validate(value)?;

        let (cod_barras, linha_digitavel) = match value.len() {
            44 => {
                let mut barcode = [0_u8; 44];
                barcode.copy_from_slice(value);

                (barcode, Self::cod_barras_to_linha_digitavel(&barcode)?)
            },
            47 => {
                let mut digitable_line = [0_u8; 48];
                digitable_line.copy_from_slice(value);

                (Self::linha_digitavel_to_cod_barras(&digitable_line)?, digitable_line)
            },
            _ => return Err(BoletoError::InvalidLength),
        };

        let tipo_valor = TipoValor::try_from(cod_barras[2])
            .map_err(|_| BoletoError::InvalidTipoValor)?;

        let digito_verificador = Self::calculate_digito_verificador(&cod_barras, &tipo_valor)?;

        if digito_verificador != cod_barras[3] {
            return Err(BoletoError::InvalidDigitoVerificador);
        }

        let segmento: Segmento = Segmento::try_from(cod_barras[1])
            .map_err(|_| BoletoError::InvalidSegmento)?;

        let convenio = match segmento {
            Segmento::Carnes => {
                let mut cnpj = [0_u8; 8];
                cnpj.copy_from_slice(&cod_barras[15..23]);
                Convenio::Carne(cnpj)
            },
            _ => {
                let mut conv = [0_u8; 4];
                conv.copy_from_slice(&cod_barras[15..19]);
                Convenio::Outros(conv)
            },
        };

        Ok(Self {
            cod_barras,
            linha_digitavel,
            valor: Self::get_valor(&cod_barras, &tipo_valor),
            segmento,
            tipo_valor,
            digito_verificador,
            convenio,
        })
    }

    pub fn validate(barcode: &[u8]) -> Result<(), BoletoError> {
        let only_numbers = barcode.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        if barcode.first() != Some(&b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        Ok(())
    }

    pub fn linha_digitavel_to_cod_barras(digitable_line: &[u8; 48]) -> Result<[u8; 44], BoletoError> {
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

    pub fn cod_barras_to_linha_digitavel(barcode: &[u8; 44]) -> Result<[u8; 48], BoletoError> {
        let _ = Self::validate(barcode)?;


        let mut digitable_line = [0_u8; 48];
        // Arrecadação
        // 00000000001   11111111122   22222222333   33333334444
        // 01234567890   12345678901   23456789012   34567890123
        // ABCDEEEEEEE   EEEEFFFFGGG   GGGGGGGGGGG   GGGGGGGGGGG

        // 00000000001-1 11111111222-2 22222233333-3 33334444444-4
        // 01234567890-1 23456789012-3 45678901234-5 67890123456-7
        // ABCDEEEEEEE-W EEEEFFFFGGG-X GGGGGGGGGGG-Y GGGGGGGGGGG-Z

        // Carnês
        // 00000000001   11111111122   22222222333   33333334444
        // 01234567890   12345678901   23456789012   34567890123
        // ABCDEEEEEEE   EEEEFFFFFFF   FGGGGGGGGGG   GGGGGGGGGGG

        // 00000000001-1 11111111222-2 22222233333-3 33334444444-4
        // 01234567890-1 23456789012-3 45678901234-5 67890123456-7
        // ABCDEEEEEEE-W EEEEFFFFFFF-X FGGGGGGGGGG-Y GGGGGGGGGGG-Z


        // Campo 1
        digitable_line[0..11].copy_from_slice(&barcode[0..11]);
        digitable_line[11] = dv_utils::mod_10(digitable_line[0..11].iter());

        // Campo 2
        digitable_line[12..23].copy_from_slice(&barcode[11..22]);
        digitable_line[23] = dv_utils::mod_10(digitable_line[12..23].iter());

        // Campo 3
        digitable_line[24..35].copy_from_slice(&barcode[22..33]);
        digitable_line[35] = dv_utils::mod_10(digitable_line[24..35].iter());

        // Campo 4
        digitable_line[36..47].copy_from_slice(&barcode[33..44]);
        digitable_line[47] = dv_utils::mod_10(digitable_line[36..48].iter());

        Ok(digitable_line)
    }

    fn calculate_digito_verificador(barcode: &[u8; 44], tipo: &TipoValor) -> Result<u8, BoletoError> {
        let _ = Self::validate(barcode)?;

        // Cria um iterator que itera sobre os caracteres do código de barras
        // exceto o dígito verificador
        let iterator_without_dv = barcode[..4]
            .iter()
            .chain(barcode[5..].iter());

        match tipo {
            TipoValor::QtdeMoedaMod10 | TipoValor::ValorReaisMod10 => Ok(
                dv_utils::mod_10(iterator_without_dv)
            ),
            _ => Ok(dv_utils::mod_11(iterator_without_dv).unwrap_or(b'0'))
        }
    }

    fn get_valor(barcode: &[u8; 44], tipo: &TipoValor) -> Option<f64> {
        match tipo {
            TipoValor::ValorReaisMod10 | TipoValor::ValorReaisMod11 => {
                let x = unsafe { str::from_utf8_unchecked(&barcode[4..15]) };
                match  x.parse::<f64>().unwrap()
                {
                    x if x.is_normal() => Some(x / 100.00),
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

mod tests {
    use super::Arrecadacao;

    #[test]
    fn geral() {
        let a = Arrecadacao::new(b"83670000001283401110000010102022342248028780").unwrap();
        println!("{:?}", a);
        assert!(false);
    }
}