#![windows_subsystem = "windows"]
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Color, PlatformError, Widget, WindowDesc};

fn build_ui() -> impl Widget<()> {
    Flex::row().with_flex_child(
        Flex::column()
            .with_child(Label::new("Hello, ").with_text_color(Color::rgba(
                1.0,
                102.0 / 255.0,
                94.0 / 255.0,
                0.5,
            )))
            .with_child(Label::new("World")),
        1.0,
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}
