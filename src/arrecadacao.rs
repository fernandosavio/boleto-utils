use std::convert::{From, TryFrom};
use std::fmt;

use serde::Serialize;

use crate::concessionarias::InfoConvenio;
use crate::utils::{self, dv_utils};
use crate::BoletoError;

pub struct CodBarras([u8; Arrecadacao::COD_BARRAS_LENGTH]);

impl CodBarras {
    pub fn new(input: &[u8]) -> Result<Self, BoletoError> {
        if input.first() != Some(&b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        if input.len() != Arrecadacao::COD_BARRAS_LENGTH {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = input.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        TipoValor::try_from(input[2]).map_err(|_| BoletoError::InvalidTipoValor)?;

        let mut cod_barras = [0u8; Arrecadacao::COD_BARRAS_LENGTH];
        cod_barras.copy_from_slice(input);

        Ok(Self(cod_barras))
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    pub fn tipo_valor(&self) -> Result<TipoValor, BoletoError> {
        TipoValor::try_from(self[2]).map_err(|_| BoletoError::InvalidTipoValor)
    }

    pub fn segmento(&self) -> Result<Segmento, BoletoError> {
        Segmento::try_from(self[1]).map_err(|_| BoletoError::InvalidSegmento)
    }

    pub fn calculate_digito_verificador(&self) -> Result<u8, BoletoError> {
        // Cria um iterator que itera sobre os caracteres do código de barras
        // exceto o dígito verificador
        let iterator_without_dv = self[..3].iter().chain(self[4..].iter());

        Ok(match self.tipo_valor()? {
            TipoValor::QtdeMoedaMod10 | TipoValor::ValorReaisMod10 => {
                dv_utils::mod_10(iterator_without_dv)
            }
            _ => dv_utils::mod_11(iterator_without_dv).unwrap_or(b'0'),
        } - b'0')
    }
}

impl From<&LinhaDigitavel> for CodBarras {
    fn from(linha_digitavel: &LinhaDigitavel) -> Self {
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

        let LinhaDigitavel(src) = linha_digitavel;

        let mut barcode = [0_u8; 44];

        barcode[0..11].copy_from_slice(&src[0..11]);
        barcode[11..22].copy_from_slice(&src[12..23]);
        barcode[22..33].copy_from_slice(&src[24..35]);
        barcode[33..44].copy_from_slice(&src[36..47]);

        Self(barcode)
    }
}

impl fmt::Debug for CodBarras {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CodBarras")
            .field(&self.as_str())
            .finish()
    }
}

impl fmt::Display for CodBarras {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::ops::Deref for CodBarras {
    type Target = [u8; Arrecadacao::COD_BARRAS_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for CodBarras {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}

pub struct LinhaDigitavel([u8; Arrecadacao::LINHA_DIGITAVEL_LENGTH]);

impl LinhaDigitavel {
    pub fn new(input: &[u8]) -> Result<Self, BoletoError> {
        if input.first() != Some(&b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        if input.len() != Arrecadacao::LINHA_DIGITAVEL_LENGTH {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = input.iter().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        let mut linha_digitavel = [0u8; Arrecadacao::LINHA_DIGITAVEL_LENGTH];
        linha_digitavel.copy_from_slice(input);

        Ok(Self(linha_digitavel))
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl TryFrom<&CodBarras> for LinhaDigitavel {
    type Error = BoletoError;

    fn try_from(cod_barras: &CodBarras) -> Result<Self, Self::Error> {
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

        let CodBarras(src) = cod_barras;
        let mut digitable_line = [0_u8; 48];

        let is_mod_10 = match TipoValor::try_from(cod_barras[2]) {
            Ok(TipoValor::QtdeMoedaMod10 | TipoValor::ValorReaisMod10) => true,
            Err(_) => return Err(BoletoError::InvalidTipoValor),
            _ => false,
        };

        // Campo 1
        digitable_line[0..11].copy_from_slice(&src[0..11]);
        digitable_line[11] = if is_mod_10 {
            dv_utils::mod_10(digitable_line[0..11].iter())
        } else {
            dv_utils::mod_11(digitable_line[0..11].iter()).unwrap_or(b'0')
        };

        // Campo 2
        digitable_line[12..23].copy_from_slice(&src[11..22]);
        digitable_line[23] = if is_mod_10 {
            dv_utils::mod_10(digitable_line[12..23].iter())
        } else {
            dv_utils::mod_11(digitable_line[12..23].iter()).unwrap_or(b'0')
        };

        // Campo 3
        digitable_line[24..35].copy_from_slice(&src[22..33]);
        digitable_line[35] = if is_mod_10 {
            dv_utils::mod_10(digitable_line[24..35].iter())
        } else {
            dv_utils::mod_11(digitable_line[24..35].iter()).unwrap_or(b'0')
        };

        // Campo 4
        digitable_line[36..47].copy_from_slice(&src[33..44]);
        digitable_line[47] = if is_mod_10 {
            dv_utils::mod_10(digitable_line[36..47].iter())
        } else {
            dv_utils::mod_11(digitable_line[36..47].iter()).unwrap_or(b'0')
        };

        Ok(Self(digitable_line))
    }
}

impl fmt::Debug for LinhaDigitavel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LinhaDigitavel")
            .field(&self.as_str())
            .finish()
    }
}

impl fmt::Display for LinhaDigitavel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::ops::Deref for LinhaDigitavel {
    type Target = [u8; Arrecadacao::LINHA_DIGITAVEL_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for LinhaDigitavel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Serialize)]
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

impl fmt::Display for Segmento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Prefeituras => "Prefeituras",
            Self::Saneamento => "Saneamento",
            Self::EnergiaEletricaEGas => "Energia elétrica e gás",
            Self::Telecomunicacoes => "Telecomunicações",
            Self::OrgaosGovernamentais => "Órgãos governamentais",
            Self::Carnes => "Carnês",
            Self::MultasTransito => "Multas de Trânsito",
            Self::ExclusivoDoBanco => "Uso exclusivo do banco emissor",
        })
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub enum Convenio {
    Carne, // Ignorando cadastro de carnês por falta de dados
    Outros(Option<&'static InfoConvenio>),
}

#[derive(Debug, Serialize)]
pub struct Arrecadacao {
    pub cod_barras: CodBarras,
    pub linha_digitavel: LinhaDigitavel,
    pub segmento: Segmento,
    pub tipo_valor: TipoValor,
    #[serde(skip)]
    pub digito_verificador: u8,
    #[serde(skip)]
    pub digitos_verificadores_campos: (u8, u8, u8, u8),
    pub valor: Option<f64>,
    pub convenio: Convenio,
}

impl fmt::Display for Arrecadacao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "            Tipo: Arrecadação\n",
                "Código de barras: {}\n",
                " Linha digitável: {}\n",
                "        Segmento: {}\n",
                "           Valor: {}",
            ),
            self.cod_barras,
            self.linha_digitavel,
            self.segmento,
            match self.valor {
                Some(v) =>format!("{:.2}", v),
                None => "Sem valor informado".to_owned()
            },
        )
    }
}

impl Arrecadacao {
    pub const COD_BARRAS_LENGTH: usize = 44;
    pub const LINHA_DIGITAVEL_LENGTH: usize = 48;

    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        let (cod_barras, linha_digitavel): (CodBarras, LinhaDigitavel) = match value.len() {
            Self::COD_BARRAS_LENGTH => {
                let cod_barras = CodBarras::new(value)?;
                let linha_digitavel = LinhaDigitavel::try_from(&cod_barras)?;
                (cod_barras, linha_digitavel)
            }
            Self::LINHA_DIGITAVEL_LENGTH => {
                let linha_digitavel = LinhaDigitavel::new(value)?;
                ((&linha_digitavel).into(), linha_digitavel)
            }
            _ => return Err(BoletoError::InvalidLength),
        };

        let tipo_valor = cod_barras.tipo_valor()?;

        let segmento: Segmento = cod_barras.segmento()?;

        let convenio = match segmento {
            Segmento::Carnes => Convenio::Carne,
            _ => Convenio::Outros(InfoConvenio::get(
                &segmento,
                utils::u8_array_to_u16(&cod_barras[15..19]),
            )),
        };

        let digito_verificador = {
            let dv = cod_barras.calculate_digito_verificador()?;

            if dv != cod_barras[3] - b'0' {
                return Err(BoletoError::InvalidDigitoVerificador);
            }
            dv
        };

        let digitos_verificadores_campos = {
            let is_mod_10 = match TipoValor::try_from(cod_barras[2]) {
                Ok(TipoValor::QtdeMoedaMod10 | TipoValor::ValorReaisMod10) => true,
                Err(_) => return Err(BoletoError::InvalidTipoValor),
                _ => false,
            };

            let correct_dvs = if is_mod_10 {
                (
                    dv_utils::mod_10(linha_digitavel[0..11].iter()),
                    dv_utils::mod_10(linha_digitavel[12..23].iter()),
                    dv_utils::mod_10(linha_digitavel[24..35].iter()),
                    dv_utils::mod_10(linha_digitavel[36..47].iter()),
                )
            } else {
                (
                    dv_utils::mod_11(linha_digitavel[0..11].iter()).unwrap_or(b'0'),
                    dv_utils::mod_11(linha_digitavel[12..23].iter()).unwrap_or(b'0'),
                    dv_utils::mod_11(linha_digitavel[24..35].iter()).unwrap_or(b'0'),
                    dv_utils::mod_11(linha_digitavel[36..47].iter()).unwrap_or(b'0'),
                )
            };

            if linha_digitavel[11] != correct_dvs.0
                || linha_digitavel[23] != correct_dvs.1
                || linha_digitavel[35] != correct_dvs.2
                || linha_digitavel[47] != correct_dvs.3
            {
                return Err(BoletoError::InvalidDigitoVerificador);
            }

            correct_dvs
        };

        Ok(Self {
            valor: Self::valor(&cod_barras, &tipo_valor),
            cod_barras,
            linha_digitavel,
            segmento,
            tipo_valor,
            digito_verificador,
            digitos_verificadores_campos,
            convenio,
        })
    }

    fn valor(barcode: &CodBarras, tipo: &TipoValor) -> Option<f64> {
        match tipo {
            TipoValor::ValorReaisMod10 | TipoValor::ValorReaisMod11 => {
                let x = unsafe { std::str::from_utf8_unchecked(&barcode[4..15]) };
                match x.parse::<f64>().unwrap() {
                    x if x.is_normal() => Some(x / 100.00),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cod_segmento_correctly() {
        assert!(matches!(Arrecadacao::new(b"81675555555555566667777777777777777777777777").unwrap().segmento, Segmento::Prefeituras));
        assert!(matches!(Arrecadacao::new(b"82665555555555566667777777777777777777777777").unwrap().segmento, Segmento::Saneamento));
        assert!(matches!(Arrecadacao::new(b"83655555555555566667777777777777777777777777").unwrap().segmento, Segmento::EnergiaEletricaEGas));
        assert!(matches!(Arrecadacao::new(b"84645555555555566667777777777777777777777777").unwrap().segmento, Segmento::Telecomunicacoes));
        assert!(matches!(Arrecadacao::new(b"85635555555555566667777777777777777777777777").unwrap().segmento, Segmento::OrgaosGovernamentais));
        assert!(matches!(Arrecadacao::new(b"86625555555555566667777777777777777777777777").unwrap().segmento, Segmento::Carnes));
        assert!(matches!(Arrecadacao::new(b"87615555555555566667777777777777777777777777").unwrap().segmento, Segmento::MultasTransito));
        assert!(matches!(Arrecadacao::new(b"88605555555555566667777777777777777777777777"), Err(BoletoError::InvalidSegmento)));
        assert!(matches!(Arrecadacao::new(b"89695555555555566667777777777777777777777777").unwrap().segmento, Segmento::ExclusivoDoBanco));

        assert!(matches!(Arrecadacao::new(b"816755555553555566667773777777777775777777777775").unwrap().segmento, Segmento::Prefeituras));
        assert!(matches!(Arrecadacao::new(b"826655555553555566667773777777777775777777777775").unwrap().segmento, Segmento::Saneamento));
        assert!(matches!(Arrecadacao::new(b"836555555553555566667773777777777775777777777775").unwrap().segmento, Segmento::EnergiaEletricaEGas));
        assert!(matches!(Arrecadacao::new(b"846455555553555566667773777777777775777777777775").unwrap().segmento, Segmento::Telecomunicacoes));
        assert!(matches!(Arrecadacao::new(b"856355555553555566667773777777777775777777777775").unwrap().segmento, Segmento::OrgaosGovernamentais));
        assert!(matches!(Arrecadacao::new(b"866255555553555566667773777777777775777777777775").unwrap().segmento, Segmento::Carnes));
        assert!(matches!(Arrecadacao::new(b"876155555553555566667773777777777775777777777775").unwrap().segmento, Segmento::MultasTransito));
        assert!(matches!(Arrecadacao::new(b"886055555559555566667778777777777778777777777779"), Err(BoletoError::InvalidSegmento)));
        assert!(matches!(Arrecadacao::new(b"896955555553555566667773777777777775777777777775").unwrap().segmento, Segmento::ExclusivoDoBanco));

    }

    #[test]
    fn get_tipo_valor_correctly() {}

    #[test]
    fn get_valor_correctly() {}

    #[test]
    fn get_convenio_correctly() {}

    #[test]
    fn validate_digito_verificador_correctly() {
        let barcodes = [
            (b"83800000000570100310200140444030700008190320", 0_u8),
            (b"83680000002158200060000010120204236635162731", 8_u8),
            (b"84640000000959900820899988923054118633769199", 4_u8),
            (b"83650000000520801380013194136151108052494658", 5_u8),
        ];

        for (barcode, expected) in barcodes.iter() {
            assert_eq!(
                CodBarras::new(*barcode).unwrap().calculate_digito_verificador().unwrap(),
                *expected,
            );
        }
    }

    #[test]
    fn validate_converting_barcode_to_linha_digitavel() {}

    #[test]
    fn validate_converting_linha_digitavel_to_barcode() {}
}
