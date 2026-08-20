# ratatui-mobius

`ratatui-mobius` is a Ratatui widget crate for placing Mobius inline 3D objects
through the [Mobius Graphics Protocol](../protocols/graphics.md).

The widget emits RGP APC sequences into a Ratatui buffer. When that buffer is
rendered inside Mobius, the terminal resolves the registered asset and draws it
as a 3D object anchored to the selected terminal region.

## When to Use It

Use `ratatui-mobius` when a Ratatui application needs to:

- Register OBJ, GLB, or STL assets with Mobius.
- Place a 3D object inside a widget area.
- Update transform, color, brightness, scale, depth, or animation state.
- Register in-memory assets as base64 payloads instead of filesystem paths.
- Clean up one object or reset all RGP objects created in the terminal session.

## Basic Flow

1. Create `MobiusGraphicSettings`.
2. Create a `MobiusGraphic`.
3. Register the asset by path or payload.
4. Render `&MobiusGraphic` into a Ratatui `Rect`.
5. Mutate settings and call `update()` when runtime state changes.
6. Call `clear()` or `MobiusGraphic::clear_all()` during cleanup.

## Path-Based Example

```rust,no_run
use std::io;

use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
use ratatui_mobius::{MobiusGraphic, MobiusGraphicSettings, ObjectFormat};

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

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 24));
    (&graphic).render(Rect::new(10, 5, 24, 10), &mut buffer);

    graphic.settings_mut().rotation = [0.0, 90.0, 0.0];
    graphic.settings_mut().brightness = 1.2;
    graphic.update()?;

    Ok(())
}
```

## Payload Registration

Use payload registration when asset bytes are already in memory or when the
application cannot rely on a shared filesystem path with Mobius.

```rust,no_run
use std::io;

use ratatui_mobius::{MobiusGraphic, MobiusGraphicSettings, ObjectFormat};

fn main() -> io::Result<()> {
    let graphic = MobiusGraphic::new(
        MobiusGraphicSettings::new("live_draw.obj")
            .id(42)
            .format(ObjectFormat::Obj)
            .normalize(false),
    );

    let obj = b"v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    graphic.register_payload_with_name(obj, Some("triangle.obj"))?;

    Ok(())
}
```

Payloads are base64-encoded and split into 3072-character chunks before being
emitted. Mobius reconstructs the payload when it receives the final chunk.

## API Notes

### `ObjectFormat`

Supported formats:

| Variant | Wire value | Notes |
| :------ | :--------- | :---- |
| `ObjectFormat::Obj` | `obj` | Wavefront OBJ. |
| `ObjectFormat::Glb` | `glb` | Binary glTF. |
| `ObjectFormat::Stl` | `stl` | STL mesh. |

`MobiusGraphicSettings::new(path)` infers the format from the file extension.
Unknown extensions default to GLB; set `.format(...)` explicitly when needed.

### `MobiusGraphicSettings`

Important settings:

| Method | Effect |
| :----- | :----- |
| `.id(u32)` | Sets the RGP object id. |
| `.format(ObjectFormat)` | Sets the asset format. |
| `.normalize(bool)` | Controls OBJ normalization at registration time. |
| `.animate(bool)` | Enables or disables default object animation. |
| `.scale(f32)` | Sets uniform scale. |
| `.depth(f32)` | Sets depth offset. |
| `.color([u8; 3])` | Applies an RGB color/tint. |
| `.brightness(f32)` | Sets brightness multiplier. |
| `.offset([f32; 3])` | Applies translation relative to the anchor. |
| `.rotation([f32; 3])` | Applies X/Y/Z rotation in degrees. |
| `.scale3([f32; 3])` | Applies non-uniform scale. |

OBJ normalization is enabled by default. Disable it when authored coordinates
are already meaningful, for example generated geometry that intentionally uses
Mobius scene coordinates.

### Generated Sequences

`MobiusGraphic` exposes sequence-building methods for applications that want to
control output manually:

| Method | Description |
| :----- | :---------- |
| `register_sequence()` | Returns one path-based `r` sequence. |
| `register_payload_sequences(bytes)` | Returns chunked payload `r` sequences. |
| `place_sequence(area)` | Returns a `p` sequence for a Ratatui area. |
| `update_sequence()` | Returns a `u` sequence with current mutable settings. |
| `delete_sequence()` | Returns a `d` sequence for this object id. |
| `delete_all_sequence()` | Returns a `d` sequence without an id. |

The convenience methods `register()`, `register_payload()`, `update()`,
`clear()`, and `clear_all()` write the corresponding sequence(s) to stdout and
flush immediately.

## Rendering Behavior

`impl Widget for &MobiusGraphic` writes the placement sequence into the first
cell of the target `Rect`. Mobius then interprets the sequence and anchors the
object at the center of that rectangle.

The rectangle dimensions become the RGP `w` and `h` placement span. Empty areas
are ignored.

## Running Examples

Run examples inside Mobius to see the inline 3D objects:

```bash
cargo run --manifest-path widget/Cargo.toml --example big_rat
cargo run --manifest-path widget/Cargo.toml --example document
cargo run --manifest-path widget/Cargo.toml --example draw
cargo run --manifest-path widget/Cargo.toml --example rubiks_cube
cargo run --manifest-path widget/Cargo.toml --example mobius_chess
```

Examples:

- [`examples/big_rat.rs`](examples/big_rat.rs): minimal inline object demo.
- [`examples/document.rs`](examples/document.rs): document-style interface with
  embedded objects.
- [`examples/draw.rs`](examples/draw.rs): 2D drawing pane with live 3D preview.
- [`examples/rubiks_cube.rs`](examples/rubiks_cube.rs): interactive 3D Rubik's
  cube.
- [`examples/mobius_chess.rs`](examples/mobius_chess.rs): 3D Mobius strip chess
  board.

## Development

```bash
cargo fmt --manifest-path widget/Cargo.toml --all -- --check
cargo clippy --manifest-path widget/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path widget/Cargo.toml --all-targets
cargo check --manifest-path widget/Cargo.toml --examples
```

## License

Licensed under the [MIT License](../LICENSE).
