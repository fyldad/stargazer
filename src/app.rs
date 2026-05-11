use std::io;
use color_eyre::eyre::Context;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::style::Styled;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use crate::{CONSTELLATION_SIZE, HEIGHT, WIDTH};
use crate::sky::Sky;

#[derive(Default)]
pub struct App {
    exit: bool,
    pub(crate) corner: (u16, u16),
    sky: Sky,
}

impl App {

    pub(crate) fn get_text<'a>(&self, width: u16, height: u16) -> Text<'a> {
        let mut text = Text::default();

        for y in self.corner.0..height + self.corner.0 {
            let mut line = Line::from("");
            for x in self.corner.1..width + self.corner.1 {
                let key = format!("{}:{}", x / CONSTELLATION_SIZE, y / CONSTELLATION_SIZE);

                match self.sky.0.get(&key) {
                    Some(stars) => {

                        match stars
                            .iter()
                            .find(|star| star.width == x && star.height == y)
                        {
                            Some(star) => line.push_span(
                                star.to_string().set_style(star.spectral_type.get_color()),
                            ),
                            None => line.push_span(" "),
                        }
                    }
                    None => line.push_span(" "),
                };
            }
            text.push_line(line);
        }
        text
    }
    pub(crate) fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("Failed to handle events")?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_up(&mut self) {
        checked_decrease(&mut self.corner.0);
    }

    fn move_down(&mut self) {
        checked_increase(&mut self.corner.0, HEIGHT);
    }

    fn move_right(&mut self) {
        checked_increase(&mut self.corner.1, WIDTH);
    }

    fn move_left(&mut self) {
        checked_decrease(&mut self.corner.1);
    }
}

fn checked_increase(val: &mut u16, max: u16) {
    let new_val = *val + 1;
    if new_val <= max {
        *val += 1
    }
}

fn checked_decrease(val: &mut u16) {
    if *val != 0 {
        *val -= 1
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Stargazer App".bold());
        let instructions = Line::from(vec![
            "Move: ".into(),
            "Arrows. ".blue().bold(),
            "New sky".into(),
            "<N>. ".blue().bold(),
            "Quit".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text = self.get_text(area.width, area.height);

        Paragraph::new::<Text>(text)
            .block(block)
            .render(area, buf);
    }
}

impl App {

}
