use rocket::State;

use syntect::highlighting::Color;
use syntect::highlighting::Theme;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub type Syntax = State<Syntect>;

pub struct Syntect {
    theme: Theme,
    types: SyntaxSet,
}

impl Syntect {
    pub fn new() -> Self {
        let themes = ThemeSet::load_defaults();
        let types = SyntaxSet::load_defaults_newlines();

        let mut theme = themes.themes["Solarized (dark)"].clone();

        theme.settings.background = Some(Color::BLACK);

        Syntect { theme, types }
    }

    pub fn highlight(&self, language: &str, text: &str) -> String {
        let theme = &self.theme;
        let types = &self.types;

        let syntax = types
            .find_syntax_by_token(language)
            .unwrap_or_else(|| types.find_syntax_plain_text());

        highlighted_html_for_string(text, types, syntax, theme)
    }
}
