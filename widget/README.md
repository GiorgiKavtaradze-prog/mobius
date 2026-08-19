# `ratatui-mobius` 🐀

A [`ratatui`](https://github.com/ratatui/ratatui) widget for placing
inline 3D objects in [Mobius](https://github.com/GiorgiKavtaradze-prog/mobius) through the
[Mobius Graphics Protocol](https://github.com/GiorgiKavtaradze-prog/mobius/blob/main/protocols/graphics.md).

## Example

```rust,no_run
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

    // Register an existing asset by path.
    graphic.register()?;

    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
    (&graphic).render(Rect::new(10, 5, 24, 10), &mut buf);

    // Update transform or style in-place after placement.
    graphic.settings_mut().rotation = [0.0, 90.0, 0.0];
    graphic.settings_mut().brightness = 1.2;
    graphic.update()?;

    Ok(())
}
```

The widget emits RGP APC sequences into the target buffer cell. Mobius then
resolves the asset and renders it as an inline 3D object anchored to that
terminal region.

## Payload Registration

If the object data is already in memory, register it directly instead of
referring to a file path:

```rust,no_run
use std::io;

use ratatui_mobius::{ObjectFormat, MobiusGraphic, MobiusGraphicSettings};

fn main() -> io::Result<()> {
    let graphic = MobiusGraphic::new(
        MobiusGraphicSettings::new("live_draw.obj")
            .id(42)
            .format(ObjectFormat::Obj),
    );

    let obj = b"v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    graphic.register_payload(obj)?;
    Ok(())
}
```

## Examples

- [`examples/big_rat.rs`](https://github.com/GiorgiKavtaradze-prog/mobius/tree/main/widget/examples/big_rat.rs): minimal inline object demo
- [`examples/document.rs`](https://github.com/GiorgiKavtaradze-prog/mobius/tree/main/widget/examples/document.rs): TempleOS-inspired editor with embedded objects
- [`examples/draw.rs`](https://github.com/GiorgiKavtaradze-prog/mobius/tree/main/widget/examples/draw.rs): 2D drawing pane with live 3D preview
- [`examples/rubiks_cube.rs`](https://github.com/GiorgiKavtaradze-prog/mobius/tree/main/widget/examples/rubiks_cube.rs): interactive 3D Rubik's cube
- [`examples/mobius_chess.rs`](https://github.com/GiorgiKavtaradze-prog/mobius/tree/main/widget/examples/mobius_chess.rs): 3D mobius strip chess board.

## License

Licensed under [The MIT License](../LICENSE).
