use anyhow::Result;
use argh::FromArgs;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, warn};
use log4rs;
use ratatui::prelude::*;
use std::{
    error::Error,
    io, panic,
    time::{Duration, Instant},
};

mod app;
mod clashtui_state;
mod keys;
mod ui;
mod utils;

use crate::app::App;
use crate::ui::EventState;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics)?;

    Ok(())
}

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    let mut last_ev = EventState::NotConsumed;
    if app.clashtui_util.err_code != 0
    {
        app.popup_txt_msg("The config.yaml might be broken, the progrem will try to fix it".to_string());
        app.msgpopup.show(); // I think it`s strange to manually set visible after the msg is send
        terminal.draw(|f| app.draw(f))?;
    }
    loop {
        terminal.draw(|f| app.draw(f))?;

        last_ev = app.handle_last_ev(&last_ev);

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            last_ev = app.event(&event::read()?)?;
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
