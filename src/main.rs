use structopt::StructOpt;
use inquire::{Confirm, Text};
use std::process::Command;
use anyhow::{Context, Result};

#[derive(StructOpt)]
#[structopt(name = "rssh-tunnel", about = "SSH Tunnel CLI")]
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

        #[structopt(long, help = "Port to forward")]
        port_forward: Option<String>,
    },
}

struct SshConfig {
    jump_host_user: String,
    jump_host: String,
    target_host_user: String,
    target_host: String,
    jump_port: String,
    target_port: String,
    port_forward: Option<String>,
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
            port_forward: if port_forward { Some("true".to_string()) } else { None },
        })
    }

    fn from_non_interactive_input(jump_host_user: Option<String>,
                                  jump_host_address: Option<String>,
                                  target_host_user: Option<String>,
                                  target_host_address: Option<String>,
                                  jump_port: Option<String>,
                                  target_port: Option<String>,
                                  port_forward: Option<String>) -> Result<Self> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();

    match cli {
        Cli::Interactive => {
            let config = SshConfig::from_interactive_input()?;
            if let Some(ref port_forward) = config.port_forward {
                if port_forward == "true" {
                    establish_interactive_tunnel(&config)?;
                } else {
                    establish_non_interactive_tunnel_without_port_forward(&config)?;
                }
            } else {
                establish_non_interactive_tunnel(&config)?;
            }
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
            let config = SshConfig::from_non_interactive_input(jump_host_user,
                                                               jump_host_address,
                                                               target_host_user,
                                                               target_host_address,
                                                               jump_port,
                                                               target_port,
                                                               port_forward)?;
            if let Some(ref port_forward) = config.port_forward {
                if port_forward == "true" {
                    establish_non_interactive_tunnel(&config)?;
                } else {
                    establish_non_interactive_tunnel_without_port_forward(&config)?;
                }
            } else {
                establish_non_interactive_tunnel(&config)?;
            }
            println!("SSH tunnel closed gracefully!");
        }
    }

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
    let jump_ssh_args = format!("-J {}@{}:{}", config.jump_host_user, config.jump_host, config.jump_port);

    let mut command = Command::new("ssh");
    command.arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg("-p")
        .arg(&config.target_port);
    if let Some(forward_port) = &config.port_forward {
        command.arg("-L").arg(format!("{}:{}:{}", forward_port, config.target_host, config.target_port));
    }
    command.status().context("Failed to establish non-interactive SSH tunnel")?;

    println!("SSH tunnel established successfully!");
    Ok(())
}

fn establish_non_interactive_tunnel_without_port_forward(config: &SshConfig) -> Result<()> {
    let jump_ssh_args = format!("-J {}@{}:{}", config.jump_host_user, config.jump_host, config.jump_port);

    Command::new("ssh")
        .arg(jump_ssh_args)
        .arg(format!("{}@{}", config.target_host_user, config.target_host))
        .arg("-p")
        .arg(&config.target_port)
        .status()
        .context("Failed to establish non-interactive SSH tunnel without port forwarding")?;

    println!("SSH tunnel established successfully!");
    Ok(())
}