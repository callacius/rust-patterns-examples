use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct Carro {
    marca: RwLock<String>,
    modelo: RwLock<String>,
    ano: RwLock<u32>,
}

#[allow(dead_code)]
impl Carro {
    pub fn instance() -> &'static Carro {
        lazy_static! {
            static ref CARRO_INSTANCE: Carro = Carro {
                marca: RwLock::new(String::from("Toyota")),
                modelo: RwLock::new(String::from("Corolla")),
                ano: RwLock::new(2022),
            };
        }
        &CARRO_INSTANCE
    }

    // Getters
    pub fn marca(&self) -> String {
        self.marca.read().unwrap().clone()
    }

    pub fn modelo(&self) -> String {
        self.modelo.read().unwrap().clone()
    }

    pub fn ano(&self) -> u32 {
        *self.ano.read().unwrap()
    }

    // Setters
    pub fn set_marca(&self, marca: String) {
        let mut w = self.marca.write().unwrap();
        *w = marca;
    }

    pub fn set_modelo(&self, modelo: String) {
        let mut w = self.modelo.write().unwrap();
        *w = modelo;
    }

    pub fn set_ano(&self, ano: u32) {
        let mut w = self.ano.write().unwrap();
        *w = ano;
    }
}
