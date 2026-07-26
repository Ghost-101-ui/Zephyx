use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::app::{ActiveTab, App};

pub fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header bar
            Constraint::Min(0),    // Main Workspace View
            Constraint::Length(1), // Status Bar
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_body(f, app, chunks[1]);
    render_status_bar(f, app, chunks[2]);

    if app.palette_open {
        render_palette_overlay(f, app, f.area());
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec![
        "[1] Dashboard",
        "[2] Logs",
        "[3] Findings",
        "[4] Decision Graph",
        "[5] Knowledge",
        "[6] Tasks",
        "[7] Explorer",
        "[8] Attack Graph",
        "[9] Pipeline",
    ];
    let index = match app.active_tab {
        ActiveTab::Dashboard => 0,
        ActiveTab::Logs => 1,
        ActiveTab::Findings => 2,
        ActiveTab::DecisionGraph => 3,
        ActiveTab::Knowledge => 4,
        ActiveTab::Tasks => 5,
        ActiveTab::Explorer => 6,
        ActiveTab::AttackGraph => 7,
        ActiveTab::WorkflowPipeline => 8,
        ActiveTab::Palette => 0,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Zephyx (zpx) v0.3 — Workflow Automation Environment "))
        .select(index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn render_body(f: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        ActiveTab::Dashboard => render_dashboard_view(f, app, area),
        ActiveTab::Logs => render_logs_view(f, app, area),
        ActiveTab::Findings => render_findings_view(f, app, area),
        ActiveTab::DecisionGraph => render_decision_view(f, app, area),
        ActiveTab::Knowledge => render_knowledge_view(f, app, area),
        ActiveTab::Tasks => render_tasks_view(f, app, area),
        ActiveTab::Explorer => render_explorer_view(f, app, area),
        ActiveTab::AttackGraph => render_attack_graph_view(f, app, area),
        ActiveTab::WorkflowPipeline => render_pipeline_view(f, app, area),
        ActiveTab::Palette => render_dashboard_view(f, app, area),
    }
}

fn render_dashboard_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(chunks[0]);

    // Target Overview Block
    let target_info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Target: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.target.name, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::styled(&app.target.ip, Style::default().fg(Color::Yellow)),
            Span::raw(")"),
        ]),
        Line::from(vec![
            Span::styled("Workflow Phase: ", Style::default().fg(Color::Gray)),
            Span::styled(app.target.phase.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("System Load: ", Style::default().fg(Color::Gray)),
            Span::raw(format!("CPU: {:.1}% | RAM: {} MB | Active Tasks: {}", app.cpu_usage, app.memory_usage, app.tasks.len())),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title(" Target & Workflow Engine Status "));

    f.render_widget(target_info, left_chunks[0]);

    // Live Findings Panel
    let findings_items: Vec<ListItem> = app
        .findings
        .iter()
        .map(|f| {
            ListItem::new(format!("[{}] {} -> {:?}", f.source_tool, f.target_ip, f.kind))
                .style(Style::default().fg(Color::White))
        })
        .collect();

    let findings_list = List::new(findings_items)
        .block(Block::default().borders(Borders::ALL).title(format!(" Discovered Findings ({}) ", app.findings.len())));

    f.render_widget(findings_list, left_chunks[1]);

    // Priority Recommendation Queue Panel
    let rec_items: Vec<ListItem> = app
        .recommendations
        .iter()
        .map(|r| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("[{}] ", r.priority), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled(&r.title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(Span::styled(format!("Tool: {} | Priority: {} | Status: {}", r.recommended_tool, r.priority, r.status), Style::default().fg(Color::Cyan))),
                Line::from(Span::styled(format!("Cmd: {}", r.suggested_command), Style::default().fg(Color::DarkGray))),
            ])
        })
        .collect();

    let rec_list = List::new(rec_items)
        .block(Block::default().borders(Borders::ALL).title(" Priority Recommendation Queue "));

    f.render_widget(rec_list, chunks[1]);
}

fn render_logs_view(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|l| ListItem::new(format!("[{}] [{}] [{}] {}", l.timestamp.format("%H:%M:%S"), l.level, l.source, l.message)))
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Process Logs & Stream "));
    f.render_widget(list, area);
}

