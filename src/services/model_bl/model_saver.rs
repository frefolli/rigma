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
        model.absorb_productions(&view.asset.grammar);
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
        let mut prod_repo = repositories::Production::new();
        let mut branch_repo = repositories::Branch::new();
        for prod_view in view {
            let left = self.identify_symbol(&prod_view.left);
            let prod = prod_repo.create(&forms::Production::from(self.asset.id, left));
            let mut branch_forms = Vec::<forms::Branch>::new();
            let mut index = 0;
            for sym in &prod_view.right {
                let sym_index = self.identify_symbol(&sym);
                branch_forms.push(forms::Branch::from(prod.id, sym_index, index));
                index += 1;
            }
            self.branches.append(&mut branch_repo.import(&branch_forms));
        }
    } 

    fn identify_symbol(&self, sym: &str) -> i32 {
        (&self.symbols).into_iter().position(|x| x.name == sym).expect("symbol should be in symbols array").try_into().expect("unable to convert index to i32")
    }
}
