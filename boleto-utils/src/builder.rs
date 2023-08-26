use crate::{cobranca::{Cobranca, CodBarras, CodigoMoeda, CodBanco}, utils::date_to_fator_vencimento};
use chrono::NaiveDate;

impl Cobranca {
    pub fn builder() -> CobrancaBuilder<NoCodBanco, NoCodMoeda> {
        CobrancaBuilder::new()
    }
}

pub struct NoCodMoeda;
pub struct NoCodBanco;


pub struct CobrancaBuilder<CB, CM> {
    pub cod_banco: CB,
    pub cod_moeda: CM,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: Option<f64>,
}

impl CobrancaBuilder<NoCodBanco, NoCodMoeda> {
    pub fn new() -> CobrancaBuilder<NoCodBanco, NoCodMoeda> {
        Self {
            cod_banco: NoCodBanco,
            cod_moeda: NoCodMoeda,
            data_vencimento: None,
            valor: None,
        }
    }
}

impl<CM> CobrancaBuilder<NoCodBanco, CM> {
    pub fn cod_banco(self, cod_banco: CodBanco) -> CobrancaBuilder<CodBanco, CM> {
        CobrancaBuilder {
            cod_banco: cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}

impl<CB> CobrancaBuilder<CB, NoCodMoeda> {
    pub fn cod_moeda(self, cod_moeda: CodigoMoeda) -> CobrancaBuilder<CB, CodigoMoeda> {
        CobrancaBuilder {
            cod_banco: self.cod_banco,
            cod_moeda: cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}

impl<CB, CM> CobrancaBuilder<CB, CM> {
    pub fn data_vencimento(self, data_vencimento: NaiveDate) -> CobrancaBuilder<CB, CM> {
        CobrancaBuilder {
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: Some(data_vencimento),
            valor: self.valor,
        }
    }
}

impl<CB, CM> CobrancaBuilder<CB, CM> {
    pub fn valor(self, valor: f64) -> CobrancaBuilder<CB, CM> {
        CobrancaBuilder {
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: Some(valor),
        }
    }
}

impl CobrancaBuilder<CodBanco, CodigoMoeda> {
    pub fn build(self) -> Cobranca {
        let cobranca = {
            let mut result = [b'0'; 44];

            let fator_vencimento = if let Some(data_vencimento) = self.data_vencimento {
                date_to_fator_vencimento(data_vencimento).unwrap_or(0u16)
            } else {
                0u16
            };

            let valor = if let Some(valor) = self.valor {
                (valor * 100.0).trunc() as u64
            } else {
                0u64
            };

            result[0..3].copy_from_slice(format!("{:03}", self.cod_banco.0).as_ref());
            result[3] = self.cod_moeda.into();
            result[5..9].copy_from_slice(format!("{:04}", fator_vencimento).as_ref());
            result[9..19].copy_from_slice(format!("{:010}", valor).as_ref());

            let mut cobranca = CodBarras::new(&result).unwrap();
            cobranca.update_dv();

            cobranca
        };

        Cobranca::new(cobranca.as_bytes()).unwrap()
    }
}