fn render_findings_view(f: &mut Frame, app: &App, area: Rect) {
    let rows: Vec<Row> = app
        .findings
        .iter()
        .map(|f| {
            Row::new(vec![
                f.id[..8].to_string(),
                f.source_tool.clone(),
                format!("{:?}", f.kind),
                f.timestamp.format("%H:%M:%S").to_string(),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(30),
            Constraint::Length(12),
        ],
    )
    .header(Row::new(vec!["ID", "Tool", "Finding Details", "Time"]).style(Style::default().fg(Color::Yellow)))
    .block(Block::default().borders(Borders::ALL).title(" Target Workspace Findings Matrix "));

    f.render_widget(table, area);
}

fn render_decision_view(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled("Decision Engine Graph & Permanent Journal", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    for rec in &app.recommendations {
        lines.push(Line::from(Span::styled(format!("Trigger Action: {}", rec.title), Style::default().fg(Color::Yellow))));
        for step in &rec.reasoning {
            lines.push(Line::from(format!("  ├─► {}", step)));
        }
        lines.push(Line::from(format!("  └─► Recommended Executable: {}", rec.suggested_command)));
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Deterministic Decision Chains "));
    f.render_widget(p, area);
}

fn render_knowledge_view(f: &mut Frame, _app: &App, area: Rect) {
    let kb = zpx_core::knowledge::KnowledgeBase::get_builtins();
    let mut lines = Vec::new();

    for article in kb {
        lines.push(Line::from(Span::styled(format!("[{}] {}", article.category, article.title), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))));
        lines.push(Line::from(article.description));
        lines.push(Line::from(Span::styled("Commands:", Style::default().fg(Color::Cyan))));
        for cmd in article.commands {
            lines.push(Line::from(format!("  $ {}", cmd)));
        }
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Offline Security Playbooks "));
    f.render_widget(p, area);
}

fn render_tasks_view(f: &mut Frame, app: &App, area: Rect) {
    let rows: Vec<Row> = app
        .tasks
        .iter()
        .map(|t| {
            Row::new(vec![
                t.id.clone(),
                t.plugin_name.clone(),
                t.state.to_string(),
                format!("{}%", t.progress_percentage),
                t.current_operation.clone(),
                format!("{}s / {}s", t.elapsed_seconds, t.estimated_seconds),
                format!("{:.1}% / {}MB", t.cpu_usage, t.memory_mb),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Min(25),
            Constraint::Length(14),
            Constraint::Length(16),
        ],
    )
    .header(Row::new(vec!["ID", "Plugin", "State", "Progress", "Operation", "Elapsed", "CPU / RAM"]).style(Style::default().fg(Color::Yellow)))
    .block(Block::default().borders(Borders::ALL).title(" Background Task Scheduler & Execution Monitor "));

    f.render_widget(table, area);
}

fn render_explorer_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let tree_items = vec![
        ListItem::new("📁 .zpx/TargetBox/"),
        ListItem::new("  ├── 📄 timeline.db (SQLite)"),
        ListItem::new("  ├── 📁 reports/"),
        ListItem::new("  ├── 📁 notes/"),
        ListItem::new("  ├── 📁 loot/"),
        ListItem::new("  ├── 📁 downloads/"),
        ListItem::new("  └── 📁 commands/"),
        ListItem::new(format!("🔌 Registered Plugins ({})", app.plugins.len())),
    ];

    let tree = List::new(tree_items).block(Block::default().borders(Borders::ALL).title(" Workspace Explorer "));
    f.render_widget(tree, chunks[0]);

    let details_p = Paragraph::new(vec![
        Line::from(Span::styled("Target Workspace Inspector", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(format!("Active Target: {} ({})", app.target.name, app.target.ip)),
        Line::from(format!("Target OS: {}", app.target.os.as_deref().unwrap_or("Unknown"))),
        Line::from(""),
        Line::from(Span::styled("Workflow Templates Available:", Style::default().fg(Color::Yellow))),
        Line::from("  • HTB Linux Machine Workflow"),
        Line::from("  • HTB Windows Machine Workflow"),
        Line::from("  • TryHackMe Web Application Workflow"),
        Line::from("  • Active Directory Domain Assessment"),
    ])
    .block(Block::default().borders(Borders::ALL).title(" Workspace Details & Workflow Templates "));

    f.render_widget(details_p, chunks[1]);
}

fn render_attack_graph_view(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled("Target Discovered Attack Graph", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    for node in &app.attack_nodes {
        lines.push(Line::from(format!(" 🟢 Node [{}] {}", node.node_type, node.label)));
        for edge in &app.attack_edges {
            if edge.source_id == node.id {
                lines.push(Line::from(format!("    ├──({:^18})──► Node {}", edge.relationship, edge.target_id)));
            }
        }
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Persistent Attack Graph Visualizer "));
    f.render_widget(p, area);
}

fn render_pipeline_view(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(format!("Active Pipeline: {}", app.active_pipeline.name), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(app.active_pipeline.description.clone()));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("Pipeline Step Sequence:", Style::default().fg(Color::Yellow))));
    for (idx, step) in app.active_pipeline.steps.iter().enumerate() {
        lines.push(Line::from(format!(
            " Step {}: {} (Plugin: {}, Profile: {:?}, Timeout: {}s)",
            idx + 1,
            step.name,
            step.plugin,
            step.profile,
            step.timeout_seconds
        )));
    }

    let p = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Automation Pipeline Runner "),
    );
    f.render_widget(p, area);
}

fn render_status_bar(f: &mut Frame, _app: &App, area: Rect) {
    let bar = Paragraph::new(
        " [1-9] Tabs | [Ctrl+P] Command Palette | [q] Quit | Zephyx Workflow Engine Active",
    )
    .style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(bar, area);
}

fn render_palette_overlay(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 4,
        width: area.width / 2,
        height: 8,
    };

    f.render_widget(Clear, popup_area);

    let popup = Paragraph::new(vec![
        Line::from("Type a command or workflow name..."),
        Line::from(""),
        Line::from(Span::styled(
            format!("> {}_", app.palette_input),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press [Esc] to dismiss",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Command Palette (Ctrl+P) "),
    );

    f.render_widget(popup, popup_area);
}
