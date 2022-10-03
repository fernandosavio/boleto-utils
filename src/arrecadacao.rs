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
        // 00000 00000 11111 111112 22222 222233 3 33333334444444
        // 01234.56789 01234.567890 12345.678901 2 34567890123456
        // AAABC.CCCCX DDDDD.DDDDDY EEEEE.EEEEEZ K UUUUVVVVVVVVVV

        // 00000000001111111111222222222233333333334444
        // 01234567890123456789012345678901234567890123
        // AAABKUUUUVVVVVVVVVVCCCCCDDDDDDDDDDEEEEEEEEEE

        let LinhaDigitavel(src) = linha_digitavel;

        let mut barcode = [0_u8; 44];

        barcode[0..4].copy_from_slice(&src[0..4]);
        barcode[4..19].copy_from_slice(&src[32..47]);
        barcode[19..24].copy_from_slice(&src[4..9]);
        barcode[24..34].copy_from_slice(&src[10..20]);
        barcode[34..44].copy_from_slice(&src[21..31]);

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

impl From<&CodBarras> for LinhaDigitavel {
    fn from(cod_barras: &CodBarras) -> Self {
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

        // Campo 1
        digitable_line[0..11].copy_from_slice(&src[0..11]);
        digitable_line[11] = dv_utils::mod_10(digitable_line[0..11].iter());

        // Campo 2
        digitable_line[12..23].copy_from_slice(&src[11..22]);
        digitable_line[23] = dv_utils::mod_10(digitable_line[12..23].iter());

        // Campo 3
        digitable_line[24..35].copy_from_slice(&src[22..33]);
        digitable_line[35] = dv_utils::mod_10(digitable_line[24..35].iter());

        // Campo 4
        digitable_line[36..47].copy_from_slice(&src[33..44]);
        digitable_line[47] = dv_utils::mod_10(digitable_line[36..47].iter());

        Self(digitable_line)
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
    const COD_BARRAS_LENGTH: usize = 44;
    const LINHA_DIGITAVEL_LENGTH: usize = 48;

    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        let (cod_barras, linha_digitavel): (CodBarras, LinhaDigitavel) = match value.len() {
            Self::COD_BARRAS_LENGTH => {
                let cod_barras = CodBarras::new(value)?;
                let linha_digitavel = LinhaDigitavel::from(&cod_barras);
                (cod_barras, linha_digitavel)
            }
            Self::LINHA_DIGITAVEL_LENGTH => {
                let linha_digitavel = LinhaDigitavel::new(value)?;
                ((&linha_digitavel).into(), linha_digitavel)
            }
            _ => return Err(BoletoError::InvalidLength),
        };

        let tipo_valor = cod_barras.tipo_valor()?;

        let digito_verificador = {
            let dv = cod_barras.calculate_digito_verificador()?;
            if dv != cod_barras[3] {
                return Err(BoletoError::InvalidDigitoVerificador);
            }
            dv - b'0'
        };

        let segmento: Segmento = cod_barras.segmento()?;

        let convenio = match segmento {
            Segmento::Carnes => Convenio::Carne,
            _ => Convenio::Outros(InfoConvenio::get(
                &segmento,
                utils::u8_array_to_u16(&cod_barras[15..19]),
            )),
        };

        Ok(Self {
            valor: Self::valor(&cod_barras, &tipo_valor),
            cod_barras,
            linha_digitavel,
            segmento,
            tipo_valor,
            digito_verificador,
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

mod tests {}
