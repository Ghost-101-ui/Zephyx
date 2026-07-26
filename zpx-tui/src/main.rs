use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

use zpx_tui::{app::ActiveTab, app::App, ui::render_ui};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("TargetBox".into(), "10.10.10.123".into());

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error running Zephyx TUI: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        app.tick();
        terminal.draw(|f| render_ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.palette_open {
                    match key.code {
                        KeyCode::Esc => app.toggle_palette(),
                        KeyCode::Char(c) => app.palette_input.push(c),
                        KeyCode::Backspace => {
                            app.palette_input.pop();
                        }
                        KeyCode::Enter => {
                            app.toggle_palette();
                        }
                        _ => {}
                    }
                } else {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('p')
                    {
                        app.toggle_palette();
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('1') => app.active_tab = ActiveTab::Dashboard,
                        KeyCode::Char('2') => app.active_tab = ActiveTab::Logs,
                        KeyCode::Char('3') => app.active_tab = ActiveTab::Findings,
                        KeyCode::Char('4') => app.active_tab = ActiveTab::DecisionGraph,
                        KeyCode::Char('5') => app.active_tab = ActiveTab::Knowledge,
                        KeyCode::Char('6') => app.active_tab = ActiveTab::Tasks,
                        KeyCode::Char('7') => app.active_tab = ActiveTab::Explorer,
                        KeyCode::Char('8') => app.active_tab = ActiveTab::AttackGraph,
                        KeyCode::Char('9') => app.active_tab = ActiveTab::WorkflowPipeline,
                        _ => {}
                    }
                }
            }
        }
    }
}
