# rssh-tunnel

rssh-tunnel is a command-line tool for creating SSH tunnels, allowing you to securely connect to remote hosts.

## Installation

To use rssh-tunnel, you need to have Rust installed. You can then install rssh-tunnel using Cargo, the Rust package manager:

```bash
cargo install rssh-tunnel
```

## Usage
rssh-tunnel provides both interactive and non-interactive modes for configuring SSH tunnels.

### Interactive Mode
To start an interactive session where you'll be prompted for configuration details:

```bash
rssh-tunnel interactive
```

## Non-Interactive Mode
To create an SSH tunnel without interacting with the prompts, you can use the following command:

```bash
rssh-tunnel tunnel \
    --jump-host-user <jump_host_user> \
    --jump-host-address <jump_host_address> \
    --target-host-user <target_host_user> \
    --target-host-address <target_host_address> \
    --jump-port <jump_ssh_port> \
    --target-port <target_ssh_port>
```

If you want to enable port forwarding in non-interactive mode, you can use the following command:

```bash
rssh-tunnel tunnel \
    --jump-host-user <jump_host_user> \
    --jump-host-address <jump_host_address> \
    --target-host-user <target_host_user> \
    --target-host-address <target_host_address> \
    --jump-port <jump_ssh_port> \
    --target-port <target_ssh_port> \
    --port-forward <forward_to_port>
```

Replace `<jump_host_user>`, `<jump_host_address>`, `<target_host_user>`, `<target_host_address>`, `<jump_ssh_port>`, `<target_ssh_port>`, and `<forward_to_port>` with your desired values.

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.