use itertools::Itertools;
use ratatui::{
    backend::WindowSize,
    buffer::Cell,
    layout::{Position, Size},
    prelude::Backend,
    style::Modifier,
};
use std::{io::Result, mem};
use valence_screens::{
    buffer::ScreenBuffer,
    pixel::{ScreenPixel, Style},
};

use crate::{
    HEIGHT, WIDTH,
    color::{BACKGROUND, ColorType, FOREGROUND, ratatui_to_valence_color},
};

#[derive(Debug, Clone)]
pub struct MinecraftTermBackend {
    screen_buffer: ScreenBuffer,
    // The cursor is not rendered rn, this is for compatibility
    cursor_position: Position,
}

impl Default for MinecraftTermBackend {
    fn default() -> Self {
        let mut screen_buffer = ScreenBuffer::new(WIDTH, HEIGHT);
        (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .for_each(|(x, y)| screen_buffer.put_bg(x, y, BACKGROUND));
        Self {
            screen_buffer,
            cursor_position: Default::default(),
        }
    }
}

impl MinecraftTermBackend {
    pub(crate) fn screen_buffer(&self) -> ScreenBuffer {
        self.screen_buffer.clone()
    }
}

impl Backend for MinecraftTermBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        content.for_each(|(x, y, cell)| {
            let modifier = &cell.modifier;
            let mut fg_color = ratatui_to_valence_color(cell.fg, ColorType::Foreground);
            let mut bg = ratatui_to_valence_color(cell.bg, ColorType::Background);
            if modifier.contains(Modifier::REVERSED) {
                mem::swap(&mut fg_color, &mut bg);
            }

            let symbol = cell.symbol();
            self.screen_buffer.put(
                x as u32,
                y as u32,
                ScreenPixel {
                    fg_char: match symbol.len() {
                        1 => match symbol.chars().next().unwrap() {
                            char if char.is_ascii() => char,
                            _ => ' ',
                        },
                        _ => ' ',
                    },
                    fg_color,
                    bg,
                    fg_style: Style::new(
                        modifier.contains(Modifier::BOLD),
                        modifier.contains(Modifier::CROSSED_OUT),
                        modifier.contains(Modifier::UNDERLINED),
                        modifier.contains(Modifier::ITALIC),
                    ),
                },
            )
        });

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<Position> {
        Ok(self.cursor_position)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<()> {
        self.cursor_position = position.into();
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.screen_buffer().fill(ScreenPixel {
            bg: BACKGROUND,
            fg_char: ' ',
            fg_color: FOREGROUND,
            fg_style: Style::default(),
        });
        Ok(())
    }

    fn size(&self) -> Result<Size> {
        Ok(Size {
            width: WIDTH as u16,
            height: HEIGHT as u16,
        })
    }

    fn window_size(&mut self) -> Result<WindowSize> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
