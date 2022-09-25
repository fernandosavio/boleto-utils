mod utils;
mod cobranca;
mod arrecadacao;
mod instituicoes_bancarias;

use crate::cobranca::Cobranca;
use crate::arrecadacao::Arrecadacao;


#[derive(Debug)]
pub enum BoletoError {
    NumbersOnly,
    InvalidLength,
    InvalidCodigoMoeda,
    InvalidDigitoVerificador,
    InvalidCobrancaBarcode,
    InvalidFatorVencimento,
    InvalidArrecadacaoBarcode,
    InvalidSegmento,
    InvalidTipoValor,
}


#[derive(Debug)]
pub enum Boleto {
    Arrecadacao(Arrecadacao),
    Cobranca(Cobranca),
}

impl Boleto {
    pub fn new(value: &[u8]) -> Result<Self, BoletoError> {
        match value.first() {
            None => Err(BoletoError::InvalidLength),
            Some(b'8') => Ok(Boleto::Arrecadacao(Arrecadacao::new(value)?)),
            _ => Ok(Boleto::Cobranca(Cobranca::new(value)?)),
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