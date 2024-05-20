use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use inquire::{Confirm, Text};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rush-tunnel", about = "SSH Tunnel CLI")]
enum Cli {
    #[structopt(about = "Interactive mode")]
    Interactive,

    #[structopt(about = "Create SSH tunnel")]
    Tunnel {
        #[structopt(long, help = "Jump host username")]
        jump_host_user: Option<String>,

        #[structopt(long, help = "Jump host address")]
        jump_host_address: Option<String>,

        #[structopt(long, help = "Target host username")]
        target_host_user: Option<String>,

        #[structopt(long, help = "Target host address")]
        target_host_address: Option<String>,

        #[structopt(long, help = "Jump host SSH port (default: 22)")]
        jump_port: Option<String>,

        #[structopt(long, help = "Target host SSH port (default: 22)")]
        target_port: Option<String>,

        #[structopt(long, help = "Port to forward (default: no)")]
        port_forward: Option<String>,
    },

    #[structopt(about = "Connect to SSH tunnel with profile name")]
    Connect {
        #[structopt(long, help = "Profile name to use")]
        profile: String,
    },

    #[structopt(about = "List all profiles")]
    Profiles,

    #[structopt(about = "Show the profiles directory path")]
    Path,
}

#[derive(Serialize, Deserialize)]
pub struct SshConfig {
    jump_host_user: String,
    jump_host: String,
    target_host_user: String,
    target_host: String,
    jump_port: String,
    target_port: String,
    port_forward: Option<String>,
}

impl Default for SshConfig {
    fn default() -> Self {
        SshConfig {
            jump_host_user: String::default(),
            jump_host: String::default(),
            target_host_user: String::default(),
            target_host: String::default(),
            jump_port: String::from("22"),
            target_port: String::from("22"),
            port_forward: None,
        }
    }
}

impl SshConfig {
    fn from_interactive_input() -> Result<Self> {
        let jump_host_user = Text::new("Enter jump host username:")
            .prompt()
            .context("Failed to get jump host username")?;
        let jump_host = Text::new("Enter jump host address:")
            .prompt()
            .context("Failed to get jump host address")?;
        let target_host_user = Text::new("Enter target host username:")
            .prompt()
            .context("Failed to get target host username")?;
        let target_host = Text::new("Enter target host address:")
            .prompt()
            .context("Failed to get target host address")?;
        let jump_port = Text::new("Enter jump host SSH port (default: 22):")
            .with_default("22")
            .prompt()
            .context("Failed to get jump host port")?;
        let target_port = Text::new("Enter target host SSH port (default: 22):")
            .with_default("22")
            .prompt()
            .context("Failed to get target host port")?;
        let port_forward = Text::new("Port-Forward? (default: no)")
            .prompt()
            .context("Failed to confirm port-forward")?;

        Ok(SshConfig {
            jump_host_user,
            jump_host,
            target_host_user,
            target_host,
            jump_port,
            target_port,
            port_forward: Some(port_forward),
        })
    }

    fn from_non_interactive_input(
        jump_host_user: Option<String>,
        jump_host_address: Option<String>,
        target_host_user: Option<String>,
        target_host_address: Option<String>,
        jump_port: Option<String>,
        target_port: Option<String>,
        port_forward: Option<String>,
    ) -> Result<Self> {
        let jump_host_user = jump_host_user
            .ok_or_else(|| anyhow::anyhow!("Missing jump host username"))?;
        let jump_host = jump_host_address
            .ok_or_else(|| anyhow::anyhow!("Missing jump host address"))?;
        let target_host_user = target_host_user
            .ok_or_else(|| anyhow::anyhow!("Missing target host username"))?;
        let target_host = target_host_address
            .ok_or_else(|| anyhow::anyhow!("Missing target host address"))?;
        let jump_port = jump_port.unwrap_or_else(|| "22".to_string());
        let target_port = target_port.unwrap_or_else(|| "22".to_string());

        Ok(SshConfig {
            jump_host_user,
            jump_host,
            target_host_user,
            target_host,
            jump_port,
            target_port,
            port_forward,
        })
    }
}

