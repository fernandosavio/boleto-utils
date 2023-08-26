#[cfg(test)]
mod test {
    use boleto_utils::cobranca::{Cobranca, CodigoMoeda, CodBanco};
    use chrono::NaiveDate;

    #[test]
    fn basic_functionality() {
        let builder = Cobranca::builder();

        let result = builder
            .cod_moeda(CodigoMoeda::Real)
            .cod_banco(CodBanco(301))
            .valor(99999999.99)
            .data_vencimento(NaiveDate::from_ymd_opt(2023, 7, 29).unwrap())
            .build();

        // Erro proposital para ler o output no terminal
        // panic!("{:?}", result.cod_barras);

        assert_eq!(result.cod_barras.as_bytes(), b"30198942699999999990000000000000000000000000");
    }
}
