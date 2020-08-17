use sfml::graphics::{
    Font,
    Color,
};
use rlua::{Table, Result,};

#[derive(Clone,)]
pub struct Style {
    pub bg_color: Color,
    pub fg_color: Color,
}

impl Style {
    pub fn new(style_table: &Table) -> Self {
        let mut style = Style::default();
        color_from_table(&style_table, "bg_color").map(|c| style.bg_color = c).ok();
        color_from_table(&style_table, "fg_color").map(|c| style.fg_color = c).ok();
        style
    }

    pub fn default() -> Self {
        Style {
            bg_color: Color::WHITE,
            fg_color: Color::BLACK,
        }
    }
}

// give you the color inside the table at this key
fn color_from_table(style_table: &Table, key: &str) -> Result<Color> {
    let color_table: Table = style_table.get(key)?;
    let color = Color::rgba(
        color_table.get::<_, u8>(1)?,
        color_table.get::<_, u8>(2)?,
        color_table.get::<_, u8>(3)?,
        color_table.get::<_, u8>(4)?,
    );
    Ok(color)
}