fn get_profiles_dir() -> Result<String> {
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;
    let profiles_dir = home_dir.join(".rush-tunnel");
    Ok(profiles_dir.to_string_lossy().to_string())
}

fn load_profile(profile_name: &str) -> Result<SshConfig> {
    let profiles_dir = get_profiles_dir()?;
    let profile_path = Path::new(&profiles_dir).join(format!("{}.toml", profile_name));
    let toml_str =
        fs::read_to_string(profile_path).context("Failed to read profile file")?;
    let ssh_config: SshConfig = toml::from_str(&toml_str)?;
    Ok(ssh_config)
}

fn save_profile(profile_name: &str, ssh_config: &SshConfig) -> Result<()> {
    let profiles_dir = get_profiles_dir()?;
    fs::create_dir_all(&profiles_dir).context("Failed to create profiles directory")?;
    let profile_path = Path::new(&profiles_dir).join(format!("{}.toml", profile_name));
    let toml_str = toml::to_string(&ssh_config).context("Failed to serialize profile")?;
    fs::write(profile_path, toml_str).context("Failed to write profile file")?;
    Ok(())
}

fn save_or_overwrite_profile(
    profile_name: &str,
    ssh_config: &SshConfig,
) -> Result<()> {
    let profiles_dir = get_profiles_dir()?;
    let profile_path = Path::new(&profiles_dir).join(format!("{}.toml", profile_name));

    if profile_path.exists() {
        let overwrite_prompt = Confirm::new(&format!(
            "Profile '{}' already exists. Overwrite?",
            profile_name
        ))
            .prompt()?;
        if !overwrite_prompt {
            return Ok(());
        }
    }
    save_profile(profile_name, ssh_config)?;

    Ok(())
}

fn list_profiles() -> Result<()> {
    let profiles_dir = get_profiles_dir()?;
    let profiles_path = Path::new(&profiles_dir);

    if !profiles_path.exists() {
        println!("No profiles found.");
        return Ok(());
    }

    let mut profiles = Vec::new();
    for entry in fs::read_dir(profiles_path)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension == "toml" {
                if let Some(file_stem) = entry.path().file_stem() {
                    profiles.push(file_stem.to_string_lossy().to_string());
                }
            }
        }
    }

    if profiles.is_empty() {
        println!("No profiles found.");
    } else {
        println!("List of profiles:");
        for profile in profiles {
            println!(" - {}", profile);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();

    match cli {
        Cli::Interactive => {
            let config = SshConfig::from_interactive_input()?;
            let profile_name = Text::new("Enter profile name:").prompt()?;
            save_or_overwrite_profile(&profile_name, &config)?;
            establish_tunnel(&config)?;
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Tunnel { jump_host_user, jump_host_address, target_host_user, target_host_address, jump_port, target_port, port_forward } => {
            let config = SshConfig::from_non_interactive_input(
                jump_host_user,
                jump_host_address,
                target_host_user,
                target_host_address,
                jump_port,
                target_port,
                port_forward,
            )?;
            let profile_name = Text::new("Enter profile name:").prompt()?;
            save_or_overwrite_profile(&profile_name, &config)?;
            establish_tunnel(&config)?;
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Connect { profile } => {
            let ssh_config = load_profile(&profile)
                .context(format!("Failed to load profile '{}'", profile))?;
            establish_tunnel(&ssh_config)?;
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Profiles => {
            list_profiles()?;
        }
        Cli::Path => {
            let profiles_dir = get_profiles_dir()?;
            println!("Profiles directory: {}", profiles_dir);
        }
    }

    Ok(())
}

fn establish_tunnel(config: &SshConfig) -> Result<()> {
    let jump_ssh_args = format!("-J {}@{}:{}", config.jump_host_user, config.jump_host, config.jump_port);

    let mut command = Command::new("ssh");
    command
        .arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg("-p")
        .arg(&config.target_port);
    if let Some(local_port) = &config.port_forward {
        command
            .arg("-L")
            .arg(format!("{}:{}:{}", local_port, config.target_host, config.target_port));
    }
    command
        .status()
        .context("Failed to establish SSH tunnel with port forwarding")?;

    Ok(())
}