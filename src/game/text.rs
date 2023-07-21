use super::*;

impl GamePlayState {
    pub fn init_text<T, E>(&mut self, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        let style = MonoTextStyle::new(&ascii::FONT_4X6, Rgb565::WHITE);
        let Ok(_) =
            Text::new("ROOM", Point::new((SCREEN_WIDTH / 2) as i32 - 7, 10), style).draw(display)
        else {
            panic!("Failed to draw.");
        };

        let style = MonoTextStyle::new(&ascii::FONT_6X10, Rgb565::WHITE);
        let Ok(_) = Text::new(
            "  ^\n<   >\n  v",
            Point::new(10, (SCREEN_HEIGHT / 2) as i32 - 6),
            style,
        )
        .draw(display) else {
            panic!("Failed to draw.");
        };
        let Ok(_) = Text::new(
            "  Pew\n<L   R>\n  180",
            Point::new((SCREEN_WIDTH - 50) as i32, (SCREEN_HEIGHT / 2) as i32 - 6),
            style,
        )
        .draw(display) else {
            panic!("Failed to draw.");
        };
    }
}
