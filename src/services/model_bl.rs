use crate::views;
use crate::forms;
use crate::models;
use crate::repositories;

pub struct ModelSaver {
    pub document: models::Document,
    pub asset: models::Asset,
    pub symbols: Vec<models::Symbol>,
    pub productions: Vec<models::Production>,
    pub branches: Vec<models::Branch>
}

impl ModelSaver {
    pub fn new() -> ModelSaver {
        ModelSaver {
            document: models::Document::new(),
            asset: models::Asset::new(),
            symbols: Vec::<models::Symbol>::new(),
            productions: Vec::<models::Production>::new(),
            branches: Vec::<models::Branch>::new()
        }
    }

    pub fn from(view: &views::Document) -> ModelSaver {
        let mut model = ModelSaver::new();
        model.absorb_document(view);
        // model.absorb_asset(view.asset);
        // model.absorb_symbols(view.asset.symbols);
        // model.absorb_productions(view.asset.productions);
        model
    }

    fn absorb_document(&mut self, view: &views::Document) {
        let form = forms::Document::from(&view.name, &view.description);
        // save Document to 
        let mut repo = repositories::Document::new();
        self.document = repo.create(&form);
    }

    pub fn save(&self) {
        // TODO
    }
}
