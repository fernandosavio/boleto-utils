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

use std::convert::TryFrom;
use std::ops::Range;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::fator_vencimento;
use crate::utils::barcode_utils;

#[derive(Debug)]
pub enum Error {
    NumbersOnly,
    InvalidLength(usize),
    InvalidCodigoMoeda,
}

#[derive(Debug)]
enum CodigoMoeda {
    Real = 9,
    Outras = 0,
}

// struct CampoLivreBradesco {
//     agencia_beneficiaria: u32,
//     carteira: u8,
//     nosso_numero: u32,
// }

#[derive(Debug)]
pub struct Arrecadacao {
    segmento: Segmento,
    convenio: String,
}

#[derive(Debug)]
pub struct Cobranca {
    id_banco: u16,
}

#[derive(Debug)]
pub enum Segmento {
    Prefeituras = 1,
    Saneamento = 2,
    EnergiaEletricaEGas = 3,
    Telecomunicacoes = 4,
    OrgaosGovernamentais = 5,
    Carnes = 6,
    MultasTransito = 7,
    ExclusivoDoBanco = 9,
}

#[derive(Debug)]
pub enum TipoBoleto {
    Arrecadacao(Arrecadacao),
    Cobranca(Cobranca),
}


#[derive(Debug)]
pub struct Boleto {
    tipo: TipoBoleto,
    codigo_barras: String,
    linha_digitavel: String,
    valor: f64,
    data_vencimento: Option<NaiveDate>,
}

impl Boleto {
    pub fn new(value: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref ONLY_NUMBERS_REGEX: Regex = Regex::new(r"^\d+$").unwrap();
        }

        if !ONLY_NUMBERS_REGEX.is_match(&value) {
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

        // match barcode.chars().next() {
        //     Some('8') => Self::parse_arrecadacao_barcode(barcode),
        //     _ => Self::parse_cobranca_barcode(barcode),
        // }
        
        Ok(
            Boleto {
                codigo_barras: barcode,
                linha_digitavel: digitable_line,
                data_vencimento: todo!(),
                tipo: todo!(),
                valor: todo!(),
            }
        )
    }

    fn parse_cobranca_barcode(value: String) -> Result<Self, Error> {
        const ID_BANCO: Range<usize> = 0..3;
        const COD_MOEDA: Range<usize> = 3..4;
        // const DIG_VERIF: Range<usize> = 4..5;
        const FATOR_VENC: Range<usize> = 5..9;
        const VALOR: Range<usize> = 9..19;
        // const CAMPO_LIVRE: Range<usize> = 19..44;

        match value[COD_MOEDA].parse().unwrap() {
            0 | 9 => {},
            _ => return Err(Error::InvalidCodigoMoeda),
        };

        let id_banco: u16 = value[ID_BANCO].parse().unwrap();
        // let digito_verificador: u8 = value[DIG_VERIF].parse().unwrap();
        let fator_vencimento: u32 = value[FATOR_VENC].parse().unwrap();
        let valor: f64 = value[VALOR].parse().unwrap();
        let valor = valor / 100.0;

        let cobranca = Cobranca { id_banco };

        Ok(Boleto {
            tipo: TipoBoleto::Cobranca(cobranca),
            data_vencimento: Some(fator_vencimento::to_date(fator_vencimento)),
            valor: valor,
            codigo_barras: value.clone(),
            linha_digitavel: value,
        })
    }

    fn parse_arrecadacao_barcode(value: String) -> Result<Self, Error> {
        todo!()
    }

}

impl TryFrom<&str> for Boleto {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{Boleto, Cobranca, TipoBoleto};

    #[test]
    fn valid_barcode() {
        let barcode = "10499898100000214032006561000100040099726390";

        let boleto = Boleto::new(barcode).unwrap();

        assert!(
            matches!(
                boleto.tipo, 
                TipoBoleto::Cobranca(Cobranca { id_banco: 104 })
            )
        );
        assert_eq!(
            boleto.data_vencimento, 
            Some(NaiveDate::from_ymd(2022, 5, 10))
        );
        assert_eq!(boleto.valor, 214.03);

        println!("{:?}", boleto);
    }
}