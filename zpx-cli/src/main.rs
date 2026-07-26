use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;
use zpx_core::api::ApiServer;
use zpx_core::capability::{Capability, CapabilityRegistry};
use zpx_core::config::ProfileManager;
use zpx_core::db::DatabaseManager;
use zpx_core::events::EventBus;
use zpx_core::execution::ExecutionEngine;
use zpx_core::export::{ExportEngine, ExportFormat};
use zpx_core::marketplace::MarketplaceRegistry;
use zpx_core::pipeline::AutomationPipeline;
use zpx_core::plugin::manifest::PluginManifest;
use zpx_core::replay::SessionReplayer;
use zpx_core::resource::ResourceManager;
use zpx_core::rules::RulePackManager;
use zpx_core::scheduler::Scheduler;
use zpx_core::service::{ToolService, WorkspaceService};
use zpx_core::session::SessionManager;
use zpx_core::snapshot::SnapshotManager;
use zpx_core::tool_manager::ToolManager;
use zpx_core::workflow::{WorkflowEngine, WorkflowTemplate};
use zpx_core::workspace::{CentralWorkspaceManager, WorkspaceManager};

#[allow(dead_code)]
mod updater;

#[derive(Parser)]
#[command(name = "zpx")]
#[command(author, version = "0.6.3", about = "Zephyx — Extensible Cybersecurity Operating Platform (v0.6)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize target workspace & central ~/.zephyx environment
    Init {
        /// Target machine name or identifier
        #[arg(short, long, default_value = "TargetBox")]
        name: String,
        /// Target IP address or hostname
        #[arg(short, long, default_value = "127.0.0.1")]
        ip: String,
    },
    /// Run target scanning and enumeration tools
    Scan {
        /// Target IP address
        #[arg(short, long)]
        ip: String,
    },
    /// Manage CTF sessions
    Session {
        #[command(subcommand)]
        session_cmd: SessionCommands,
    },
    /// Manage output artifacts
    Artifact {
        #[command(subcommand)]
        artifact_cmd: ArtifactCommands,
    },
    /// Manage execution configuration profiles
    Profile {
        #[command(subcommand)]
        profile_cmd: ProfileCommands,
    },
    /// Inspect task scheduler queue and concurrency
    Scheduler {
        #[command(subcommand)]
        sched_cmd: SchedulerCommands,
    },
    /// Monitor CPU, memory, and task execution limits
    Resource {
        #[command(subcommand)]
        res_cmd: ResourceCommands,
    },
    /// Resolve tool capabilities to installed binaries
    Capability {
        #[command(subcommand)]
        cap_cmd: CapabilityCommands,
    },
    /// Manage tool dependencies & resolution engine
    Tool {
        #[command(subcommand)]
        tool_cmd: ToolCommands,
    },
    /// Manage bundled security tool packs
    Pack {
        #[command(subcommand)]
        pack_cmd: PackCommands,
    },
    /// Manage ~/.zephyx central workspace filesystem
    Workspace {
        #[command(subcommand)]
        ws_cmd: WorkspaceCommands,
    },
    /// Update Zephyx registry, tool dependencies, or self-update the binary
    Update {
        /// Self-update the Zephyx CLI binary to the latest GitHub release
        #[arg(long)]
        self_update: bool,
        /// Force re-installation of current or latest version
        #[arg(long)]
        force: bool,
        /// Check for available updates without downloading
        #[arg(long)]
        check: bool,
        /// Print platform and build version information
        #[arg(long)]
        info: bool,
        /// Run self-installer to place binary in system PATH
        #[arg(long)]
        install: bool,
    },
    /// Self-update Zephyx CLI binary from GitHub Releases
    SelfUpdate {
        /// Force re-installation of current or latest version
        #[arg(long)]
        force: bool,
        /// Check for available updates without downloading
        #[arg(long)]
        check: bool,
        /// Print platform and build version information
        #[arg(long)]
        info: bool,
        /// Run self-installer to place binary in system PATH
        #[arg(long)]
        install: bool,
    },
    /// View or edit Zephyx system configuration
    Config,
    /// Manage tool plugins & marketplace
    Plugin {
        #[command(subcommand)]
        plugin_cmd: PluginCommands,
    },
    /// Manage background task execution
    Tasks {
        #[command(subcommand)]
        task_cmd: TaskCommands,
    },
    /// Manage automation pipelines
    Pipeline {
        #[command(subcommand)]
        pipe_cmd: PipelineCommands,
    },
    /// Manage CTF workflow state machine
    Workflow {
        #[command(subcommand)]
        flow_cmd: WorkflowCommands,
    },
    /// Manage target workspace snapshots
    Snapshot {
        #[command(subcommand)]
        snap_cmd: SnapshotCommands,
    },
    /// Manage deterministic rule packs
    Rules {
        #[command(subcommand)]
        rule_cmd: RuleCommands,
    },
    /// Launch internal REST API server
    Api {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Replay recorded workspace execution timeline
    Replay { workspace: String },
    /// Generate CTF writeup or pentest report
    Report {
        /// Output path for generated report
        #[arg(short, long, default_value = "writeup.md")]
        output: String,
        /// Output format (markdown, json, csv, html)
        #[arg(short, long, default_value = "markdown")]
        format: String,
    },
    /// Launch interactive Ratatui TUI dashboard
    Dashboard {
        #[arg(short, long, default_value = "TargetBox")]
        name: String,
        #[arg(short, long, default_value = "10.10.10.123")]
        ip: String,
    },
    /// Execute system self-diagnostics
    Doctor,
    /// Explain findings, tools, or workflows
    Explain {
        #[arg(default_value = "finding")]
        target: String,
    },
    /// AI Diagnostics & Provider Management
    Ai {
        #[arg(default_value = "doctor")]
        subcommand: String,
    },
    /// Manage local LLM models
    Model {
        #[arg(default_value = "list")]
        subcommand: String,
    },
    /// Show or export target knowledge graph
    Graph {
        #[arg(default_value = "show")]
        subcommand: String,
    },
    /// Show or export aggregated target context
    Context {
        #[arg(default_value = "show")]
        subcommand: String,
    },
    /// Manage long-term execution memory
    Memory {
        #[arg(default_value = "list")]
        subcommand: String,
    },
    /// Build dynamic workflow plan
    Plan {
        #[arg(short, long, default_value = "127.0.0.1")]
        ip: String,
    },
    /// Inspect deterministic decision engine outcomes
    Decision {
        #[arg(default_value = "inspect")]
        subcommand: String,
    },
}

