use crate::config_parser::TableConfig;
use ansi_to_tui::IntoText;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::size;
use ratatui::buffer::Buffer;
use ratatui::style::Style as RatatuiStyle;
use ratatui::text::{Span, Text};
use ratatui::widgets::Widget;
use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tabled::settings::style::BorderColor;
use tabled::{
    settings::{
        object::{Columns, Rows},
        style::{HorizontalLine, Style},
        Alignment, Modify, Panel, Width,
    },
    Table, Tabled,
};

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    config: Vec<TableConfig>,
    input: String, // Add a field to store the input text
    scroll_offset: u16,
    max_height: usize,
}

fn get_column_width(max: u16, min_col_width: u16) -> u16 {
    if max < min_col_width {
        return max;
    }
    let number_of_columns = max / min_col_width;
    let remaining = max % min_col_width;
    min_col_width + remaining / number_of_columns
}

#[derive(Tabled)]
struct Row {
    h1: String,
    h2: String,
}

fn rgb(r: usize, g: usize, b: usize) -> tabled::settings::Color {
    tabled::settings::Color::rgb_fg(r as u8, g as u8, b as u8)
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(initial_query: String, config: Vec<TableConfig>) -> Self {
        Self {
            config: config,
            running: false,
            input: initial_query, // Initialize the input field
            scroll_offset: 0,
            max_height: 0,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    pub fn draw_to_buffer(mut self) {
        let (width, height) = size().unwrap();
        // Generate buffer
        let area = Rect::new(0, 0, width, height);
        let column_width = get_column_width(area.width, 40_u16) as usize;
        let config = filter_by_input(&self.config, &self.input);
        let buffer = self.generate_buffer(area, config, column_width);

        // Print the buffer
        print_buffer(&buffer);
    }

    pub fn create_simple_table(table_config: &TableConfig, width: usize) -> Table {
        let mut table = Vec::new();

        for binding in table_config.bindings.iter() {
            let row = Row {
                h2: binding.key.to_string(),
                h1: binding.name.to_string(),
            };

            table.push(row);
        }

        let mut table = Table::builder(table);
        table.remove_record(0);
        let mut table = table.build();

        let app_colors: [tabled::settings::Color; 12] = [
            rgb(242, 173, 159),
            rgb(240, 198, 198),
            rgb(245, 189, 230),
            rgb(198, 160, 246),
            rgb(238, 153, 160),
            rgb(245, 169, 127),
            rgb(238, 212, 159),
            rgb(166, 218, 149),
            rgb(139, 213, 202),
            rgb(145, 215, 227),
            rgb(125, 196, 228),
            rgb(138, 173, 244),
        ];

        let color = app_colors[hash_string(table_config.name.split(":").collect::<Vec<&str>>()[0])
            as usize
            % app_colors.len()]
        .clone();

        let style = Style::modern()
            .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
            .remove_vertical()
            .remove_horizontal();
        table
            .modify(Columns::first(), Alignment::left())
            .modify(Columns::last(), Alignment::right())
            .modify(Columns::first(), Width::wrap(25))
            .modify(Columns::last(), Width::wrap(width - 25 - 4))
            .with(Panel::header(table_config.name.clone()))
            .with(style)
            .with(Modify::new(Rows::new(..)).with(BorderColor::filled(color)))
            .with(Width::increase(width));

        table
    }

    fn generate_buffer(
        &mut self,
        area: Rect,
        config: Vec<TableConfig>,
        column_width: usize,
    ) -> Buffer {
        let mut y = vec![0; area.width as usize / column_width];
        let mut buffer = Buffer::empty(Rect::new(0, 0, area.width, 0));
        let mut max_height = 0;

        for table_config in config {
            let table = App::create_simple_table(&table_config, column_width - 2);
            let height = table.total_height() as u16;
            let column = y.iter().enumerate().min_by_key(|&(_, &h)| h).unwrap().0;
            let rect = Rect::new(
                (column * column_width) as u16,
                y[column], // Adjust y position to account for input field
                column_width as u16,
                height + 3,
            );

            if (buffer.area().height as usize) < (y[column] + height) as usize {
                buffer.resize(Rect::new(0, 0, area.width, y[column] + height + 3));
            }

            let widget =
                Text::from(table.to_string().into_text().unwrap()).style(RatatuiStyle::default());
            widget.render(rect, &mut buffer);
            y[column] += height;
            max_height = max_height.max(y[column] as usize);
        }

        self.max_height = max_height;
        buffer
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let column_width = get_column_width(area.width, 40_u16) as usize;

        // Render input field at the top
        let input_field = Paragraph::new(Line::from(vec![
            Span::raw(" Search: ")
                .fg(Color::Black)
                .bg(Color::Rgb(181, 232, 176)),
            Span::raw(format!(" {}", self.input.clone())).fg(Color::White),
        ]))
        .block(Block::default().borders(Borders::NONE));
        frame.render_widget(input_field, Rect::new(0, 0, area.width, 1));

        // Set the cursor position
        frame.set_cursor_position((10 + self.input.len() as u16, 0));

        // Filter the configuration based on user input
        let config = filter_by_input(&self.config, &self.input);

        // Generate buffer
        let buffer = self.generate_buffer(area, config, column_width);

        let mut buffer_widget = CustomBufferWidget::new(&buffer);
        buffer_widget.set_scroll_offset(self.scroll_offset);
        frame.render_widget(
            buffer_widget,
            Rect::new(0, 2, frame.area().width, frame.area().height - 3),
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::NONE, KeyCode::Char(ch)) => {
                self.input.push(ch);
                self.scroll_offset = 0;
            }
            (KeyModifiers::NONE, KeyCode::Backspace) => {
                self.input.pop();
            }

            (KeyModifiers::NONE, KeyCode::Down) => {
                let screen_height = size().unwrap().1 as usize;
                if (self.scroll_offset as usize + screen_height - 5) < self.max_height {
                    self.scroll_offset += 2;
                }
            }

            (KeyModifiers::NONE, KeyCode::Up) => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 2;
                }
            }
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

fn filter_by_input(config: &Vec<TableConfig>, input: &str) -> Vec<TableConfig> {
    let mut app_input = "";
    let mut binding_input = input;
    if input.contains(":") {
        (app_input, binding_input) = input.split_once(":").unwrap();
    }

    config
        .iter()
        .filter_map(|cfg| {
            if cfg.name.contains(app_input) {
                let bindings: Vec<_> = cfg
                    .bindings
                    .iter()
                    .filter(|binding| binding.name.contains(binding_input))
                    .cloned()
                    .collect();
                if !bindings.is_empty() {
                    return Some(TableConfig {
                        name: cfg.name.clone(),
                        bindings,
                    });
                }
            }
            None
        })
        .collect()
}

fn print_buffer(buffer: &Buffer) {
    for y in 0..buffer.area.height {
        let mut row_is_empty = true;
        for x in 0..buffer.area.width {
            let cell = buffer.cell((x, y)).unwrap();

            let symbol = cell.symbol();
            print!("{}", &symbol);

            if !symbol.trim().is_empty() {
                row_is_empty = false;
            }
        }

        // Early exit after last line with content
        println!();
        if row_is_empty {
            break;
        }
    }
}

struct CustomBufferWidget<'a> {
    buffer: &'a Buffer,
    scroll_offset: u16,
}

impl<'a> CustomBufferWidget<'a> {
    fn new(buffer: &'a Buffer) -> Self {
        CustomBufferWidget {
            buffer,
            scroll_offset: 0,
        }
    }

    fn set_scroll_offset(&mut self, offset: u16) {
        self.scroll_offset = offset;
    }
}

impl<'a> Widget for CustomBufferWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if buf.area.height < area.height + area.y {
            buf.resize(Rect::new(
                area.x,
                area.y,
                buf.area.width,
                area.height + area.y - buf.area.height,
            ));
        }

        for y in self.scroll_offset..self.buffer.area().height {
            for x in 0..self.buffer.area().width {
                let src_cell = self.buffer.cell((x, y)).unwrap();
                let dest_x = area.x + x;
                let dest_y = area.y + y - self.scroll_offset;
                if dest_x < buf.area().width && dest_y < buf.area().height {
                    buf.cell_mut((dest_x, dest_y))
                        .unwrap()
                        .set_symbol(src_cell.symbol())
                        .set_style(src_cell.style());
                }
            }
        }
    }
}
