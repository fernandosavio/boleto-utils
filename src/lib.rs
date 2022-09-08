/*
111 2 3 4444 5555555555 6666666666666666666666666
└┬┘ ┬ ┬ └┬─┘ └───┬────┘ └─────────┬─────────────┘
 │  │ │  │       │                └── `campo_livre`: Campo livre para uso dos bancos;
 │  │ │  │       └─────────────────── `valor`: Valor do boleto
 │  │ │  └─────────────────────────── `fator_vencimento`: Fator de vencimento
 │  │ └────────────────────────────── `digito_verificador`: Dígito verificador
 │  └──────────────────────────────── `codigo_moeda`: Código da Moeda (Real=9 e Outros=0)
 └─────────────────────────────────── `id_banco`: identificação do banco
*/

mod utils;
mod cobranca;

use std::convert::TryFrom;
use chrono::NaiveDate;
use crate::utils::barcode_utils;
use crate::cobranca::BarcodeCobranca;


#[derive(Debug)]
pub enum Error {
    NumbersOnly,
    InvalidLength(usize),
    InvalidCodigoMoeda,
    InvalidDigitableLine(String),
}

// struct CampoLivreBradesco {
//     agencia_beneficiaria: u32,
//     carteira: u8,
//     nosso_numero: u32,
// }

// #[derive(Debug)]
// pub struct Arrecadacao {
//     segmento: Segmento,
//     convenio: String,
// }

// #[derive(Debug)]
// pub enum Segmento {
//     Prefeituras = 1,
//     Saneamento = 2,
//     EnergiaEletricaEGas = 3,
//     Telecomunicacoes = 4,
//     OrgaosGovernamentais = 5,
//     Carnes = 6,
//     MultasTransito = 7,
//     ExclusivoDoBanco = 9,
// }

#[derive(Debug)]
pub enum TipoBoleto {
    // Arrecadacao(Arrecadacao),
    Cobranca(BarcodeCobranca),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Boleto {
    tipo: TipoBoleto,
    codigo_barras: String,
    linha_digitavel: String,
    valor: f64,
    data_vencimento: Option<NaiveDate>,
}

impl TryFrom<&str> for Boleto {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Boleto {
    pub fn new(value: &str) -> Result<Self, Error> {

        let only_numbers = value.chars().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(Error::NumbersOnly);
        }

        let (barcode, digitable_line): (String, String) = match value.len() {
            44 => (
                String::from(value),
                barcode_utils::barcode_to_digitable_line(value),
            ),
            46..=48 => (
                barcode_utils::digitable_line_to_barcode(value),
                String::from(value),
            ),
            length => return Err(Error::InvalidLength(length)),
        };

        match barcode.chars().next() {
            None => unreachable!(),
            Some('8') => todo!(),
            _ => {
                let cob = BarcodeCobranca::new(&barcode).unwrap();

                Ok(
                    Boleto {
                        codigo_barras: barcode,
                        linha_digitavel: digitable_line,
                        data_vencimento: cob.data_vencimento,
                        valor: cob.valor,
                        tipo: TipoBoleto::Cobranca(cob),
                    }
                )
            },
        }
    }


}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{Boleto, TipoBoleto};

    #[test]
    fn valid_barcode() {
        let barcode = "10499898100000214032006561000100040099726390";

        let boleto = Boleto::new(barcode).unwrap();

        assert!(
            matches!(boleto.tipo, TipoBoleto::Cobranca(_))
        );
        assert_eq!(
            boleto.data_vencimento,
            Some(NaiveDate::from_ymd(2022, 5, 10))
        );
        assert_eq!(boleto.valor, 214.03);

        println!("{:?}", boleto);
    }
}