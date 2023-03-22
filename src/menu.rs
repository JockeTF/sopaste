use askama::Template;

use crate::models::ListRow;

#[derive(Debug, PartialEq, Eq, Template)]
#[template(path = "menu/main.html")]
pub enum MainMenu {
    About,
    Create,
    None,
}

impl MainMenu {
    #[allow(clippy::needless_pass_by_value)]
    fn link(&self, target: MainMenu) -> String {
        let link = match target {
            MainMenu::About => "/about",
            MainMenu::Create => "/",
            MainMenu::None => "",
        };

        if self == &target {
            format!("<span class=\"link active\">{target:?}</span>")
        } else {
            format!("<a href=\"{link}\">{target:?}</a>")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PastePage {
    Highlight,
    Tree,
    Raw,
}

#[derive(Template)]
#[template(path = "menu/paste.html")]
pub struct PasteMenu<'a> {
    list: &'a ListRow,
    page: PastePage,
}

impl<'a> PasteMenu<'a> {
    pub fn new(list: &'a ListRow, page: PastePage) -> Self {
        PasteMenu { list, page }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn link(&self, target: PastePage) -> String {
        let id = &self.list.id;

        let link = match target {
            PastePage::Highlight => format!("/{id}"),
            PastePage::Tree => format!("/{id}/tree"),
            PastePage::Raw => format!("/{id}/raw"),
        };

        if self.page == target {
            format!("<span class=\"link active\">{target:?}</span>")
        } else {
            format!("<a href=\"{link}\">{target:?}</a>")
        }
    }
}
