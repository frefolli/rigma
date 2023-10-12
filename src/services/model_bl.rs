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
    fn new() -> ModelSaver {
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
        model.absorb_asset();
        model.absorb_symbols(&view.asset.symbols);
        // model.absorb_produtions(&view.asset.productions);
        model
    }

    fn absorb_document(&mut self, view: &views::Document) {
        let form = forms::Document::from(&view.name, &view.description);
        let mut repo = repositories::Document::new();
        self.document = repo.create(&form);
    }

    fn absorb_asset(&mut self) {
        let form = forms::Asset::from(self.document.id);
        let mut repo = repositories::Asset::new();
        self.asset = repo.create(&form);
    }

    fn absorb_symbols(&mut self, view: &Vec<views::Symbol>) {
        let mut forms_ = Vec::<forms::Symbol>::new();
        for symbol in view {
            forms_.push(forms::Symbol::from(self.asset.id, &symbol.name, symbol.terminality));
        }
        let mut repo = repositories::Symbol::new();
        self.symbols = repo.import(&forms_);
    }

    fn absorb_productions(&mut self, view: &Vec<views::Production>) {
    } 
}
