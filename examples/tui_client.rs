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
//! ## Normal Mode (press Esc to enter)
//! - i: Enter editing mode
//! - q: Quit application
//! - ?: Show help with all commands
//! - c: Clear screen
//! - j/↓: Scroll down one line
//! - k/↑: Scroll up one line
//! - d/PgDn: Scroll down one page
//! - u/PgUp: Scroll up one page
//! - g/Home: Jump to top
//! - G/End: Jump to bottom
//!
//! ## Editing Mode (default)
//! - Enter: Send command
//! - Esc: Enter normal mode
//! - Ctrl+j/Ctrl+↓: Scroll down
//! - Ctrl+k/Ctrl+↑: Scroll up
//! - Ctrl+c: Clear screen
//! - PgUp/PgDn/Home/End: Scroll navigation
//! - Left/Right: Move cursor
//! - Backspace: Delete character

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
        KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use reqwest;
use serde::{Deserialize, Serialize};
use seersdk_rs::*;
use std::io;
use std::time::Duration;

/// Waypoint structure for HTTP API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Waypoint {
    id: String,
    x: f64,
    y: f64,
}

/// Application state
struct App {
    robot_ip: String,
    http_url: String,
    client: RbkClient,
    http_client: reqwest::Client,
    input: String,
    cursor_position: usize,
    messages: Vec<String>,
    input_mode: InputMode,
    should_quit: bool,
    scroll_state: ListState,
    scroll_offset: usize,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new(robot_ip: String) -> Self {
        let client = RbkClient::new(robot_ip.clone());
        let http_client = reqwest::Client::new();
        let http_url = format!("http://{}:8080", robot_ip);
        let messages = vec![
            "=== RBK Robot TUI Client ===".to_string(),
            format!("Connected to: {}", robot_ip),
            "".to_string(),
            "Press '?' in Normal mode for help...".to_string(),
        ];
        let mut scroll_state = ListState::default();
        let scroll_offset = if messages.len() > 0 {
            messages.len() - 1
        } else {
            0
        };
        scroll_state.select(Some(scroll_offset));
        Self {
            robot_ip,
            http_url,
            client,
            http_client,
            input: String::new(),
            cursor_position: 0,
            messages,
            input_mode: InputMode::Editing,
            should_quit: false,
            scroll_state,
            scroll_offset,
        }
    }

    fn show_help(&mut self) {
        self.messages.clear();
        self.add_message("=== RBK Robot Commands ===".to_string());
        self.add_message("".to_string());
        self.add_message("Robot Control:".to_string());
        self.add_message("  battery (bat, 1)      - Query battery status".to_string());
        self.add_message("  position (pos, loc, 2) - Query robot position".to_string());
        self.add_message("  info (3)              - Query robot information".to_string());
        self.add_message("  speed                 - Query robot speed".to_string());
        self.add_message("  block                 - Query block status".to_string());
        self.add_message("  navstatus             - Query navigation status".to_string());
        self.add_message("".to_string());
        self.add_message("Navigation:".to_string());
        self.add_message("  nav <target> (4)      - Navigate to target".to_string());
        self.add_message("  stop (5)              - Stop navigation".to_string());
        self.add_message("  pause (6)             - Pause navigation".to_string());
        self.add_message("  resume (7)            - Resume navigation".to_string());
        self.add_message("".to_string());
        self.add_message("Jack Control:".to_string());
        self.add_message("  jack load (8)         - Load jack".to_string());
        self.add_message("  jack unload (9)       - Unload jack".to_string());
        self.add_message("".to_string());
        self.add_message("Waypoint Management:".to_string());
        self.add_message("  wp list               - List all waypoints".to_string());
        self.add_message("  wp add <id> <x> <y>   - Add waypoint".to_string());
        self.add_message("  wp delete <id>        - Delete waypoint".to_string());
        self.add_message("".to_string());
        self.add_message("Utility:".to_string());
        self.add_message("  help                  - Show this help".to_string());
        self.add_message("  clear                 - Clear screen".to_string());
        self.add_message("".to_string());
        self.add_message("=== Keyboard Shortcuts ===".to_string());
        self.add_message("".to_string());
        self.add_message("Normal Mode (press Esc):".to_string());
        self.add_message("  i                     - Enter editing mode".to_string());
        self.add_message("  q                     - Quit application".to_string());
        self.add_message("  ?                     - Show this help".to_string());
        self.add_message("  c                     - Clear screen".to_string());
        self.add_message("  j / ↓                 - Scroll down".to_string());
        self.add_message("  k / ↑                 - Scroll up".to_string());
        self.add_message("  d / PgDn              - Page down".to_string());
        self.add_message("  u / PgUp              - Page up".to_string());
        self.add_message("  g / Home              - Go to top".to_string());
        self.add_message("  G / End               - Go to bottom".to_string());
        self.add_message("".to_string());
        self.add_message("Editing Mode (default):".to_string());
        self.add_message("  Enter                 - Send command".to_string());
        self.add_message("  Esc                   - Normal mode".to_string());
        self.add_message("  Ctrl+↑ / Ctrl+k       - Scroll up".to_string());
        self.add_message("  Ctrl+↓ / Ctrl+j       - Scroll down".to_string());
        self.add_message("  Ctrl+c                - Clear screen".to_string());
        self.add_message("  PgUp/PgDn/Home/End    - Scroll navigation".to_string());
        self.add_message("".to_string());
    }

