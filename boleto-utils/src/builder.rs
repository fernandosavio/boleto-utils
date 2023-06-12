use crate::{cobranca::{Cobranca, CodBarras, LinhaDigitavel, CodigoMoeda}, instituicoes_bancarias::InfoBanco, utils::date_to_fator_vencimento};
use chrono::NaiveDate;

impl Cobranca {
    pub fn builder() -> CobrancaBuilder<NoCodBarras, NoLinhaDigitavel, NoCodBanco, NoCodMoeda> {
        CobrancaBuilder::new()
    }
}

pub struct NoCodBarras;
pub struct NoLinhaDigitavel;
pub struct NoCodMoeda;
pub struct CodBanco(u16);
pub struct NoCodBanco;



pub struct CobrancaBuilder<CD, LD, CB, CM> {
    pub cod_barras: CD,
    pub linha_digitavel: LD,
    pub cod_banco: CB,
    pub cod_moeda: CM,
    pub data_vencimento: Option<NaiveDate>,
    pub valor: Option<f64>,
}

impl CobrancaBuilder<NoCodBarras, NoLinhaDigitavel, NoCodBanco, NoCodMoeda> {
    pub fn new() -> CobrancaBuilder<NoCodBarras, NoLinhaDigitavel, NoCodBanco, NoCodMoeda> {
        Self {
            cod_barras: NoCodBarras,
            linha_digitavel: NoLinhaDigitavel,
            cod_banco: NoCodBanco,
            cod_moeda: NoCodMoeda,
            data_vencimento: None,
            valor: None,
        }
    }
}

impl<CB, CM> CobrancaBuilder<NoCodBarras, NoLinhaDigitavel, CB, CM> {
    pub fn cod_barras(self, cod_barras: CodBarras) -> CobrancaBuilder<CodBarras, LinhaDigitavel, CB, CM> {
        CobrancaBuilder {
            linha_digitavel: LinhaDigitavel::from(&cod_barras),
            cod_barras: cod_barras,
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }

    pub fn linha_digitavel(self, linha_digitavel: LinhaDigitavel) -> CobrancaBuilder<CodBarras, LinhaDigitavel, CB, CM> {
        CobrancaBuilder {
            cod_barras: CodBarras::from(&linha_digitavel),
            linha_digitavel: linha_digitavel,
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}

impl<CD, LD, CM> CobrancaBuilder<CD, LD, NoCodBanco, CM> {
    pub fn cod_banco(self, cod_banco: CodBanco) -> CobrancaBuilder<CD, LD, CodBanco, CM> {
        CobrancaBuilder {
            cod_barras: self.cod_barras,
            linha_digitavel: self.linha_digitavel,
            cod_banco: cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}

impl<CD, LD, CB> CobrancaBuilder<CD, LD, CB, NoCodMoeda> {
    pub fn cod_moeda(self, cod_moeda: CodigoMoeda) -> CobrancaBuilder<CD, LD, CB, CodigoMoeda> {
        CobrancaBuilder {
            cod_barras: self.cod_barras,
            linha_digitavel: self.linha_digitavel,
            cod_banco: self.cod_banco,
            cod_moeda: cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}

impl<CD, LD, CB, CM> CobrancaBuilder<CD, LD, CB, CM> {
    pub fn data_vencimento(self, data_vencimento: NaiveDate) -> CobrancaBuilder<CD, LD, CB, CM> {
        CobrancaBuilder {
            cod_barras: self.cod_barras,
            linha_digitavel: self.linha_digitavel,
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: Some(data_vencimento),
            valor: self.valor,
        }
    }
}

impl<CD, LD, CB, CM> CobrancaBuilder<CD, LD, CB, CM> {
    pub fn valor(self, valor: f64) -> CobrancaBuilder<CD, LD, CB, CM> {
        CobrancaBuilder {
            cod_barras: self.cod_barras,
            linha_digitavel: self.linha_digitavel,
            cod_banco: self.cod_banco,
            cod_moeda: self.cod_moeda,
            data_vencimento: self.data_vencimento,
            valor: Some(valor),
        }
    }
}

impl CobrancaBuilder<CodBarras, LinhaDigitavel, CodBanco, CodigoMoeda> {
    pub fn build(self) -> Cobranca {
        let fator_vencimento = if let Some(data_vencimento) = self.data_vencimento {
            date_to_fator_vencimento(data_vencimento).unwrap_or(0u16)
        } else {
            0u16
        };

        Cobranca {
            digito_verificador: self.cod_barras.calculate_dv(),
            cod_barras: self.cod_barras,
            linha_digitavel: self.linha_digitavel,
            cod_banco: self.cod_banco.0,
            info_banco: InfoBanco::get_by_id(self.cod_banco.0),
            cod_moeda: self.cod_moeda,
            fator_vencimento: fator_vencimento,
            data_vencimento: self.data_vencimento,
            valor: self.valor,
        }
    }
}
