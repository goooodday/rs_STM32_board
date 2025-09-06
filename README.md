# storm32-led-blink

This project is a basic LED blinking example for the STM32F103RCTx microcontroller, written in Rust.

## Getting Started

### Prerequisites

*   Rust toolchain with `rustup`
*   `cargo-embed` for flashing and debugging
*   `probe-run` (often used with `cargo-embed`)
*   An STM32F103RCTx development board

### Building the Project

To build the project, navigate to the `storm32-led-blink` directory and run:

```bash
cargo build --release
```

### Flashing and Running on the Board

To flash the compiled binary onto your STM32F103RCTx board and run it, use `cargo embed`:

```bash
cargo embed --release --chip STM32F103RCTx
```

This command will compile the project (if not already built), flash it to the board, and start a debug session (if configured).

## Images

You can add screenshots or diagrams here. Use Markdown image syntax:

```markdown
![Alt text for image](path/to/your/image.png)
```

For example:

![StormM32 LED](images/IMG_9734.png)
