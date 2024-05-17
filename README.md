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
To run the CLI in interactive mode:

```bash
rssh-tunnel interactive
```

In interactive mode, the CLI will prompt you to enter the following information:
* Jump host username
* Jump host address
* Target host username
* Target host address
* Jump host SSH port (default: 22)
* Target host SSH port (default: 22)
* Port to forward (default: no)

### Non-Interactive Mode
To create an SSH tunnel without interacting with the prompts, you can use the following command:

```bash
rssh-tunnel tunnel [OPTIONS]
```

#### Options:
* `--jump_host_user`: Jump host username.
* `--jump_host_address`: Jump host address.
* `--target_host_user`: Target host username.
* `--target_host_address`: Target host address.
* `--jump_port`: Jump host SSH port (default: 22).
* `--target_port`: Target host SSH port (default: 22).
* `--port_forward`: Port to forward (default: no).

#### Examples:

Create an SSH tunnel without port forwarding:

```bash
rssh-tunnel tunnel \
    --jump-host-user <jump_host_user> \
    --jump-host-address <jump_host_address> \
    --target-host-user <target_host_user> \
    --target-host-address <target_host_address> \
    --jump-port <jump_ssh_port> \
    --target-port <target_ssh_port>
```

Create an SSH tunnel with port forwarding:

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