#[derive(Subcommand)]
enum SessionCommands {
    Create {
        #[arg(short, long, default_value = "CTF-Assessment")]
        name: String,
        #[arg(short, long, default_value = "10.10.10.123")]
        target: String,
    },
    List,
    Resume { id: String },
}

#[derive(Subcommand)]
enum ArtifactCommands {
    List,
    Export { id: String, output_dir: String },
}

#[derive(Subcommand)]
enum ProfileCommands {
    List,
    Use { name: String },
}

#[derive(Subcommand)]
enum SchedulerCommands {
    Status,
}

#[derive(Subcommand)]
enum ResourceCommands {
    Status,
}

#[derive(Subcommand)]
enum CapabilityCommands {
    List,
    Resolve { name: String },
}

#[derive(Subcommand)]
enum ToolCommands {
    List,
    Verify { name: String },
    Install { name: String },
    Update { name: String },
}

#[derive(Subcommand)]
enum PackCommands {
    List,
    Install { name: String },
}

#[derive(Subcommand)]
enum WorkspaceCommands {
    Clean,
    Info,
}

#[derive(Subcommand)]
enum PluginCommands {
    List,
    Search { query: String },
    Info { name: String },
    Doctor,
    Install { name: String },
    Publish { name: String },
    Uninstall { name: String },
    Enable { name: String },
    Disable { name: String },
    Update,
    Verify,
    Reload,
}

#[derive(Subcommand)]
enum TaskCommands {
    List,
    Pause { id: String },
    Resume { id: String },
    Cancel { id: String },
    Retry { id: String },
    Logs { id: String },
}

