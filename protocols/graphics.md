# Mobius Graphics Protocol

Mobius Graphics Protocol (RGP) is a terminal-native protocol for registering
3D assets and rendering them as inline objects anchored to terminal cells.

RGP is designed for programs running inside Mobius. A CLI tool, TUI, remote
session, or Ratatui application can emit escape sequences that create 3D
objects without using an external overlay window.

## Design Goals

- Keep graphics attached to terminal cell positions.
- Support 3D assets directly, starting with OBJ, GLB, and STL.
- Work over ordinary terminal output, including SSH sessions.
- Allow both filesystem paths and embedded payloads.
- Preserve backward compatibility as optional fields are added.
- Leave room for future interaction, such as object clicks, macros, and object
  state updates.

## Transport

RGP uses APC (Application Program Command):

```text
ESC _ mobius;g;<verb>[;<key=value>...] ESC \
```

Mobius also accepts the C1 ST terminator (`0x9c`) in place of `ESC \`.

The prefix fields are:

| Field | Meaning |
| :---- | :------ |
| `mobius` | Protocol namespace. |
| `g` | Graphics protocol family. |
| `<verb>` | Operation selector. |

All fields after the verb are separated by semicolons. Most fields use
`key=value`. Payload registration is the exception: after header fields, the
final semicolon-separated token is treated as the base64 payload chunk.

Values should not contain unescaped semicolons. Paths and names should be
simple UTF-8 strings suitable for terminal transport.

## Version and Capabilities

### Support Query

Client sends:

```text
ESC _ mobius;g;s ESC \
```

Mobius replies:

```text
ESC _ mobius;g;s;v=1;fmt=obj|glb|stl;path=1;payload=1;chunk=1;anim=1;depth=1;color=1;brightness=1;transform=1;update=1;normalize=1;camera=1 ESC \
```

Capability fields:

| Field | Meaning |
| :---- | :------ |
| `v=1` | Protocol version 1. |
| `fmt=obj|glb|stl` | Supported asset formats. |
| `path=1` | Path-based registration is supported. |
| `payload=1` | Embedded base64 payload registration is supported. |
| `chunk=1` | Payloads may be split across multiple register commands. |
| `anim=1` | Placement animation flag is supported. |
| `depth=1` | Placement depth is supported. |
| `color=1` | RGB color/tint is supported. |
| `brightness=1` | Brightness multiplier is supported. |
| `transform=1` | Translation, rotation, and non-uniform scale are supported. |
| `update=1` | In-place object updates are supported. |
| `normalize=1` | OBJ normalization option is supported. |
| `camera=1` | Camera preset updates are supported. |

If no reply arrives, the terminal should be treated as not supporting RGP.

## Object Model

An RGP object has:

- A numeric object id chosen by the emitting application.
- A registered asset source.
- An optional placement in terminal cell space.
- Styling and transform state.
- Optional metadata reserved for future interaction.

Registration and placement are separate operations. A client may register an
asset once and place, update, or delete it later by id.

## Verbs

| Verb | Name | Description |
| :--: | :--- | :---------- |
| `s` | Support | Query protocol support. |
| `r` | Register | Register an asset by path or embedded payload. |
| `p` | Place | Place a registered asset in terminal cell space. |
| `u` | Update | Partially update an existing placement. |
| `d` | Delete | Delete one object or every RGP object. |
| `c` | Camera | Partially update and optionally activate a camera preset. |

Unknown verbs are ignored.

## Register Object Asset

The `r` verb registers a 3D asset under an object id.

### Path Registration

```text
ESC _ mobius;g;r;id=42;fmt=obj;path=CairoSpinyMouse.obj ESC \
```

Required fields:

| Field | Description |
| :---- | :---------- |
| `id` | Application-chosen object id. |
| `fmt` | Asset format: `obj`, `glb`, or `stl`. |
| `path` | Asset path known to Mobius. |

Optional fields:

| Field | Default | Description |
| :---- | :------ | :---------- |
| `normalize` | `1` | OBJ normalization flag. `1` recenters each OBJ mesh around its bounding-box center and scales by the largest axis. `0` preserves authored OBJ coordinates. |

### Payload Registration

Payload registration embeds asset bytes directly in the terminal stream. This is
useful when the application and Mobius do not share a filesystem, such as over
SSH.

Single-chunk payload:

```text
ESC _ mobius;g;r;id=42;fmt=obj;source=payload;more=0;name=rat.obj;<base64-payload> ESC \
```

Chunked payload:

```text
ESC _ mobius;g;r;id=42;fmt=glb;source=payload;more=1;name=scene.glb;<chunk-1> ESC \
ESC _ mobius;g;r;id=42;fmt=glb;source=payload;more=1;<chunk-2> ESC \
ESC _ mobius;g;r;id=42;fmt=glb;source=payload;more=0;<chunk-n> ESC \
```

Required fields:

| Field | Description |
| :---- | :---------- |
| `id` | Application-chosen object id. |
| `fmt` | Asset format: `obj`, `glb`, or `stl`. |
| `source=payload` | Selects embedded payload registration. |
| `more` | `1` when more chunks follow, `0` for the final chunk. |

Optional fields:

| Field | Default | Description |
| :---- | :------ | :---------- |
| `name` | Format-specific fallback | Source name used for diagnostics and temporary asset naming. |
| `normalize` | `1` | OBJ normalization flag. Include on the first chunk when needed. |

Mobius accumulates chunks for the same `id` until it receives `more=0`. The
object becomes available for placement after the final chunk is decoded and
loaded.

## Place Object

The `p` verb places a registered object in terminal cell space.

```text
ESC _ mobius;g;p;id=42;row=12;col=8;w=4;h=2;animate=1;scale=1.0;depth=2.5;color=ff8844;brightness=1.0;px=0;py=0;pz=0;rx=0;ry=45;rz=0;sx=1;sy=1;sz=1 ESC \
```

Required fields:

| Field | Description |
| :---- | :---------- |
| `id` | Registered object id. |
| `row` | Anchor row at the center of the placement. |
| `col` | Anchor column at the center of the placement. |
| `w` | Placement width in terminal cells. |
| `h` | Placement height in terminal cells. |

Optional fields:

| Field | Default | Description |
| :---- | :------ | :---------- |
| `animate` | `0` | Enables default animation when `1` or `true`. |
| `scale` | `1.0` | Uniform scale multiplier. |
| `depth` | `0.0` | Z offset/extrusion depth. |
| `color` | none | RGB color as `RRGGBB`. `tint` is accepted as an alias. |
| `brightness` | `1.0` | Brightness multiplier. |
| `px`, `py`, `pz` | `0.0` | Translation offset relative to the anchor. |
| `rx`, `ry`, `rz` | `0.0` | Rotation in degrees around X, Y, and Z. |
| `sx`, `sy`, `sz` | `1.0` | Non-uniform scale multipliers. |

## Update Object

The `u` verb partially updates an existing placement without changing the
registration or anchor.

```text
ESC _ mobius;g;u;id=42;ry=120;px=0.25;animate=0 ESC \
```

Required fields:

| Field | Description |
| :---- | :---------- |
| `id` | Object id to update. |

Optional fields mirror the mutable placement fields:

- `animate`
- `scale`
- `depth`
- `color` or `tint`
- `brightness`
- `px`, `py`, `pz`
- `rx`, `ry`, `rz`
- `sx`, `sy`, `sz`

Omitted fields keep their previous values.

## Delete Object

Delete one object:

```text
ESC _ mobius;g;d;id=42 ESC \
```

Delete all RGP objects:

```text
ESC _ mobius;g;d ESC \
```

Use delete-all only for full-scene reset flows, demos, and cleanup paths where
removing objects created by other processes is acceptable.

## Camera Control

The `c` verb partially updates one of ten persistent camera presets. Slot ids
are decimal numbers from `0` through `9`.

```text
ESC _ mobius;g;c;id=0;set=0;type=Ortho;px=0.25;scale=1.0 ESC \
ESC _ mobius;g;c;id=1;set=1;type=Persp;fov=50;rx=10;ry=20 ESC \
```

Required fields:

| Field | Description |
| :---- | :---------- |
| `id` | Camera preset slot, `0` through `9`. |

Optional fields:

| Field | Default | Description |
| :---- | :------ | :---------- |
| `set` | `0` | Activates the slot after applying this update when `1` or `true`. |
| `type` | unchanged | `Flat`, `Ortho`, `Persp`, or `Mobius`. |
| `scale` | unchanged | Orthographic projection scale. Values below `0.01` are ignored. |
| `fov` | unchanged | Perspective vertical FOV in degrees. Valid range is approximately `2.87` through `177.13`. |
| `px`, `py`, `pz` | unchanged | Camera translation/pan/dolly components. |
| `rx`, `ry`, `rz` | unchanged | Pitch, yaw, and roll in degrees. |

Projection values are stored independently. A command may set both `scale` and
`fov` regardless of the current or requested `type`; changing `type` does not
reinterpret the other projection value.

Camera validation rules:

- Invalid `id`, `set`, `type`, value-less fields, non-numeric fields, `NaN`,
  and infinities cause the whole camera command to be ignored.
- Out-of-range `scale` or `fov` values are ignored for that projection value,
  but other valid fields in the same command still apply.
- `set=1` still activates the slot when the command is otherwise valid, even if
  an out-of-range projection value was ignored.

## Example Session

Register an object by path:

```text
ESC _ mobius;g;r;id=7;fmt=obj;path=CairoSpinyMouse.obj ESC \
```

Place it at row 5, column 10, spanning 3 by 2 cells:

```text
ESC _ mobius;g;p;id=7;row=5;col=10;w=3;h=2;animate=1;scale=1.0;depth=1.5;color=7fd0ff;brightness=1.0;ry=30 ESC \
```

Rotate it:

```text
ESC _ mobius;g;u;id=7;ry=180 ESC \
```

Switch to perspective mode with a 50-degree FOV:

```text
ESC _ mobius;g;c;id=0;set=1;type=Persp;fov=50;rx=10;ry=20 ESC \
```

Return to flat mode:

```text
ESC _ mobius;g;c;id=0;set=1;type=Flat ESC \
```

Delete the object:

```text
ESC _ mobius;g;d;id=7 ESC \
```

## Compatibility Guidance

For robust clients:

- Send a support query before emitting RGP-specific content.
- Use conservative object ids scoped to your application.
- Prefer payload registration for remote sessions where paths may not exist on
  the Mobius host.
- Chunk large payloads. The `ratatui-mobius` helper uses 3072 base64 characters
  per chunk.
- Keep commands backward-compatible by treating unknown reply fields as optional
  capabilities.

## References

- [TempleOS](https://templeos.org)
- [DolDoc](https://tinkeros.github.io/WbTempleOS/Doc/DolDocOverview.html)
- [Glyph Protocol](https://rapha.land/introducing-glyph-protocol-for-terminals/)
- [APC control](https://en.wikipedia.org/wiki/C0_and_C1_control_codes#C1_controls)
