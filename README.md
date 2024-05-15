# 🌟 Super Easy SSH Tunnel Setup with Rust! 🌟

Are you tired of dealing with complex SSH tunnel configurations? Say goodbye to the hassle and welcome simplicity with our incredible Rust-based SSH tunnel setup! 🎉

## What is it?

This tool is your ultimate solution for effortlessly setting up SSH tunnels. Whether you're connecting to a remote server or forwarding ports securely, our Rust-powered utility streamlines the entire process.

## Features

✨ **Interactive Configuration**: No more manual tinkering with configuration files! Our tool guides you through each step with friendly prompts, ensuring you never miss a setting.

✨ **Port-Forwarding Made Simple**: Need to forward ports securely? With just a few clicks, you'll have a secure tunnel up and running, allowing seamless communication between local and remote hosts.

✨ **Error-Free Setup**: Say goodbye to cryptic errors! Our tool handles all the intricacies of SSH tunneling, so you can focus on what's important without worrying about technicalities.

## How to Use

1. **Install Rust**: Before getting started, ensure you have Rust installed on your system. If not, no worries! Installing Rust is straightforward. Simply visit [rust-lang.org](https://www.rust-lang.org/) and follow the installation instructions for your operating system.

2. **Clone the Repository**: Once Rust is installed, clone our repository to your local machine using your preferred Git client. If you're not familiar with Git, you can download the repository as a ZIP file and extract it to your desired location.

3. **Navigate to the Repository**: Open your terminal or command prompt and navigate to the directory where you cloned or extracted the repository.

4. **Run the Tool**: To start using our SSH tunnel setup tool, execute the following command:

    ```bash
    cargo run --release --bin ssh-tunnel
    ```

   This command launches our tool in interactive mode, guiding you through the setup process step by step. Follow the prompts to enter the required information, including:

    - Jump host username and address
    - Target host username and address
    - SSH ports for the jump host and target host
    - Whether you want to enable port forwarding

5. **Sit Back and Relax**: Once you've provided all the necessary information, our tool handles the rest! It establishes the SSH tunnel according to your specifications, ensuring a seamless and secure connection between your local machine and the target host.

6. **Advanced Options**: For users comfortable with command-line parameters, our tool also supports non-interactive mode. Simply provide the required information as command-line arguments when running the tool. For example:

    ```bash
    cargo run --release --bin rssh-tunnel tunnel --jump-host-user <jump_host_user> --jump-host <jump_host_address> --target-host-user <target_host_user> --target-host <target_host_address> --jump-port <jump_ssh_port> --target-port <target_ssh_port> --port-forward <true/false>
    ```

   Replace `<jump_host_user>`, `<jump_host_address>`, `<target_host_user>`, `<target_host_address>`, `<jump_ssh_port>`, `<target_ssh_port>`, and `<true/false>` with your desired values.

7. **Enjoy Secure Communication**: With the SSH tunnel successfully established, you can now enjoy secure communication between your local machine and the target host. Whether you're accessing remote resources or forwarding ports for specific services, our tool ensures a hassle-free experience every time!

## Contributions

We welcome contributions from the community! If you have ideas for improvements or new features, feel free to open an issue or submit a pull request. Together, we can make SSH tunneling even better!

## Spread the Word!

Found our tool useful? Spread the word to your friends and colleagues! Let's make SSH tunneling a breeze for everyone.

## Get Started Now!

Ready to simplify your SSH tunnel setup? Clone our repository, follow the easy setup instructions, and get started today! Your hassle-free SSH experience awaits. ✨
