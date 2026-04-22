use std::{env, fs, io};

use anyhow::{Context, Result};
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
  prelude::*, widgets::{Block, Borders, Clear, Paragraph, Wrap}
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Deck {
  cards: Vec<Card>,
}

#[derive(Debug, Clone, Deserialize)]
struct Card {
  front: String,
  back: String,
  hint: Option<String>,
}

#[derive(Debug, Default)]
struct App {
  cards: Vec<Card>,
  index: usize,
  showing_back: bool,
  should_quit: bool,
}

impl App {
  fn new(deck: Deck) -> Self {
    Self {
      cards: deck.cards,
      ..Self::default()
    }
  }

  fn is_empty(&self) -> bool {
    self.cards.is_empty()
  }

  fn current(&self) -> Option<&Card> {
    self.cards.get(self.index)
}

  fn flip(&mut self) {
    if !self.is_empty() {
      self.showing_back = !self.showing_back;
    }
  }

  fn next(&mut self) {
    if self.is_empty() {
      return;
    }

    self.index = (self.index + 1) % self.cards.len();
    self.showing_back = false;
  }

  fn previous(&mut self) {
    if self.is_empty() {
      return;
    }

    self.index = if self.index == 0 {
      self.cards.len() - 1
    } else {
      self.index - 1
    };

    self.showing_back = false;
  }

  fn card_title(&self) -> String {
    if self.is_empty() {
      "No cards".to_string()
    } else {
      let side = if self.showing_back { "Back" } else { "Front" };
      format!("{side} ({}/{})", self.index + 1, self.cards.len())
    }
  }

  fn card_text(&self) -> String {
    match self.current() {
      Some(card) => {
        let main = if self.showing_back {
          &card.back
        } else {
          &card.front
        };

        let hint = card
          .hint
          .as_deref()
          .map(str::trim)
          .filter(|s| !s.is_empty())
          .unwrap_or("-");

        format!("{main}\n\nHint: {hint}")
      }
      None => "No cards found.\n\nHint: -".to_string(),
    }
  }
}

fn load_deck(path: &str) -> Result<Deck> {
  let raw =
    fs::read_to_string(path).with_context(|| format!("failed to read deck file: {path}"))?;

  let deck: Deck = serde_yaml::from_str(&raw).context("failed to parse YAML deck")?;

  Ok(deck)
}

fn main() -> Result<()> {
  let path = env::args()
    .nth(1)
    .unwrap_or_else(|| "examples/deck.yml".to_string());

  let deck = load_deck(&path)?;
  run(deck)
}

fn run(deck: Deck) -> Result<()> {
  enable_raw_mode().context("failed to enable raw mode")?;

  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend).context("failed to create terminal")?;
  let mut app = App::new(deck);

  let result = (|| -> Result<()> {
    loop {
      terminal
        .draw(|f| ui(f, &app))
        .context("failed to draw UI")?;

      if app.should_quit {
        break;
      }

      if let Event::Key(key) = event::read().context("failed to read terminal event")? {
        if key.kind == KeyEventKind::Press {
          match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            KeyCode::Char('f') | KeyCode::Enter | KeyCode::Char(' ') => app.flip(),
            KeyCode::Char('n') | KeyCode::Right => app.next(),
            KeyCode::Char('p') | KeyCode::Left => app.previous(),
            _ => {}
          }
        }
      }
    }

    Ok(())
  })();

  let cleanup_result = cleanup_terminal(&mut terminal);
  result.and(cleanup_result)
}

fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
  disable_raw_mode().context("failed to disable raw mode")?;

  execute!(terminal.backend_mut(), LeaveAlternateScreen)
    .context("failed to leave alternate screen")?;

  terminal.show_cursor().context("failed to show cursor")?;
  Ok(())
}

fn ui(f: &mut Frame, app: &App) {
  let area = f.area();
  f.render_widget(Clear, area);

  let chunks = Layout::vertical([Constraint::Min(5), Constraint::Length(3)])
    .margin(1)
    .split(area);

  let card = Paragraph::new(app.card_text())
    .block(
      Block::default()
        .title(app.card_title())
        .borders(Borders::NONE),
    )
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Left);

  let footer = Paragraph::new("[Q] Quit   [F/Enter/Space] Flip   [←/P] Prev   [→/N] Next")
    .block(
      Block::default()
      .style(Style::new().blue())
      .borders(Borders::NONE)
    )
    .alignment(Alignment::Center);

  f.render_widget(card, chunks[0]);
  f.render_widget(footer, chunks[1]);
}
