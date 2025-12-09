[![Русский](https://img.shields.io/badge/Русский-%F0%9F%87%B7%F0%9F%87%BA-green?style=for-the-badge)](README_ru.md)

# Rust Language Module for Plugify

The Rust Language Module for Plugify enables developers to write plugins in Rust for the Plugify framework. This module provides a seamless integration for Rust plugins, allowing them to be dynamically loaded and managed by the Plugify core.

## Features

- **Rust Plugin Support**: Write your plugins in Rust and integrate them effortlessly with the Plugify framework.
- **Automatic Exporting**: Easily export and import methods between plugins and the language module.
- **Initialization and Cleanup**: Handle plugin initialization, startup, and cleanup with dedicated module events.
- **Interoperability**: Communicate with plugins written in other languages through auto-generated interfaces.

## Getting Started

### Prerequisites

- Rust Compiler
- Plugify Framework Installed

### Installation

#### Option 1: Install via Plugify Plugin Manager

You can install the C++ Language Module using the Mamba package manager by running the following command:

```bash
mamba install -n your_env_name -c https://untrustedmodders.github.io/plugify-module-rust/ plugify-module-rust
```

#### Option 2: Manual Installation

1. Install dependencies:  

   a. Windows
   > Setting up [CMake tools with Visual Studio Installer](https://learn.microsoft.com/en-us/cpp/build/cmake-projects-in-visual-studio#installation)

   b. Linux:
   ```sh
   sudo apt-get install -y build-essential cmake ninja-build
   ```
   
   c. Mac:
   ```sh
   brew install cmake ninja
   ```

2. Clone this repository:

    ```bash
    git clone https://github.com/untrustedmodders/plugify-module-rust.git --recursive
    cd plugify-module-rust
    ```

3. Build the Rust language module:

    ```bash
    mkdir build && cd build
    cmake ..
    cmake --build .
    ```

### Usage

1. **Integration with Plugify**

   Ensure that your Rust language module is available in the same directory as your Plugify setup.

2. **Write Rust Plugins**

   Develop your plugins in Rust using the Plugify Rust API. Refer to the [Plugify Rust Plugin Guide](https://untrustedmodders.github.io/languages/rust/first-plugin) for detailed instructions.

3. **Build and Install Plugins**

   Compile your Rust plugins and place the shared libraries in a directory accessible to the Plugify core.

4. **Run Plugify**

   Start the Plugify framework, and it will dynamically load your Rust plugins.

## Example

### Initialize your module

```sh
cargo new my-rust-plugin
```

### Specify cdylib crate type

```toml
[lib]
crate-type = ["cdylib"]
```

### Get the rust-plugify module

Note that you need to include the v in the version tag.

```toml
[dependencies]
plugify = { git = "https://github.com/untrustedmodders/rust-plugify" }
```

```rust
use plugify::{register_plugin};

fn on_plugin_start() {
    println!("Rust: on_plugin_start")
}

fn on_plugin_update(_dt: f32) {
    println!("Rust: on_plugin_update")
}

fn on_plugin_end() {
    println!("Rust: on_plugin_end")
}

register_plugin!(
    start: on_plugin_start,
    update: on_plugin_update,
    end: on_plugin_end
);
```

## Documentation

For comprehensive documentation on writing plugins in Rust using the Plugify framework, refer to the [Plugify Documentation](https://untrustedmodders.github.io).

## Contributing

Feel free to contribute by opening issues or submitting pull requests. We welcome your feedback and ideas!

## License

This Rust Language Module for Plugify is licensed under the [MIT License](LICENSE).
