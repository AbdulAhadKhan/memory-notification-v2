# Memory Notification v2

# Auto Notification - Memory Usage by Processes

The **Auto Notification - Memory Usage by Processes** project aims to develop a system utility that continuously monitors memory allocated to various processes during runtime. The script sends email notifications to system administrators when the memory consumption violates predefined policies. This automated solution helps system administrators identify and address excessive memory usage, ensuring optimal system performance.

## Features

-   Continuous monitoring of memory usage by processes.
-   Real-time notifications via email when policies are violated.
-   Policy customization using configurations.
-   Blazing fast utility using the Rust Lang 🦀.

## Prerequisites

-   Rust programming knowledge.
-   Familiarity with Linux/Unix system calls.
-   Proficiency in bash scripting.

## Usage

1. Ensure that Rust is installed on your system. If not, follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust.
2. Clone the repository to your local machine.
3. CD into the repository directory.
4. Run the following command to build the project:

    ```bash
    cargo build --release
    ```

5. Edit the `config.toml` file to customize the policies and email settings.
6. Run the following command to execute the project:

    ```bash
    ./target/release/memory-notification-v2
    ```

Alternatively you can install the utility system-wide by running the following command:

```bash
cargo install --path .
```

## Future Scope

The **Auto Notification - Memory Usage by Processes** project has potential for further enhancements and expansions, including:

-   Integration with other notification channels (SMS, instant messaging platforms).
-   Dynamic threshold configuration based on application requirements.
-   Detailed reports and analytics on memory usage trends.
-   Integration with existing monitoring systems (Nagios, Zabbix, etc.).

## Contributing

Contributions to this project are welcome. To contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes.
4. Push the branch to your forked repository.
5. Open a pull request with a detailed description of your changes.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

We would like to acknowledge the contributions of the open-source community and the valuable resources that helped in the development of this project.
