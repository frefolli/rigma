use crate::models;
use crate::repositories;

pub struct ModelLoader {
    pub document: models::Document,
    pub asset: models::Asset,
    pub symbols: Vec<models::Symbol>,
    pub productions: Vec<models::Production>,
    pub branches: Vec<models::Branch>
}

impl ModelLoader {
    fn new() -> ModelSaver {
        ModelSaver {
            document: models::Document::new(),
            asset: models::Asset::new(),
            symbols: Vec::<models::Symbol>::new(),
            productions: Vec::<models::Production>::new(),
            branches: Vec::<models::Branch>::new()
        }
    }

    pub fn from(asset: i32) -> ModelLoader {
        let mut model = ModelLoader::new();
        model.load_asset(asset);
        model.load_document();
        model.load_symbols();
        model.load_productions();
        model.load_branches();
        model
    }

    pub fn load_document(&mut self) {
        self.document = repositories::Document::new().find(self.asset.document);
    }

    pub fn load_asset(&mut self, asset: i32) {
        self.asset = repositories::Asset::new().find(asset);
    }

    pub fn load_symbols(&mut self) {
        self.symbols = repositories::Symbol::new().filter(self.asset.id);
    }

    pub fn load_productions(&mut self) {
        self.productions = repositories::Symbol::new().filter(self.asset.id);
    }

    pub fn load_branches(&mut self) {
        branches = Vec::<models::Branch>::new();
        for prod in &self.productions {
            self.branches.append(&mut repositories::Branch::new().filter(prod.id));
        }
    }
}
