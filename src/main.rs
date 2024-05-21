use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use inquire::{Confirm, CustomType, Password, Select, Text};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::crypto::{encrypt_password, is_password_strong};

mod crypto;

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
        jump_port: Option<i16>,

        #[structopt(long, help = "Target host SSH port (default: 22)")]
        target_port: Option<i16>,

        #[structopt(long, help = "Port to forward (default: no)")]
        port_forward: Option<i16>,
    },

    #[structopt(about = "Connect to SSH tunnel with profile name")]
    Connect {
        #[structopt(long, help = "Profile name to use")]
        profile: Option<String>,
    },

    #[structopt(about = "List all profiles")]
    Profiles,

    #[structopt(about = "Show the profiles directory path")]
    Path,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SshConfig {
    jump_host_user: String,
    jump_host: String,
    target_host_user: String,
    target_host: String,
    jump_port: i16,
    target_port: i16,
    port_forward: Option<i16>,
    enc1: Option<String>,
    enc2: Option<String>,
}

impl SshConfig {
    fn from_interactive_input() -> Result<Self> {
        let jump_host_user = prompt_input("Enter jump host username:")?;
        let jump_host = prompt_input("Enter jump host address:")?;
        let target_host_user = prompt_input("Enter target host username:")?;
        let target_host = prompt_input("Enter target host address:")?;
        let jump_port = prompt_port("Enter jump host SSH port (default: 22):", 22)?;
        let target_port = prompt_port("Enter target host SSH port (default: 22):", 22)?;
        let port_forward = CustomType::<i16>::new("Port-Forward? (default: no)")
            .prompt_skippable()
            .context("Failed to confirm port-forward")?;

        let (enc1, enc2) = if Confirm::new("Save password?").with_default(false).prompt()? {
            get_encrypted_passwords()?
        } else {
            (None, None)
        };

        Ok(SshConfig {
            jump_host_user,
            jump_host,
            target_host_user,
            target_host,
            jump_port,
            target_port,
            port_forward,
            enc1,
            enc2,
        })
    }

    fn from_non_interactive_input(
        jump_host_user: Option<String>,
        jump_host_address: Option<String>,
        target_host_user: Option<String>,
        target_host_address: Option<String>,
        jump_port: Option<i16>,
        target_port: Option<i16>,
        port_forward: Option<i16>,
    ) -> Result<Self> {
        let jump_host_user = jump_host_user.ok_or_else(|| anyhow::anyhow!("Missing jump host username"))?;
        let jump_host = jump_host_address.ok_or_else(|| anyhow::anyhow!("Missing jump host address"))?;
        let target_host_user = target_host_user.ok_or_else(|| anyhow::anyhow!("Missing target host username"))?;
        let target_host = target_host_address.ok_or_else(|| anyhow::anyhow!("Missing target host address"))?;
        let jump_port = jump_port.unwrap_or(22);
        let target_port = target_port.unwrap_or(22);

        let (enc1, enc2) = if Confirm::new("Save password?").with_default(false).prompt()? {
            get_encrypted_passwords()?
        } else {
            (None, None)
        };

        Ok(SshConfig {
            jump_host_user,
            jump_host,
            target_host_user,
            target_host,
            jump_port,
            target_port,
            port_forward,
            enc1,
            enc2,
        })
    }
}

fn prompt_input(message: &str) -> Result<String> {
    Text::new(message)
        .prompt()
        .context(format!("Failed to get {}", message))
}

fn prompt_port(message: &str, default: i16) -> Result<i16> {
    CustomType::<i16>::new(message)
        .with_default(default)
        .with_error_message("Please enter a valid port number between 1 and 65535")
        .prompt()
        .context(format!("Failed to get {}", message))
}

