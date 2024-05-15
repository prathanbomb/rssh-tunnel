use clap::{Parser};
use inquire::{Confirm, Text};
use std::process::Command;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "ssh-tunnel")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Clone)]
enum Commands {
    Tunnel,
}

struct SshConfig {
    jump_host_user: String,
    jump_host: String,
    target_host_user: String,
    target_host: String,
    jump_port: String,
    target_port: String,
    port_forward: bool,
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
        let jump_port: String = Text::new("Enter jump host SSH port (default: 22):")
            .with_default("22")
            .prompt()
            .context("Failed to get jump host port")?;
        let target_port: String = Text::new("Enter target host SSH port (default: 22):")
            .with_default("22")
            .prompt()
            .context("Failed to get target host port")?;

        let port_forward = Confirm::new("Port-Forward mode?")
            .with_default(true)
            .prompt()
            .context("Failed to confirm port-forward mode")?;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if !matches!(cli.command, Commands::Tunnel) {
        return Err(anyhow::anyhow!("Invalid command"));
    }

    let config = SshConfig::from_interactive_input()?;

    if config.port_forward {
        establish_interactive_tunnel(&config)?;
    } else {
        establish_non_interactive_tunnel(&config)?;
    }

    println!("SSH tunnel closed gracefully!");
    Ok(())
}

fn establish_interactive_tunnel(config: &SshConfig) -> Result<()> {
    let jump_ssh_args = format!("-J {}@{}:{}", config.jump_host_user, config.jump_host, config.jump_port);

    Command::new("ssh")
        .arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg("-p")
        .arg(&config.target_port)
        .status()
        .context("Failed to establish interactive SSH tunnel")?;

    println!("SSH tunnel established successfully!");
    Ok(())
}

fn establish_non_interactive_tunnel(config: &SshConfig) -> Result<()> {
    let local_port: String = Text::new("Enter local port for forwarding (default: 8080):")
        .with_default("8080")
        .prompt()
        .context("Failed to get local port")?;

    let jump_ssh_args = format!("-J {}@{}:{}", config.jump_host_user, config.jump_host, config.jump_port);
    let tunnel_args = format!("-L {}:{}:{}", local_port, config.target_host, config.target_port);

    Command::new("ssh")
        .arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg(tunnel_args)
        .arg("-N")
        .spawn()
        .context("Failed to spawn SSH process")?
        .wait()
        .context("Failed to establish port-forward SSH tunnel")?;

    Ok(())
}