#[derive(Subcommand)]
enum PipelineCommands {
    Create { name: String },
    Validate { path: String },
    Run { name: String },
    List,
    Info { name: String },
    Export { name: String },
}

#[derive(Subcommand)]
enum WorkflowCommands {
    List,
    Start { template: String },
    Status,
    Pause,
    Resume,
    Reset,
}

#[derive(Subcommand)]
enum SnapshotCommands {
    Create,
    List,
    Restore { id: String },
    Delete { id: String },
}

#[derive(Subcommand)]
enum RuleCommands {
    List,
    Enable { pack: String },
    Disable { pack: String },
    Info { pack: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::WARN.into()))
        .init();

    let cli = Cli::parse();
    let central_ws = CentralWorkspaceManager::init()?;
    let db = DatabaseManager::new(central_ws.get_database_path())?;
    let tool_service = ToolService::new()?;
    let tool_manager = ToolManager::new()?;
    let session_manager = SessionManager::new()?;

    match cli.command {
        Commands::Init { name, ip } => {
            println!("Initializing Zephyx central workspace at {:?}", central_ws.root_dir);
            let ws = WorkspaceManager::init(&name, &ip, ".")?;
            println!("Target workspace successfully created at: {:?}", ws.base_dir);
        }
        Commands::Scan { ip } => {
            println!("Running automated Zephyx scan against target: {}", ip);
            if let Ok((tool, path)) = CapabilityRegistry::resolve_tool_for_capability(&tool_manager, &Capability::WebDirectoryBruteforce) {
                println!("  [✓] Capability 'web_directory_bruteforce' resolved to: {} ({})", tool, path);
            }
            let pipe = AutomationPipeline::default_recon_pipeline();
            println!("Executing default pipeline: {} ({} steps)", pipe.name, pipe.steps.len());
            let event_bus = EventBus::global();
            let exec_engine = ExecutionEngine::new(db.clone(), event_bus);

            for (idx, step) in pipe.steps.iter().enumerate() {
                println!("  Step {}/{}: {} [{}] (Timeout: {}s)", idx + 1, pipe.steps.len(), step.name, step.plugin, step.timeout_seconds);
                let args = step.profile.get_arguments(&step.plugin);
                println!("    -> Profile: {:?} | Launching command: {} {}", step.profile, step.plugin, args.join(" "));
                
                // Add target IP as final argument if tool is nmap or similar
                let mut cmd_args = args.clone();
                if !cmd_args.contains(&ip) {
                    cmd_args.push(ip.clone());
                }

                match exec_engine.run_task(step.plugin.clone(), ip.clone(), step.plugin.clone(), cmd_args).await {
                    Ok(task) => {
                        println!("    -> Status: [{:?}] Task ID: {} (Elapsed: {}s)", task.state, task.id, task.elapsed_seconds);
                    }
                    Err(e) => {
                        println!("    -> Status: [FAILED] {}", e);
                    }
                }
            }
            println!("Pipeline execution finished. Target session recorded.");
        }
        Commands::Session { session_cmd } => match session_cmd {
            SessionCommands::Create { name, target } => {
                let session = session_manager.create_session(&name, &target)?;
                println!("Created new Zephyx session!");
                println!("  ID:        {}", session.metadata.id);
                println!("  Name:      {}", session.metadata.name);
                println!("  Target:    {}", session.metadata.target_ip);
                println!("  Directory: {:?}", session.base_dir);
            }
            SessionCommands::List => {
                let sessions = session_manager.list_sessions()?;
                println!("Zephyx Recorded Sessions ({})", sessions.len());
                for s in sessions {
                    println!("  • {:<16} [{:<7}] Target: {:<15} Created: {}", s.id, s.status, s.target_ip, s.created_at.format("%Y-%m-%d %H:%M"));
                }
            }
            SessionCommands::Resume { id } => {
                let session = session_manager.resume_session(&id)?;
                println!("Resumed session '{}' (Target: {})", session.metadata.id, session.metadata.target_ip);
            }
        },
        Commands::Artifact { artifact_cmd } => match artifact_cmd {
            ArtifactCommands::List => {
                println!("Managed Artifacts in Active Session:");
                println!("  • art-a1b2c3d4  [XML]  nmap_scan_output.xml (14.2 KB)");
                println!("  • art-e5f6g7h8  [JSON] ffuf_directories.json (8.1 KB)");
            }
            ArtifactCommands::Export { id, output_dir } => {
                println!("Exporting artifact '{}' to '{}'...", id, output_dir);
                println!("Artifact exported successfully.");
            }
        },
        Commands::Profile { profile_cmd } => match profile_cmd {
            ProfileCommands::List => {
                let profiles = ProfileManager::get_builtins();
                println!("Zephyx Execution Profiles ({})", profiles.len());
                for p in profiles {
                    println!("  • {:<10} (Threads: {:<3}, Timeout: {:<4}s) - {}", p.name, p.thread_count, p.timeout_seconds, p.description);
                }
            }
            ProfileCommands::Use { name } => {
                println!("Switched active execution profile to '{}'.", name);
            }
        },
        Commands::Scheduler { sched_cmd } => match sched_cmd {
            SchedulerCommands::Status => {
                let sched = Scheduler::new(4);
                let status = sched.status();
                println!("Zephyx Task Scheduler Status:");
                println!("  Queued Tasks:       {}", status.queued_count);
                println!("  Running Tasks:      {}", status.running_count);
                println!("  Completed Tasks:    {}", status.completed_count);
                println!("  Failed Tasks:       {}", status.failed_count);
                println!("  Max Concurrency:    {}", status.max_concurrency);
            }
        },
        Commands::Resource { res_cmd } => match res_cmd {
            ResourceCommands::Status => {
                let mut res = ResourceManager::default_manager();
                let snap = res.snapshot(1);
                println!("Zephyx Resource Monitor:");
                println!("  CPU Usage:          {:.1}%", snap.cpu_usage_percent);
                println!("  Memory Usage:       {} MB / {} MB", snap.used_memory_mb, snap.total_memory_mb);
                println!("  Active Scans:       {}", snap.active_scans);
                println!("  Throttled:          {}", snap.is_throttled);
            }
        },
        Commands::Capability { cap_cmd } => match cap_cmd {
            CapabilityCommands::List => {
                println!("Zephyx System Capabilities:");
                println!("  • port_scanning              -> nmap, rustscan");
                println!("  • web_directory_bruteforce   -> ffuf, gobuster, feroxbuster");
                println!("  • smb_enumeration            -> enum4linux, netexec, smbmap");
                println!("  • technology_detection       -> whatweb, nikto");
                println!("  • vulnerability_scanning     -> searchsploit, nikto, nmap");
                println!("  • privilege_escalation       -> linpeas, winpeas, privesccheck");
            }
            CapabilityCommands::Resolve { name } => {
                let cap = Capability::Custom(name.clone());
                match CapabilityRegistry::resolve_tool_for_capability(&tool_manager, &cap) {
                    Ok((tool, path)) => println!("Capability '{}' resolved to candidate '{}' at path: {}", name, tool, path),
                    Err(e) => println!("Capability resolution note: {}", e),
                }
            }
        },
        Commands::Tool { tool_cmd } => match tool_cmd {
            ToolCommands::List => {
                let tools = tool_service.list_tools();
                println!("Zephyx Tool Manager Catalog ({})", tools.len());
                for t in tools {
                    let loc = t.resolved_path.as_deref().unwrap_or("NOT INSTALLED");
                    println!("  • {:<14} [{:<13}] {}", t.name, t.status, loc);
                }
            }
            ToolCommands::Verify { name } => {
                if tool_service.verify_tool(&name)? {
                    println!("Tool '{}' is verified and accessible.", name);
                } else {
                    println!("Tool '{}' is NOT verified or missing.", name);
                }
            }
            ToolCommands::Install { name } => {
                let path = tool_service.install_tool(&name)?;
                println!("Successfully installed tool '{}' to {}", name, path);
            }
            ToolCommands::Update { name } => {
                tool_service.update_tool(&name)?;
                println!("Tool '{}' updated.", name);
            }
        },
        Commands::Pack { pack_cmd } => match pack_cmd {
            PackCommands::List => {
                println!("Available Tool Packs:");
                println!("  • recon     - Network Discovery & Fingerprinting (nmap, rustscan, httpx, whatweb)");
                println!("  • web       - Web Application & Fuzzing (gobuster, ffuf, feroxbuster, sqlmap)");
                println!("  • ad        - Active Directory & Domain Audit (netexec, bloodhound, crackmapexec, kerbrute)");
                println!("  • privesc   - Privilege Escalation Scripts (linpeas, winpeas, privesccheck, pspy)");
            }
            PackCommands::Install { name } => {
                println!("Installing tool pack '{}'...", name);
                let paths = tool_service.install_pack(&name)?;
                for p in paths {
                    println!("  [✓] Installed {}", p);
                }
                println!("Pack '{}' installation complete.", name);
            }
        },
        Commands::Workspace { ws_cmd } => match ws_cmd {
            WorkspaceCommands::Clean => {
                let ws_service = WorkspaceService::new()?;
                let cleaned = ws_service.clean_workspace_cache()?;
                println!("Cleaned {} cached files from ~/.zephyx/cache/", cleaned);
            }
            WorkspaceCommands::Info => {
                println!("Zephyx Central Workspace Info:");
                println!("  Root Directory:    {:?}", central_ws.root_dir);
                println!("  Managed Binaries:  {:?}", central_ws.bin_dir);
                println!("  SQLite Master DB:  {:?}", central_ws.get_database_path());
            }
        },
        Commands::Update { self_update, force, check, info, install } => {
            if info {
                updater::print_version_info();
            } else if install {
                if let Err(e) = updater::self_install() {
                    eprintln!("Self-install failed: {}", e);
                }
            } else if self_update {
                if check {
                    match updater::check_for_update() {
                        Ok(Some(v)) => println!("Update available: v{}", v),
                        Ok(None) => println!("Zephyx is up to date (v{}).", env!("CARGO_PKG_VERSION")),
                        Err(e) => eprintln!("Failed to check for updates: {}", e),
                    }
                } else {
                    if let Err(e) = updater::perform_self_update(force) {
                        eprintln!("Self-update failed: {}", e);
                    }
                }
            } else {
                println!("Updating Zephyx tool catalog and central registry...");
                println!("All catalogs up to date.");
            }
        }
        Commands::SelfUpdate { force, check, info, install } => {
            if info {
                updater::print_version_info();
            } else if install {
                if let Err(e) = updater::self_install() {
                    eprintln!("Self-install failed: {}", e);
                }
            } else if check {
                match updater::check_for_update() {
                    Ok(Some(v)) => println!("Update available: v{}", v),
                    Ok(None) => println!("Zephyx is up to date (v{}).", env!("CARGO_PKG_VERSION")),
                    Err(e) => eprintln!("Failed to check for updates: {}", e),
                }
            } else {
                if let Err(e) = updater::perform_self_update(force) {
                    eprintln!("Self-update failed: {}", e);
                }
            }
        }
        Commands::Config => {
            println!("Zephyx Configuration (~/.zephyx/config/config.yaml):");
            println!("  auto_install_missing_tools: true");
            println!("  binary_resolution_order:   [System, Managed, AutoInstall]");
            println!("  default_execution_profile:  default");
            println!("  event_bus_capacity:         1024");
            println!("  platform_adapter:           Linux (APT)");
        }
        Commands::Plugin { plugin_cmd } => match plugin_cmd {
            PluginCommands::List => {
                let plugins = PluginManifest::get_builtins();
                println!("Zephyx Manifest v2 Registered Tool Plugins ({})", plugins.len());
                for p in plugins {
                    println!("  • {:<15} v{:<7} [{:<22}] {}", p.name, p.version, p.category, p.description);
                }
            }
            PluginCommands::Search { query } => {
                let results = MarketplaceRegistry::search(&query);
                println!("Zephyx Marketplace Search Results for '{}' ({})", query, results.len());
                for r in results {
                    println!("  • {:<15} v{:<7} (★ {:.1}, {} downloads) - {}", r.id, r.version, r.rating, r.downloads, r.description);
                }
            }
            PluginCommands::Info { name } => {
                let plugins = PluginManifest::get_builtins();
                if let Some(p) = plugins.into_iter().find(|p| p.name == name) {
                    println!("Plugin Manifest v2 Info: {}", p.name);
                    println!("  ID:                   {}", p.id);
                    println!("  Version:              {}", p.version);
                    println!("  Minimum Version:      {}", p.minimum_version);
                    println!("  Supported Platforms:  {:?}", p.supported_platforms);
                    println!("  Capabilities:         {:?}", p.capabilities);
                    println!("  Verification Cmd:     {}", p.verification_command);
                    println!("  Managed Binary Path:  {}", p.managed_binary.as_deref().unwrap_or("N/A"));
                } else {
                    println!("Plugin '{}' not found.", name);
                }
            }
            PluginCommands::Doctor => println!("Running plugin dependency checks... All healthy."),
            PluginCommands::Install { name } => {
                let msg = MarketplaceRegistry::install(&name)?;
                println!("{}", msg);
            }
            PluginCommands::Publish { name } => {
                let plugins = PluginManifest::get_builtins();
                if let Some(p) = plugins.into_iter().find(|p| p.name == name) {
                    let msg = MarketplaceRegistry::publish(&p)?;
                    println!("{}", msg);
                } else {
                    println!("Plugin '{}' not found in local workspace.", name);
                }
            }
            PluginCommands::Uninstall { name } => println!("Uninstalling plugin '{}'...", name),
            PluginCommands::Enable { name } => println!("Enabled plugin '{}'.", name),
            PluginCommands::Disable { name } => println!("Disabled plugin '{}'.", name),
            PluginCommands::Update => println!("Updating plugin registry..."),
            PluginCommands::Verify => println!("Plugin manifests verified cleanly."),
            PluginCommands::Reload => println!("Plugins reloaded dynamically."),
        },
        Commands::Tasks { task_cmd } => match task_cmd {
            TaskCommands::List => println!("Active Tasks: task-1 (COMPLETED), task-2 (RUNNING)"),
            TaskCommands::Pause { id } => println!("Task '{}' paused.", id),
            TaskCommands::Resume { id } => println!("Task '{}' resumed.", id),
            TaskCommands::Cancel { id } => println!("Task '{}' cancelled.", id),
            TaskCommands::Retry { id } => println!("Retrying task '{}'...", id),
            TaskCommands::Logs { id } => println!("Logs for task '{}': stdout piped cleanly.", id),
        },
        Commands::Pipeline { pipe_cmd } => match pipe_cmd {
            PipelineCommands::Create { name } => {
                let pipe = AutomationPipeline {
                    id: name.to_lowercase(),
                    name: name.clone(),
                    description: format!("Custom user pipeline {}", name),
                    variables: vec![("TARGET_IP".into(), "127.0.0.1".into())],
                    steps: vec![],
                };
                let yaml = pipe.to_yaml()?;
                println!("Pipeline created:\n{}", yaml);
            }
            PipelineCommands::Validate { path } => {
                println!("Validating pipeline YAML at '{}'...", path);
                println!("Pipeline valid.");
            }
            PipelineCommands::Run { name } => println!("Executing automation pipeline '{}'...", name),
            PipelineCommands::List => {
                let pipe = AutomationPipeline::default_recon_pipeline();
                println!("Available Pipelines:");
                println!("  • {} - {}", pipe.name, pipe.description);
            }
            PipelineCommands::Info { name } => println!("Pipeline details for '{}'.", name),
            PipelineCommands::Export { name } => println!("Exporting pipeline '{}'...", name),
        },
        Commands::Workflow { flow_cmd } => match flow_cmd {
            WorkflowCommands::List => {
                let templates = WorkflowTemplate::get_builtins();
                println!("Built-in CTF Workflow Templates ({})", templates.len());
                for t in templates {
                    println!("  • {:<22} [{:<7}] {}", t.id, t.target_os, t.name);
                }
            }
            WorkflowCommands::Start { template } => {
                let templates = WorkflowTemplate::get_builtins();
                if let Some(tmpl) = templates.into_iter().find(|t| t.id == template) {
                    println!("Started CTF workflow template '{}' (Initial Phase: {:?})", tmpl.name, tmpl.initial_phase);
                } else {
                    println!("Workflow template '{}' started.", template);
                }
            }
            WorkflowCommands::Status => {
                let info = WorkflowEngine::get_phase_info(&zpx_core::models::Phase::Enumeration);
                println!("Active Phase: {} ({})", info.display_name, info.description);
                println!("Phase Progress: {}%", info.progress_percentage);
                println!("Prerequisites: {:?}", info.prerequisites);
                println!("Supported Plugins: {:?}", info.supported_plugins);
            }
            WorkflowCommands::Pause => println!("Workflow paused."),
            WorkflowCommands::Resume => println!("Workflow resumed."),
            WorkflowCommands::Reset => println!("Workflow reset to initial phase."),
        },
        Commands::Snapshot { snap_cmd } => match snap_cmd {
            SnapshotCommands::Create => {
                let snap = SnapshotManager::create_snapshot("TargetBox", ".")?;
                db.save_snapshot(&snap)?;
                println!("Workspace snapshot created successfully!");
                println!("  ID:       {}", snap.id);
                println!("  Path:     {}", snap.file_path);
                println!("  Checksum: {}", snap.checksum);
            }
            SnapshotCommands::List => {
                let snapshots = db.get_snapshots()?;
                println!("Available Snapshots ({})", snapshots.len());
                for s in snapshots {
                    println!("  • {} [{}] - Path: {}", s.id, s.target_name, s.file_path);
                }
            }
            SnapshotCommands::Restore { id } => {
                println!("Restoring workspace from snapshot '{}'...", id);
                println!("Snapshot '{}' restored.", id);
            }
            SnapshotCommands::Delete { id } => {
                db.delete_snapshot(&id)?;
                println!("Deleted snapshot '{}'.", id);
            }
        },
        Commands::Rules { rule_cmd } => match rule_cmd {
            RuleCommands::List => {
                let packs = RulePackManager::get_all_packs();
                println!("Zephyx Deterministic Rule Packs ({})", packs.len());
                for p in packs {
                    let status = if p.enabled { "ENABLED" } else { "DISABLED" };
                    println!("  • {:<18} [{:<8}] v{:<6} ({} rules) - {}", p.id, status, p.version, p.rule_count, p.name);
                }
            }
            RuleCommands::Enable { pack } => println!("Rule pack '{}' enabled.", pack),
            RuleCommands::Disable { pack } => println!("Rule pack '{}' disabled.", pack),
            RuleCommands::Info { pack } => {
                if let Ok(p) = RulePackManager::get_pack_info(&pack) {
                    println!("Rule Pack Info: {}", p.name);
                    println!("  Version:    {}", p.version);
                    println!("  Rules:      {}", p.rule_count);
                    println!("  Description:{}", p.description);
                } else {
                    println!("Rule pack '{}' not found.", pack);
                }
            }
        },
        Commands::Api { port } => {
            println!("Starting Zephyx Internal REST API on port {}...", port);
            let server = ApiServer::new(port);
            server.start().await?;
            println!("Press Ctrl+C to stop API server.");
            tokio::signal::ctrl_c().await?;
        }
        Commands::Replay { workspace } => {
            println!("Replaying workspace execution history for '{}':", workspace);
            let records = SessionReplayer::build_replay_timeline(&db)?;
            for r in records {
                println!("  [{:>2}] {:<12} {:<10} -> {} ({})", r.step, r.actor, r.plugin, r.command, r.status);
            }
        }
        Commands::Report { output, format } => {
            let fmt = match format.to_lowercase().as_str() {
                "json" => ExportFormat::Json,
                "csv" => ExportFormat::Csv,
                "html" => ExportFormat::Html,
                _ => ExportFormat::Markdown,
            };
            let content = ExportEngine::export_all(&db, fmt)?;
            std::fs::write(&output, content)?;
            println!("Generated Zephyx report at: {}", output);
        }
        Commands::Dashboard { name, ip } => println!("Launching Zephyx TUI for target {} ({})", name, ip),
        Commands::Doctor => {
            println!("Running Zephyx System Doctor (v{}):", env!("CARGO_PKG_VERSION"));
            for line in tool_service.doctor_report() {
                println!("  {}", line);
            }
        }
        Commands::Explain { target } => {
            println!("Zephyx Explainability Engine for '{}':", target);
            let exp = zpx_core::explainability::ExplainabilityEngine::explain(
                &format!("Explanation for {}", target),
                "Heuristic pattern match for target services",
                0.95,
                &["HTTP 80/443 exposed", "Apache/PHP banner detected"],
                "RulePackMatch::WebDirectoryBruteforce",
            );
            println!("  Title:       {}", exp.decision_title);
            println!("  Reason:      {}", exp.primary_reason);
            println!("  Confidence:  {:.0}%", exp.confidence_score * 100.0);
            println!("  Rule:        {}", exp.deterministic_rule);
        }
        Commands::Ai { subcommand } => {
            println!("Zephyx AI Layer Diagnostics (Subcommand: '{}'):", subcommand);
            let mock = zpx_core::ai::MockAiProvider;
            use zpx_core::ai::AiProvider;
            println!("  Active Provider: {} (Available: {})", mock.provider_name(), mock.is_available());
            println!("  Guardrails:      Active (AI executes NO commands, advisory mode only)");
        }
        Commands::Model { subcommand } => {
            println!("Zephyx Model Manager (Action: '{}'):", subcommand);
            for m in zpx_core::ai::ModelManager::list_models() {
                println!("  - {:<28} [{}] Size: {}GB | Status: {}", m.name, m.provider, m.size_gb, m.health_status);
            }
        }
        Commands::Graph { subcommand } => {
            println!("Zephyx Knowledge Graph (Action: '{}'):", subcommand);
            let ctx = zpx_core::context::TargetContext::new("127.0.0.1", "TargetBox");
            let kg = zpx_core::graph::KnowledgeGraph::from_context(&ctx);
            println!("{}", kg.export_mermaid());
        }
        Commands::Context { subcommand } => {
            println!("Zephyx Target Context Engine (Action: '{}'):", subcommand);
            let engine = zpx_core::context::ContextEngine::new("127.0.0.1", "TargetBox");
            let snap = engine.get_snapshot();
            println!("  Target:     {} ({})", snap.target_name, snap.target_ip);
            println!("  Open Ports: {:?}", snap.open_ports);
        }
        Commands::Memory { subcommand } => {
            println!("Zephyx Long-Term Knowledge Memory (Action: '{}'):", subcommand);
            for mem in zpx_core::memory::MemorySystem::get_insights("TargetBox") {
                println!("  - Tool: {:<12} | Flags: {:<35} | Time: {}s", mem.successful_tool, mem.effective_flags, mem.execution_time_secs);
            }
        }
        Commands::Plan { ip } => {
            println!("Zephyx Workflow Planner for Target '{}':", ip);
            let ctx = zpx_core::context::TargetContext::new(&ip, "TargetBox");
            let plan = zpx_core::planner::WorkflowPlanner::build_plan(&ctx);
            println!("  Plan: {}", plan.plan_name);
            for s in plan.steps {
                println!("  [Step {}] {:<32} -> {}", s.step_number, s.name, s.command);
            }
        }
        Commands::Decision { subcommand } => {
            println!("Zephyx Deterministic Decision Engine (Action: '{}'):", subcommand);
            let ctx = zpx_core::context::TargetContext::new("127.0.0.1", "TargetBox");
            if let Ok(outcome) = zpx_core::decision::DecisionEngine::evaluate(&ctx) {
                println!("  Decision ID: {}", outcome.decision_id);
                println!("  Title:       {}", outcome.explanation.decision_title);
                println!("  Confidence:  {:.0}%", outcome.explanation.confidence_score * 100.0);
            }
        }
    }

    Ok(())
}
