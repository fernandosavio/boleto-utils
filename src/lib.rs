mod utils;
pub mod cobranca;
pub mod arrecadacao;
mod instituicoes_bancarias;
mod concessionarias;

use serde::Serialize;

use thiserror::Error;

use arrecadacao::CodBarras as CodBarrasArr;
use cobranca::CodBarras as CodBarrasCob;

use crate::cobranca::Cobranca;
use crate::arrecadacao::Arrecadacao;


#[derive(Error, Debug)]
pub enum BoletoError {
    #[error("deve conter apenas números")]
    NumbersOnly,
    #[error("tamanho inválido")]
    InvalidLength,
    #[error("código moeda inválido")]
    InvalidCodigoMoeda,
    #[error("dígito verificador inválido")]
    InvalidDigitoVerificador,
    #[error("código de barras de cobrança inválido")]
    InvalidCobrancaBarcode,
    #[error("fator de vencimento inválido")]
    InvalidFatorVencimento,
    #[error("código de barras de arrecadação inválido")]
    InvalidArrecadacaoBarcode,
    #[error("segmento inválido")]
    InvalidSegmento,
    #[error("tipo de valor inválido")]
    InvalidTipoValor,
}


#[derive(Debug, Serialize)]
#[serde(tag = "tipo", content = "dados")]
pub enum Boleto {
    #[serde(rename = "arrecadacao")]
    Arrecadacao(Arrecadacao),
    #[serde(rename = "cobranca")]
    Cobranca(Cobranca),
}

impl std::fmt::Display for Boleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Arrecadacao(dados) => format!("{dados}"),
            Self::Cobranca(dados) => format!("{dados}"),
        })
    }
}

impl Boleto {
    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        match value.first() {
            None => Err(BoletoError::InvalidLength),
            Some(b'8') => Ok(Boleto::Arrecadacao(Arrecadacao::new(value)?)),
            _ => Ok(Boleto::Cobranca(Cobranca::new(value)?)),
        }
    }

    pub fn calculate_digito_verificador(value: &[u8]) -> Result<u8, BoletoError> {
        match value.first() {
            None => Err(BoletoError::InvalidLength),
            Some(b'8') => {
                CodBarrasArr::new(value)?.calculate_digito_verificador()
            },
            _ => {
                CodBarrasCob::new(value)?.calculate_digito_verificador()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::Boleto;

    #[test]
    fn valid_barcode() {
        let barcode = b"10499898100000214032006561000100040099726390";

        let boleto = Boleto::new(barcode).unwrap();

        match boleto {
            Boleto::Cobranca(cob) => {
                assert_eq!(
                    cob.data_vencimento,
                    Some(NaiveDate::from_ymd(2022, 5, 10))
                );
                assert_eq!(cob.valor, Some(214.03));
            },
            _ => assert!(false, "Should be Cobranca"),
        }
    }
}