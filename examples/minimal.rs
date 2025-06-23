use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
};
use ratatui_minecraft::eventless;

fn main() {
    ratatui_minecraft::run((), |_, t| _ = t.draw(render), eventless)
}

fn render(frame: &mut Frame<'_>) {
    [
        ("green", Style::new().green()),
        ("bold", Style::new().bold()),
        ("italic", Style::new().italic()),
        ("underlined", Style::new().underlined()),
    ]
    .into_iter()
    .enumerate()
    .for_each(|(index, (name, style))| {
        frame.render_widget(
            Span::raw(name).style(style),
            Rect {
                y: frame.area().y + index as u16,
                ..frame.area()
            },
        )
    });
}
