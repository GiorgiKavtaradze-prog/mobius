<div align="center">

# Mobius

### A GPU-rendered terminal emulator with inline 3D graphics

**Inspired by TempleOS · Built with Rust & Bevy**

[![Crates.io](https://img.shields.io/crates/v/mobius?style=for-the-badge&logo=rust&logoColor=white&color=%23f74c00)](https://crates.io/crates/mobius)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge&logo=open-source-initiative&logoColor=white)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-edition-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Bevy](https://img.shields.io/badge/Bevy-0.19-%23e6e6e6?style=for-the-badge&logo=bevy&logoColor=white)](https://bevyengine.org)
[![GitHub](https://img.shields.io/badge/GitHub-GiorgiKavtaradze--prog%2Fmobius-black?style=for-the-badge&logo=github&logoColor=white)](https://github.com/GiorgiKavtaradze-prog/mobius)

---

[✨ Features](#-features) · [🚀 Installation](#-installation) · [⚡ Quick Start](#-quick-start) · [🎮 Key Bindings](#-key-bindings) · [🧊 3D Modes](#-3d-presentation-modes) · [📡 RGP Protocol](#-mobius-graphics-protocol-rgp) · [🧩 Widget](#-ratatui-widget) · [❄️ Nix](#-nix-packaging) · [🛠️ Development](#️-development)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🖥️ GPU-Rendered Terminal

- Hardware-accelerated rendering powered by **Bevy** & **wgpu**
- Smooth 60+ FPS text rendering with **Parley** & **Ratatui**
- Transparent window support with configurable opacity
- Low-power redraw mode to save battery

</td>
<td width="50%">

### 🧊 Inline 3D Graphics

- Place **3D objects directly inside terminal cells**
- Native support for **OBJ**, **GLB**, and **STL** formats
- Animated, colored, and transformable objects
- Custom **Mobius Graphics Protocol (RGP)** for terminal-native graphics

</td>
</tr>
<tr>
<td width="50%">

### 🐁 Spinning Rat Cursor

- A 3D **Cairo Spiny Mouse** replaces your boring block cursor
- Spins and bobs with configurable animation
- Fully customizable model, color, brightness & scale

</td>
<td width="50%">

### 🎥 Multiple 3D Presentation Modes

- **Flat 2D** — classic terminal view
- **Orthographic 3D** — warped terminal plane
- **Perspective 3D** — fly through your terminal
- **Möbius Strip 3D** — the terminal as a Möbius strip

</td>
</tr>
<tr>
<td width="50%">

### 🎮 10 Camera Presets

- Save & recall **10 persistent camera slots**
- Switch instantly with key bindings or via RGP
- Per-slot orthographic scale & perspective FOV

</td>
<td width="50%">

### 🎨 Fully Customizable

- TOML-based configuration
- Complete **ANSI 16-color theme** support
- Custom fonts, styles & sizes
- Remappable key bindings

</td>
</tr>
</table>

---

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/GiorgiKavtaradze-prog/mobius.git
cd mobius

# Build in release mode
cargo build --release

# Run Mobius
./target/release/mobius
```

### With Cargo

```bash
cargo install mobius
```

### With Nix

```bash
# Run directly
nix run github:GiorgiKavtaradze-prog/mobius

# Install to profile
nix profile install github:GiorgiKavtaradze-prog/mobius
```

> **Note:** Mobius requires a GPU with Vulkan/Metal/DX12 support (via wgpu).

---

## ⚡ Quick Start

```bash
# Launch with default settings
mobius

# Launch with a custom config file
mobius --config-file ~/.config/mobius/mobius.toml

# Launch with a custom window title
mobius --title "My Terminal"

# Launch a specific command
mobius --command zsh
```

### Configuration

Mobius looks for configuration in this order:

1. `--config-file` CLI argument
2. `$XDG_CONFIG_HOME/mobius/mobius.toml` (system config)
3. `config/mobius.toml` (local fallback)

```toml
[window]
width = 960
height = 620
opacity = 0.8

[terminal]
default_cols = 104
default_rows = 32
scrollback = 2000

[font]
family = "JetBrains Mono"
style = "Regular"
size = 14

[theme]
foreground = "#dcd7ba"
background = "#1f1f28"
cursor = "#7e9cd8"

[cursor.model]
path = "CairoSpinyMouse.obj"
scale_factor = 6.0
brightness = 0.5
visible = true

[cursor.animation]
spin_speed = 1.4
bob_speed = 2.2
bob_amplitude = 0.08
```

---

## 🎮 Key Bindings

| Key Combination               | Action                             |
| :---------------------------- | :--------------------------------- |
| `Ctrl+Alt+Enter`              | Toggle **Orthographic 3D** mode    |
| `Ctrl+Alt+P`                  | Toggle **Perspective 3D** mode     |
| `Ctrl+Alt+M`                  | Toggle **Möbius Strip** mode       |
| `Ctrl+Alt+Shift+0-9`          | Activate **camera preset** 0–9     |
| `Ctrl+Alt+↑` / `Ctrl+Alt+↓`   | Increase / decrease **plane warp** |
| `Alt+↑` / `Alt+↓`             | Scroll **up / down** one line      |
| `Alt+PageUp` / `Alt+PageDown` | Scroll **up / down** one page      |
| `Ctrl+Alt+C`                  | **Copy** selection                 |
| `Ctrl+Alt+V`                  | **Paste** clipboard                |
| `Ctrl+=` / `Ctrl+-`           | Increase / decrease **font size**  |
| `Ctrl+Alt+0`                  | **Reset** font size                |

> All bindings are fully remappable in `mobius.toml`.

---

## 🧊 3D Presentation Modes

### Flat 2D

The classic terminal experience — fast, familiar, and focused.

### Orthographic 3D

The terminal becomes a **warped 3D plane** you can tilt, rotate, and zoom. Adjust the warp with `Ctrl+Alt+↑/↓` for a curved, immersive surface.

### Perspective 3D

**Fly through your terminal** in true perspective. Pan, dolly, and look around with full 3D camera controls.

### Möbius Strip 3D

The terminal wraps around a **Möbius strip** — a one-sided surface where the terminal seamlessly loops back on itself. A trippy, mind-bending way to view your shell.

---

## 📡 Mobius Graphics Protocol (RGP)

RGP is a custom terminal protocol for inserting **3D objects as first-class inline terminal objects** — inspired by TempleOS-style inline document graphics.

### Transport

```
ESC _ mobius;g;<verb>[;<key=value>...] ESC \
```

### Verbs

| Verb | Description                                    |
| :--- | :--------------------------------------------- |
| `s`  | Support query & version detection              |
| `r`  | Register object asset (path or base64 payload) |
| `p`  | Place object in terminal cell space            |
| `u`  | Update object transform/style                  |
| `d`  | Delete object                                  |
| `c`  | Camera control (10 presets)                    |

### Example

```bash
# Register a 3D object
ESC _ mobius;g;r;id=7;fmt=obj;path=CairoSpinyMouse.obj ESC \

# Place it at row 5, column 10, spanning 3×2 cells
ESC _ mobius;g;p;id=7;row=5;col=10;w=3;h=2;animate=1;scale=1.0;depth=1.5;color=7fd0ff;brightness=1.0;ry=30 ESC \

# Rotate it later
ESC _ mobius;g;u;id=7;ry=180 ESC \

# Switch to perspective view with 50° FOV
ESC _ mobius;g;c;id=0;set=1;type=Persp;fov=50;rx=10;ry=20 ESC \

# Delete it
ESC _ mobius;g;d;id=7 ESC \
```

📖 **Full protocol spec:** [`protocols/graphics.md`](protocols/graphics.md)

---

## 🧩 Ratatui Widget

Mobius ships with **`ratatui-mobius`** — a [Ratatui](https://github.com/ratatui/ratatui) widget for embedding 3D objects in your own TUI applications.

```rust
use std::io;

use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
use ratatui_mobius::{ObjectFormat, MobiusGraphic, MobiusGraphicSettings};

fn main() -> io::Result<()> {
    let mut graphic = MobiusGraphic::new(
        MobiusGraphicSettings::new("assets/objects/SpinyMouse.glb")
            .id(7)
            .format(ObjectFormat::Glb)
            .animate(true)
            .scale(1.0)
            .depth(1.5)
            .rotation([0.0, 30.0, 0.0]),
    );

    graphic.register()?;

    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
    (&graphic).render(Rect::new(10, 5, 24, 10), &mut buf);

    // Update transform or style in-place after placement
    graphic.settings_mut().rotation = [0.0, 90.0, 0.0];
    graphic.settings_mut().brightness = 1.2;
    graphic.update()?;

    Ok(())
}
```

### Widget Examples

- [`big_rat.rs`](widget/examples/big_rat.rs) — minimal inline object demo
- [`document.rs`](widget/examples/document.rs) — TempleOS-inspired editor with embedded objects
- [`draw.rs`](widget/examples/draw.rs) — 2D drawing pane with live 3D preview
- [`rubiks_cube.rs`](widget/examples/rubiks_cube.rs) — interactive 3D Rubik's cube
- [`mobius_chess.rs`](widget/examples/mobius_chess.rs) — 3D Möbius strip chess board

📖 **Widget docs:** [`widget/README.md`](widget/README.md)

---

## ❄️ Nix Packaging

Mobius provides first-class Nix support with **NixOS** and **Home Manager** modules.

### NixOS

```nix
{
  programs.mobius = {
    enable = true;
    settings = {
      window = {
        opacity = 0.9;
        width = 1200;
        height = 800;
      };
      shell.program = "zsh";
      font = {
        family = "JetBrains Mono";
        size = 14;
      };
    };
  };
}
```

### Home Manager

```nix
{
  programs.mobius = {
    enable = true;
    settings = {
      window.opacity = 0.85;
      shell.program = "fish";
      theme = {
        foreground = "#c0caf5";
        background = "#1a1b26";
      };
    };
  };
}
```

### GPU Backend Selection

```nix
{
  programs.mobius = {
    enable = true;
    gpuBackend = "vulkan";    # or "gl" / "gles"
    gpuAdapter = "RTX 3060";  # substring match against adapter name
  };
}
```

📖 **Nix docs:** [`nix/README.md`](nix/README.md)

---

## 🛠️ Development

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Run with hot reload
cargo run

# Build optimized release
cargo build --release

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --check
```

### Project Structure

```
mobius/
├── src/              # Terminal runtime, rendering, scene, protocol
│   ├── camera.rs     # 10 camera presets & interaction
│   ├── config.rs     # TOML configuration
│   ├── keyboard.rs   # Key bindings & input translation
│   ├── mouse.rs      # Mouse selection & scrolling
│   ├── rgp.rs        # Mobius Graphics Protocol
│   ├── scene/        # 3D scene, planes, Möbius strip
│   └── terminal.rs   # Terminal surface & rendering
├── protocols/        # Protocol specifications
│   └── graphics.md   # RGP spec
├── widget/           # ratatui-mobius widget crate
├── config/           # Default configuration
├── assets/           # 3D models & icons
├── nix/              # Nix packaging & modules
└── website/          # Project website
```

---

## 📦 Assets

| Model                 | Format | Description                       |
| :-------------------- | :----- | :-------------------------------- |
| `CairoSpinyMouse.obj` | OBJ    | The iconic spinning rat cursor 🐁 |
| `SpinyMouse.glb`      | GLB    | Animated spiny mouse              |
| `Ferris.glb`          | GLB    | The Rust mascot 🦀                |
| `SkateMouse.stl`      | STL    | Skateboarding mouse 🛹            |

---

## 📄 License

Licensed under [The MIT License](LICENSE).

<div align="center">

---

**Made with 🧀 & 🦀 by [Giorgi Kavtardaze](https://github.com/GiorgiKavtaradze-prog)**

</div>
