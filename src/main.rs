mod app;
mod config;
mod i18n;

fn main() -> cosmic::iced::Result {
    i18n::init();
    cosmic::applet::run::<app::AppModel>(())
}
