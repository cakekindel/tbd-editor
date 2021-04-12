use prelude::*;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::Terminal;
use tui::backend::TermionBackend;

// TODO(orion): use something other than i64, long files will cause overflow
pub struct Pos(i64, i64);

pub enum Buffer {
  File {
    path: std::path::PathBuf,
    pos: Pos,
  },
  Scratch {
    pos: Pos,
  }
}

pub struct App {
  // TODO(orion): should all buffers be eagerly stored in mem?
  buffers: Vec<Buffer>,
  term: Terminal<TermionBackend<RawTerminal<std::io::Stdout>>>
}

impl App {
  pub fn try_new() -> Result<App, AnyError> {
    std::io::stdout()
      .into_raw_mode()
      .fmap(TermionBackend::new)
      .ee()
      .bind(ee!(Terminal::new))
      .then_do(ee!(Terminal::clear))
      .fmap(|term| Self {
        buffers: vec![],
        term,
      })
  }

  pub fn add_file(&mut self, path: impl ToString) -> Result<(), AnyError> {
    let path = path.to_string();

    std::env::current_dir()
      .ee()
      .fmap(|cwd| cwd.join(path))
      .then_do(|path| std::fs::read(path).ee()) // error if file does not exist
      .fmap(|path| self.buffers.push(Buffer::File {path, pos: Pos(0, 0)}))
  }

  pub fn draw(&mut self) -> Result<(), AnyError> {
    use tui::layout::{Layout, Constraint, Direction};
    use tui::widgets::{Paragraph, Widget, Block, Borders};
  
    self.term.draw(|frame| {
      let chunks = Layout::default()
                     .direction(Direction::Vertical)
                     .margin(1)
                     .constraints([
                       Constraint::Percentage(10),
                       Constraint::Percentage(80),
                       Constraint::Percentage(10),
                     ].as_ref())
                     .split(frame.size());
  
      frame.render_widget(
          Paragraph::new(vec![])
            .block(Block::default()
                             .title("Block")
                             .borders(Borders::ALL))
                         , chunks[0]
                         );
  
      frame.render_widget( Block::default()
                             .title("Block 2")
                             .borders(Borders::ALL)
                         , chunks[1]
                         );
    })
    .ee()
  }
}
