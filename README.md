A [ratatui](https://ratatui.rs) backend that uses [valence-screens](https://github.com/White-145/valence-screens) to render the terminal
![image](https://github.com/user-attachments/assets/cf43c751-d819-4325-a5f6-2ba99847ed28)
# Installation 
`cargo add --git https://github.com/janTatesa/ratatui-minecraft`
# Usage
The library provides a simple `run` method which takes an initial state, a function that refreshes the ui and a function that handles events. After implementing the application simply just `cargo run` and type `localhost` to the IP adress field in minecraft server creation. You will be greeted wwith more instructions
# Example
```rust
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
```
# WARNING
The server is set to offline mode (to make debugging easier) meaning that there is no account verification, and everyone who has access to the port `25565` on your computer can connect to the server and interact with your application. **THIS IS AN EXPERIMENTAL PROJECT, DON'T IMPLEMENT ANY SYSTEM APPLICATIONS WITH IT**