fn get_encrypted_passwords() -> Result<(Option<String>, Option<String>)> {
    let pj = Password::new("Enter password for jump host:").prompt()?;
    let pt = Password::new("Enter password for target host:").prompt()?;
    let mp = Password::new("Enter master password:").prompt()?;

    if is_password_strong(&mp) {
        let enc1 = Some(encrypt_password(&mp, &pj).expect("Failed to encrypt jump host password"));
        let enc2 = Some(encrypt_password(&mp, &pt).expect("Failed to encrypt target host password"));
        Ok((enc1, enc2))
    } else {
        println!("Master password is not strong enough.");
        Ok((None, None))
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
    let toml_str = fs::read_to_string(profile_path).context("Failed to read profile file")?;
    toml::from_str(&toml_str).context("Failed to deserialize profile")
}

fn save_profile(profile_name: &str, ssh_config: &SshConfig) -> Result<()> {
    let profiles_dir = get_profiles_dir()?;
    fs::create_dir_all(&profiles_dir).context("Failed to create profiles directory")?;
    let profile_path = Path::new(&profiles_dir).join(format!("{}.toml", profile_name));
    let toml_str = toml::to_string(&ssh_config).context("Failed to serialize profile")?;
    fs::write(profile_path, toml_str).context("Failed to write profile file")
}

fn save_or_overwrite_profile(profile_name: &str, ssh_config: &SshConfig) -> Result<()> {
    let profiles_dir = get_profiles_dir()?;
    let profile_path = Path::new(&profiles_dir).join(format!("{}.toml", profile_name));

    if profile_path.exists()
        && !Confirm::new(&format!("Profile '{}' already exists. Overwrite?", profile_name))
        .prompt()?
    {
        return Ok(());
    }

    save_profile(profile_name, ssh_config)
}

fn list_profiles() -> Result<Option<Vec<String>>> {
    let profiles_dir = get_profiles_dir()?;
    let profiles_path = Path::new(&profiles_dir);

    if !profiles_path.exists() {
        return Ok(None);
    }

    let profiles = fs::read_dir(profiles_path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| if ext == "toml" { Some(e) } else { None })
                    .and_then(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()))
            })
        })
        .collect::<Vec<_>>();

    Ok(Some(profiles))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();

    match cli {
        Cli::Interactive => {
            let config = SshConfig::from_interactive_input()?;
            let profile_name = prompt_input("Enter profile name:")?;
            save_or_overwrite_profile(&profile_name, &config)?;
            establish_tunnel(&config)?;
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Tunnel {
            jump_host_user,
            jump_host_address,
            target_host_user,
            target_host_address,
            jump_port,
            target_port,
            port_forward,
        } => {
            let config = SshConfig::from_non_interactive_input(
                jump_host_user,
                jump_host_address,
                target_host_user,
                target_host_address,
                jump_port,
                target_port,
                port_forward,
            )?;
            let profile_name = prompt_input("Enter profile name:")?;
            save_or_overwrite_profile(&profile_name, &config)?;
            establish_tunnel(&config)?;
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Connect { profile } => {
            if profile.is_none() {
                let profiles = list_profiles().expect("Failed to list profiles");
                let selected = Select::<String>::new("Select profile:", profiles.unwrap()).prompt()?;
                let ssh_config = load_profile(&selected)?;
                establish_tunnel(&ssh_config)?;
            } else {
                let ssh_config =
                    load_profile(&profile.clone().unwrap()).context(format!("Failed to load profile '{:?}'", &profile))?;
                establish_tunnel(&ssh_config)?;
            }
            println!("SSH tunnel closed gracefully!");
        }
        Cli::Profiles => {
            let profiles = list_profiles()?;
            if let Some(profiles) = profiles {
                for profile in profiles {
                    println!("- {}", profile);
                }
            } else {
                println!("No profiles found");
            }
        }
        Cli::Path => {
            let profiles_dir = get_profiles_dir()?;
            println!("Profiles directory: {}", profiles_dir);
        }
    }

    Ok(())
}

fn establish_tunnel(config: &SshConfig) -> Result<()> {
    println!("SSH Configuration:");
    println!(
        "  Jump Host:        {}@{}:{}",
        config.jump_host_user, config.jump_host, config.jump_port
    );
    println!(
        "  Target Host:      {}@{}:{}",
        config.target_host_user, config.target_host, config.target_port
    );

    let jump_ssh_args = format!(
        "-J {}@{}:{}",
        config.jump_host_user, config.jump_host, config.jump_port
    );

    let mut command = Command::new("ssh");
    command
        .arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg("-p")
        .arg(&config.target_port.to_string());
    if let Some(local_port) = &config.port_forward {
        command
            .arg("-L")
            .arg(format!(
                "{}:{}:{}",
                local_port, config.target_host, config.target_port
            ));
    }

    command
        .status()
        .context("Failed to establish SSH tunnel with port forwarding")?;

    Ok(())
}