//! TUI Client for RBK Robot
//!
//! An interactive Terminal User Interface for sending and receiving RBK messages.
//! Uses the seersdk-rs crate to communicate with robots.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example tui_client -- <robot_ip>
//! # Example:
//! cargo run --example tui_client -- localhost
//! ```
//!
//! # Controls
//!
//! - Arrow keys: Navigate between input field and command list
//! - Enter: Send command or execute selected preset
//! - Esc/q: Quit application
//! - Tab: Cycle through input fields

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use seersdk_rs::*;
use std::io;
use std::time::Duration;

/// Application state
struct App {
    robot_ip: String,
    client: RbkClient,
    input: String,
    cursor_position: usize,
    messages: Vec<String>,
    input_mode: InputMode,
    should_quit: bool,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new(robot_ip: String) -> Self {
        let client = RbkClient::new(robot_ip.clone());
        let messages = vec![
            "=== RBK Robot TUI Client ===".to_string(),
            format!("Connected to: {}", robot_ip),
                "".to_string(),
                "Available Commands:".to_string(),
                "  1. battery - Query battery status".to_string(),
                "  2. position - Query robot position".to_string(),
                "  3. info - Query robot information".to_string(),
                "  4. nav <target> - Navigate to target".to_string(),
                "  5. stop - Stop navigation".to_string(),
                "  6. pause - Pause navigation".to_string(),
                "  7. resume - Resume navigation".to_string(),
                "  8. jack load - Load jack".to_string(),
                "  9. jack unload - Unload jack".to_string(),
                "".to_string(),
                "Type a command and press Enter...".to_string(),
            ];
        Self {
            robot_ip,
            client,
            input: String::new(),
            cursor_position: 0,
            messages,
            input_mode: InputMode::Editing,
            should_quit: false,
        }
    }

    fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        // Keep only last 100 messages
        if self.messages.len() > 100 {
            self.messages.drain(0..50);
        }
    }

    async fn execute_command(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        self.add_message(format!("> {}", cmd));

        let result = match parts[0].to_lowercase().as_str() {
            "battery" | "bat" | "1" => self.query_battery().await,
            "position" | "pos" | "loc" | "2" => self.query_position().await,
            "info" | "3" => self.query_info().await,
            "nav" | "navigate" | "4" => {
                if parts.len() > 1 {
                    self.navigate_to_target(parts[1]).await
                } else {
                    Err("Usage: nav <target>".to_string())
                }
            }
            "stop" | "5" => self.stop_navigation().await,
            "pause" | "6" => self.pause_navigation().await,
            "resume" | "7" => self.resume_navigation().await,
            "jack" => {
                if parts.len() > 1 {
                    match parts[1].to_lowercase().as_str() {
                        "load" | "8" => self.jack_load().await,
                        "unload" | "9" => self.jack_unload().await,
                        _ => Err(format!("Unknown jack command: {}", parts[1])),
                    }
                } else {
                    Err("Usage: jack <load|unload>".to_string())
                }
            }
            "speed" => self.query_speed().await,
            "block" => self.query_block_status().await,
            "navstatus" => self.query_nav_status().await,
            "help" => {
                self.add_message("Available commands:".to_string());
                self.add_message("  battery, position, info, nav <target>".to_string());
                self.add_message("  stop, pause, resume, jack <load|unload>".to_string());
                Ok(())
            }
            "clear" => {
                self.messages.clear();
                self.add_message("=== RBK Robot TUI Client ===".to_string());
                Ok(())
            }
            _ => Err(format!("Unknown command: {}. Type 'help' for available commands.", parts[0])),
        };

        match result {
            Ok(_) => {}
            Err(e) => self.add_message(format!("Error: {}", e)),
        }
    }

    async fn query_battery(&mut self) -> Result<(), String> {
        let request = BatteryStatusRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                self.add_message("Battery Status:".to_string());
                self.add_message(format!("  Level: {:.1}%", response.battery_level * 100.0));
                self.add_message(format!("  Voltage: {:.2}V", response.voltage));
                self.add_message(format!("  Current: {:.2}A", response.current));
                self.add_message(format!("  Temperature: {:.1}°C", response.battery_temp));
                self.add_message(format!("  Charging: {}", response.charging));
                Ok(())
            }
            Err(e) => Err(format!("Failed to query battery: {}", e)),
        }
    }

    async fn query_position(&mut self) -> Result<(), String> {
        let request = RobotPoseRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                self.add_message("Robot Position:".to_string());
                self.add_message(format!("  X: {:.3}m", response.x));
                self.add_message(format!("  Y: {:.3}m", response.y));
                self.add_message(format!("  Angle: {:.3}rad ({:.1}°)", response.angle, response.angle.to_degrees()));
                self.add_message(format!("  Confidence: {:.1}%", response.confidence * 100.0));
                Ok(())
            }
            Err(e) => Err(format!("Failed to query position: {}", e)),
        }
    }

    async fn query_info(&mut self) -> Result<(), String> {
        let request = CommonInfoRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                self.add_message("Robot Information:".to_string());
                self.add_message(format!("  ID: {}", response.id));
                self.add_message(format!("  Model: {}", response.model));
                self.add_message(format!("  Version: {}", response.version));
                Ok(())
            }
            Err(e) => Err(format!("Failed to query info: {}", e)),
        }
    }

    async fn navigate_to_target(&mut self, target: &str) -> Result<(), String> {
        let move_cmd = MoveToTarget::new(target);
        let request = MoveToTargetRequest::new(move_cmd);
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                if response.code == StatusCode::Success {
                    self.add_message(format!("✓ Navigation started to target: {}", target));
                } else {
                    self.add_message(format!("✗ Navigation failed: {}", response.message));
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to navigate: {}", e)),
        }
    }

    async fn stop_navigation(&mut self) -> Result<(), String> {
        let request = StopExerciseRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_) => {
                self.add_message("✓ Navigation stopped".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to stop: {}", e)),
        }
    }

    async fn pause_navigation(&mut self) -> Result<(), String> {
        let request = PauseTaskRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_) => {
                self.add_message("✓ Navigation paused".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to pause: {}", e)),
        }
    }

    async fn resume_navigation(&mut self) -> Result<(), String> {
        let request = ResumeTaskRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_) => {
                self.add_message("✓ Navigation resumed".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to resume: {}", e)),
        }
    }

    async fn jack_load(&mut self) -> Result<(), String> {
        let request = LoadJackRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_) => {
                self.add_message("✓ Jack loading".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to load jack: {}", e)),
        }
    }

    async fn jack_unload(&mut self) -> Result<(), String> {
        let request = UnloadJackRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_) => {
                self.add_message("✓ Jack unloading".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to unload jack: {}", e)),
        }
    }

    async fn query_speed(&mut self) -> Result<(), String> {
        let request = RobotSpeedRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(_response) => {
                self.add_message("✓ Speed query completed".to_string());
                Ok(())
            }
            Err(e) => Err(format!("Failed to query speed: {}", e)),
        }
    }

    async fn query_block_status(&mut self) -> Result<(), String> {
        let request = BlockStatusRequest::new();
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                self.add_message("Block Status:".to_string());
                self.add_message(format!("  Blocked: {}", response.is_blocked));
                if let Some(reason) = response.reason {
                    self.add_message(format!("  Reason: {}", reason));
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to query block status: {}", e)),
        }
    }

    async fn query_nav_status(&mut self) -> Result<(), String> {
        let request = NavStatusRequest::new(GetNavStatus::new());
        match self.client.request(request, Duration::from_secs(5)).await {
            Ok(response) => {
                self.add_message("Navigation Status:".to_string());
                self.add_message(format!("  Status: {:?}", response.status));
                self.add_message(format!("  Type: {:?}", response.ty));
                self.add_message(format!("  Target: {}", response.target_id));
                Ok(())
            }
            Err(e) => Err(format!("Failed to query nav status: {}", e)),
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Messages
            Constraint::Length(3),  // Input
            Constraint::Length(3),  // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(format!("RBK Robot TUI Client - Connected to: {}", app.robot_ip))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Messages area
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = Line::from(Span::raw(m));
            ListItem::new(content)
        })
        .collect();

    let messages_widget = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .style(Style::default().fg(Color::White));
    f.render_widget(messages_widget, chunks[1]);

    // Input area
    let input_widget = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Command Input"));
    f.render_widget(input_widget, chunks[2]);

    // Set cursor position
    if app.input_mode == InputMode::Editing {
        f.set_cursor_position((chunks[2].x + app.cursor_position as u16 + 1, chunks[2].y + 1));
    }

    // Help text
    let help_text = match app.input_mode {
        InputMode::Normal => "Press 'i' to start editing, 'q' to quit",
        InputMode::Editing => "Press 'Esc' to stop editing, 'Enter' to send command",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[3]);
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if app.should_quit {
            break;
        }

        // Poll for events with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let cmd = app.input.drain(..).collect::<String>();
                            app.cursor_position = 0;
                            app.execute_command(&cmd).await;
                        }
                        KeyCode::Char(c) => {
                            app.input.insert(app.cursor_position, c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_position > 0 {
                                app.input.remove(app.cursor_position - 1);
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Left => {
                            if app.cursor_position > 0 {
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app.cursor_position < app.input.len() {
                                app.cursor_position += 1;
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get robot IP from command line arguments
    let args: Vec<String> = std::env::args().collect();
    let robot_ip = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: {} <robot_ip>", args[0]);
        println!("Example: {} localhost", args[0]);
        std::process::exit(1);
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(robot_ip);
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}
