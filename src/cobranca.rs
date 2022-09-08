use std::ops::Range;

use crate::utils::fator_vencimento_to_date;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum BarcodeError {
    InvalidCodigoMoeda,
}

#[derive(Debug)]
pub enum CodigoMoeda {
    Real,
    Outras,
}

#[derive(Debug)]
pub struct BarcodeCobranca {
    pub cod_banco: u16,
    pub cod_moeda: CodigoMoeda,
    pub digito_verificador: u8,
    pub fator_vencimento: u16,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: f64,
}

impl BarcodeCobranca {
    pub fn new(barcode: &str) -> Result<Self, BarcodeError> {
        const ID_BANCO: Range<usize> = 0..3;
        const COD_MOEDA: Range<usize> = 3..4;
        const DIG_VERIF: Range<usize> = 4..5;
        const FATOR_VENC: Range<usize> = 5..9;
        const VALOR: Range<usize> = 9..19;
        // const CAMPO_LIVRE: Range<usize> = 19..44;

        let cod_moeda = match barcode[COD_MOEDA].parse().unwrap() {
            9 => CodigoMoeda::Real,
            0 => CodigoMoeda::Outras,
            _ => return Err(BarcodeError::InvalidCodigoMoeda),
        };

        let fator_vencimento: u16 = barcode[FATOR_VENC].parse().unwrap();

        Ok(Self {
            cod_moeda,
            fator_vencimento: fator_vencimento,
            cod_banco: barcode[ID_BANCO].parse().unwrap(),
            digito_verificador: barcode[DIG_VERIF].parse().unwrap(),
            data_vencimento: fator_vencimento_to_date(fator_vencimento),
            valor: (barcode[VALOR].parse::<f64>().unwrap()) / 100.00,
        })
    }
}
