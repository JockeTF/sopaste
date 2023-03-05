use std::iter::once;

use syntect::easy::HighlightLines;
use syntect::highlighting::Color;
use syntect::highlighting::Theme;
use syntect::highlighting::ThemeSet;
use syntect::html::styled_line_to_highlighted_html;
use syntect::html::IncludeBackground;
use syntect::parsing::SyntaxSet;
use syntect::Error;

const THEME: &str = "Solarized (dark)";

fn brighten_color(color: &mut Color) {
    color.r = color.r.saturating_add(32);
    color.g = color.g.saturating_add(32);
    color.b = color.b.saturating_add(32);
}

fn brighten_theme(theme: &mut Theme) {
    theme.settings.foreground.as_mut().map(brighten_color);

    for item in &mut theme.scopes {
        item.style.foreground.as_mut().map(brighten_color);
    }
}

pub struct Syntect {
    theme: Theme,
    types: SyntaxSet,
}

impl Syntect {
    pub fn new() -> Self {
        let themes = ThemeSet::load_defaults();
        let types = SyntaxSet::load_defaults_nonewlines();

        let mut theme = themes.themes[THEME].clone();

        brighten_theme(&mut theme);

        Syntect { theme, types }
    }

    pub fn highlight(&self, language: &str, text: &str) -> Result<String, Error> {
        use IncludeBackground::No;

        let theme = &self.theme;
        let types = &self.types;

        let syntax = types
            .find_syntax_by_token(language)
            .unwrap_or_else(|| types.find_syntax_plain_text());

        let mut highlighter = HighlightLines::new(syntax, theme);

        let html = text.lines().map(|line| {
            let styled = highlighter.highlight_line(line, types)?;
            let html = styled_line_to_highlighted_html(&styled, No)?;

            Ok(format!("<li>{html}<br></li>"))
        });

        let prefix = once(Ok(String::from("<pre><ol>")));
        let suffix = once(Ok(String::from("</ol></pre>")));

        prefix.chain(html).chain(suffix).collect()
    }
}
