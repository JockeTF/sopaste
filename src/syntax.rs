use rocket::State;

use syntect::easy::HighlightLines;
use syntect::highlighting::Theme;
use syntect::highlighting::ThemeSet;
use syntect::html::styled_line_to_highlighted_html;
use syntect::html::IncludeBackground;
use syntect::parsing::SyntaxSet;

pub type Syntax = State<Syntect>;

pub struct Syntect {
    theme: Theme,
    types: SyntaxSet,
}

impl Syntect {
    pub fn new() -> Self {
        let themes = ThemeSet::load_defaults();
        let types = SyntaxSet::load_defaults_nonewlines();

        let theme = themes.themes["Solarized (dark)"].clone();

        Syntect { theme, types }
    }

    pub fn highlight(&self, language: &str, text: &str) -> String {
        use IncludeBackground::No;

        let theme = &self.theme;
        let types = &self.types;

        let syntax = types
            .find_syntax_by_token(language)
            .unwrap_or_else(|| types.find_syntax_plain_text());

        let mut highlighter = HighlightLines::new(syntax, theme);

        let html = text
            .lines()
            .map(|line| highlighter.highlight(line, types))
            .map(|line| styled_line_to_highlighted_html(&line, No))
            .map(|line| format!("<li>{}<br></li>", line))
            .collect::<String>();

        format!("<pre><ol>{}</ol></pre>", html)
    }
}
