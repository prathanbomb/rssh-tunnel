# rush-tunnel

rush-tunnel is a command-line tool for creating SSH tunnels, allowing you to securely connect to remote hosts.

## Features

- **Interactive and Non-Interactive Modes:** Choose between interactive mode, where the CLI prompts you for input, or non-interactive mode, where you provide input via command-line options.
- **Profile Management:** Easily create, save, load, and overwrite SSH tunnel configurations as profiles.
- **Port Forwarding:** Optionally set up port forwarding for your SSH tunnels.
- **Easy Installation:** Install rush-tunnel easily using Cargo, the Rust package manager.

## Installation

To use rush-tunnel, you need to have Rust installed. You can then install rush-tunnel using Cargo, the Rust package manager:

```bash
cargo install rush-tunnel
```

## Usage
rush-tunnel provides both interactive and non-interactive modes for configuring SSH tunnels.

### Interactive Mode
To run the CLI in interactive mode:

```bash
rush-tunnel interactive
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
rush-tunnel tunnel [OPTIONS]
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
rush-tunnel tunnel \
    --jump-host-user <jump_host_user> \
    --jump-host-address <jump_host_address> \
    --target-host-user <target_host_user> \
    --target-host-address <target_host_address> \
    --jump-port <jump_ssh_port> \
    --target-port <target_ssh_port>
```

Create an SSH tunnel with port forwarding:

```bash
rush-tunnel tunnel \
    --jump-host-user <jump_host_user> \
    --jump-host-address <jump_host_address> \
    --target-host-user <target_host_user> \
    --target-host-address <target_host_address> \
    --jump-port <jump_ssh_port> \
    --target-port <target_ssh_port> \
    --port-forward <forward_to_port>
```

Replace `<jump_host_user>`, `<jump_host_address>`, `<target_host_user>`, `<target_host_address>`, `<jump_ssh_port>`, `<target_ssh_port>`, and `<forward_to_port>` with your desired values.

### Connect with Profile
You can connect to an SSH tunnel using a profile name with the following command:
```bash
rush-tunnel connect --profile <profile_name>
```
Replace `<profile_name>` with the name of the profile you want to connect to.

### Managing Profiles
You can also manage your profiles by listing all profiles or checking the profiles directory path.

To list all profiles:
```bash
rush-tunnel profiles
```

To show the profiles directory path:
```bash
rush-tunnel path
```

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.