    fn clear_screen(&mut self) {
        self.messages.clear();
        self.add_message("=== RBK Robot TUI Client ===".to_string());
        self.add_message(format!("Connected to: {}", self.robot_ip));
        self.add_message("".to_string());
        self.add_message("Screen cleared. Press '?' for help.".to_string());
    }

    fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        // Keep only last 100 messages
        if self.messages.len() > 100 {
            self.messages.drain(0..50);
        }
        // Auto-scroll to bottom when new message is added
        self.scroll_to_bottom();
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
            self.scroll_state.select(Some(self.scroll_offset));
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll_offset < self.messages.len().saturating_sub(1) {
            self.scroll_offset += 1;
            self.scroll_state.select(Some(self.scroll_offset));
        }
    }

    fn scroll_page_up(&mut self, page_size: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(page_size);
        self.scroll_state.select(Some(self.scroll_offset));
    }

    fn scroll_page_down(&mut self, page_size: usize) {
        self.scroll_offset = (self.scroll_offset + page_size)
            .min(self.messages.len().saturating_sub(1));
        self.scroll_state.select(Some(self.scroll_offset));
    }

    fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.scroll_state.select(Some(0));
    }

    fn scroll_to_bottom(&mut self) {
        if !self.messages.is_empty() {
            self.scroll_offset = self.messages.len() - 1;
            self.scroll_state.select(Some(self.scroll_offset));
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
            "wp" | "waypoint" => {
                if parts.len() > 1 {
                    match parts[1].to_lowercase().as_str() {
                        "list" | "ls" => self.list_waypoints().await,
                        "add" => {
                            if parts.len() >= 5 {
                                let id = parts[2].to_string();
                                let x: Result<f64, _> = parts[3].parse();
                                let y: Result<f64, _> = parts[4].parse();
                                match (x, y) {
                                    (Ok(x), Ok(y)) => self.add_waypoint(id, x, y).await,
                                    _ => Err("Invalid coordinates. Usage: wp add <id> <x> <y>".to_string()),
                                }
                            } else {
                                Err("Usage: wp add <id> <x> <y>".to_string())
                            }
                        }
                        "delete" | "del" | "rm" => {
                            if parts.len() >= 3 {
                                self.delete_waypoint(parts[2]).await
                            } else {
                                Err("Usage: wp delete <id>".to_string())
                            }
                        }
                        _ => Err(format!("Unknown waypoint command: {}. Try: list, add, delete", parts[1])),
                    }
                } else {
                    Err("Usage: wp <list|add|delete>".to_string())
                }
            }
            "help" | "?" => {
                self.show_help();
                Ok(())
            }
            "clear" | "cls" => {
                self.clear_screen();
                Ok(())
            }
            _ => Err(format!(
                "Unknown command: {}. Type 'help' for available commands.",
                parts[0]
            )),
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
                self.add_message(format!(
                    "  Level: {:.1}%",
                    response.battery_level * 100.0
                ));
                self.add_message(format!(
                    "  Voltage: {:.2}V",
                    response.voltage
                ));
                self.add_message(format!(
                    "  Current: {:.2}A",
                    response.current
                ));
                self.add_message(format!(
                    "  Temperature: {:.1}°C",
                    response.battery_temp
                ));
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
                self.add_message(format!(
                    "  Angle: {:.3}rad ({:.1}°)",
                    response.angle,
                    response.angle.to_degrees()
                ));
                self.add_message(format!(
                    "  Confidence: {:.1}%",
                    response.confidence * 100.0
                ));
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
                    self.add_message(format!(
                        "✓ Navigation started to target: {}",
                        target
                    ));
                } else {
                    self.add_message(format!(
                        "✗ Navigation failed: {}",
                        response.message
                    ));
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

    async fn list_waypoints(&mut self) -> Result<(), String> {
        let url = format!("{}/waypoints", self.http_url);
        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Waypoint>>().await {
                        Ok(waypoints) => {
                            self.add_message("Waypoints:".to_string());
                            if waypoints.is_empty() {
                                self.add_message("  No waypoints defined".to_string());
                            } else {
                                for wp in waypoints {
                                    self.add_message(format!(
                                        "  {} - ({:.2}, {:.2})",
                                        wp.id, wp.x, wp.y
                                    ));
                                }
                            }
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to parse waypoints: {}", e)),
                    }
                } else {
                    Err(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Failed to connect: {}", e)),
        }
    }

    async fn add_waypoint(&mut self, id: String, x: f64, y: f64) -> Result<(), String> {
        let waypoint = Waypoint { id: id.clone(), x, y };
        let url = format!("{}/waypoints", self.http_url);
        match self.http_client.post(&url).json(&vec![waypoint]).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.add_message(format!("✓ Waypoint '{}' added at ({:.2}, {:.2})", id, x, y));
                    Ok(())
                } else {
                    Err(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Failed to add waypoint: {}", e)),
        }
    }

    async fn delete_waypoint(&mut self, id: &str) -> Result<(), String> {
        let url = format!("{}/waypoints/{}", self.http_url, id);
        match self.http_client.delete(&url).send().await {
            Ok(response) => {
                if response.status() == 204 {
                    self.add_message(format!("✓ Waypoint '{}' deleted", id));
                    Ok(())
                } else if response.status() == 404 {
                    Err(format!("Waypoint '{}' not found", id))
                } else {
                    Err(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Failed to delete waypoint: {}", e)),
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Messages
            Constraint::Length(3), // Input
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(format!(
        "RBK Robot TUI Client - Connected to: {}",
        app.robot_ip
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Messages area
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(m));
            let style = if Some(i) == app.scroll_state.selected() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let messages_widget = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "Messages [{}/{}] - Use ↑↓ PgUp/PgDn Home/End to scroll",
            app.scroll_offset + 1,
            app.messages.len()
        )))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(messages_widget, chunks[1], &mut app.scroll_state);

    // Input area
    let input_widget = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command Input"),
        );
    f.render_widget(input_widget, chunks[2]);

    // Set cursor position
    if app.input_mode == InputMode::Editing {
        f.set_cursor_position((
            chunks[2].x + app.cursor_position as u16 + 1,
            chunks[2].y + 1,
        ));
    }

    // Help text
    let help_text = match app.input_mode {
        InputMode::Normal => "Normal: i=edit q=quit ?=help c=clear j/k=scroll d/u=page g/G=top/bottom",
        InputMode::Editing => {
            "Edit: Esc=normal Enter=send Ctrl+c=clear Ctrl+j/k=scroll PgUp/PgDn/Home/End=nav"
        }
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
        terminal.draw(|f| ui(f, &mut app))?;

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
                        KeyCode::Char('?') => {
                            app.show_help();
                        }
                        KeyCode::Char('c') => {
                            app.clear_screen();
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_up();
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_down();
                        }
                        KeyCode::Char('u') | KeyCode::PageUp => {
                            app.scroll_page_up(10);
                        }
                        KeyCode::Char('d') | KeyCode::PageDown => {
                            app.scroll_page_down(10);
                        }
                        KeyCode::Char('g') | KeyCode::Home => {
                            app.scroll_to_top();
                        }
                        KeyCode::Char('G') | KeyCode::End => {
                            app.scroll_to_bottom();
                        }
                        _ => {}
                    },
                    InputMode::Editing => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            match key.code {
                                KeyCode::Char('k') | KeyCode::Up => {
                                    app.scroll_up();
                                }
                                KeyCode::Char('j') | KeyCode::Down => {
                                    app.scroll_down();
                                }
                                KeyCode::Char('c') => {
                                    app.clear_screen();
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
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
                                KeyCode::PageUp => {
                                    app.scroll_page_up(10);
                                }
                                KeyCode::PageDown => {
                                    app.scroll_page_down(10);
                                }
                                KeyCode::Home => {
                                    app.scroll_to_top();
                                }
                                KeyCode::End => {
                                    app.scroll_to_bottom();
                                }
                                KeyCode::Esc => {
                                    app.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            }
                        }
                    }
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
