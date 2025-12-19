use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

pub fn loader() -> FluentLanguageLoader {
    let loader = fluent_language_loader!();
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .expect("Falha ao carregar idiomas");
    loader
}

pub fn init() {
    let _ = loader();
}
