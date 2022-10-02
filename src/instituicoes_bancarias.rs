extern crate csv;

use std::collections::HashMap;
use std::fs;
use std::fmt;

use lazy_static::lazy_static;


lazy_static! {
    static ref REGISTRY_BY_ID: HashMap<u16, InfoBanco> = {
        let file = fs::File::open("data/instituicoes-bancarias.csv").expect("File not opened");
        let mut reader = csv::Reader::from_reader(file);

        reader.records()
            .map(|line| {
                let record = line.expect("Line not parsed");
                let banco: InfoBanco = InfoBanco {
                    id: record[0].parse().expect("id not parsed"),
                    nome: record[1].to_string(),
                };
                (banco.id, banco)
            })
            .collect()
    };
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct InfoBanco {
    id: u16,
    nome: String,
}

impl fmt::Display for InfoBanco {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:03}] {}", self.id, self.nome)
    }
}

impl InfoBanco {
    pub fn get_by_id(id: u16) -> Option<&'static InfoBanco> {
        REGISTRY_BY_ID.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use crate::instituicoes_bancarias::REGISTRY_BY_ID;

    use super::InfoBanco;

    #[test]
    fn should_load_and_get_info_bancos() {
        let info = InfoBanco::get_by_id(0);
        assert!(matches!(info, None));

        println!("HashMap.len() == {}", REGISTRY_BY_ID.len());

        let info = InfoBanco::get_by_id(1).expect("Find by ID");
        assert_eq!(info.id, 1);
        assert!(info.nome.contains("Banco do Brasil"));

        let info = InfoBanco::get_by_id(341).expect("Find by ID");
        assert_eq!(info.id, 341);
        assert!(info.nome.contains("Itaú"));

        let info = InfoBanco::get_by_id(655).expect("Find by ID");
        assert_eq!(info.id, 655);
        assert!(info.nome.contains("Votorantim"));
    }
}
