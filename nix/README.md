# Nix Packaging

This directory contains the standalone Nix package definition for Mobius. The
repository root also provides a flake with packages, a development shell, an
overlay, NixOS and Home Manager modules, and checks.

## Supported Systems

- `x86_64-linux`
- `aarch64-linux`
- `x86_64-darwin`
- `aarch64-darwin`

## Flake Outputs

| Output | Purpose |
| :----- | :------ |
| `packages.<system>.mobius` | Mobius package. |
| `packages.<system>.default` | Alias for the Mobius package. |
| `devShells.<system>.default` | Rust development shell with Cargo, clippy, rustfmt, and rust-analyzer. |
| `checks.<system>.mobius` | Build and test check through the package build. |
| `formatter.<system>` | `nixfmt-rfc-style`. |
| `overlays.default` | Adds `pkgs.mobius`. |
| `nixosModules.default` | Declarative NixOS module. |
| `homeManagerModules.default` | Declarative Home Manager module. |

## Quick Start

```bash
# Run directly
nix run github:GiorgiKavtaradze-prog/mobius

# Build the package
nix build github:GiorgiKavtaradze-prog/mobius

# Install into the current user profile
nix profile install github:GiorgiKavtaradze-prog/mobius

# Enter the development shell from a checkout
nix develop
```

## NixOS Module

The module's default package is `pkgs.mobius`, so add the Mobius overlay or set
`programs.mobius.package` explicitly.

### Flake Example

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mobius.url = "github:GiorgiKavtaradze-prog/mobius";
  };

  outputs = { nixpkgs, mobius, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ({ ... }: {
          nixpkgs.overlays = [ mobius.overlays.default ];
        })
        mobius.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
```

```nix
# configuration.nix
{ pkgs, ... }:

{
  programs.mobius = {
    enable = true;
    defaultShell = pkgs.zsh;
    gpuBackend = "vulkan";
    gpuAdapter = "RTX 3060";
    settings = {
      window = {
        width = 1200;
        height = 800;
        opacity = 0.9;
      };
      font = {
        family = "JetBrains Mono";
        size = 14;
      };
      terminal = {
        scrollback = 5000;
      };
    };
  };
}
```

When enabled, the NixOS module:

- Installs Mobius into `environment.systemPackages`.
- Writes `/etc/mobius/mobius.toml` when `settings` is non-empty.
- Wraps the binary with `--config-file /etc/mobius/mobius.toml` when settings
  are written.
- Sets `WGPU_BACKEND` and `WGPU_ADAPTER_NAME` in the wrapper when GPU options
  are configured.
- Sets a default `SHELL` in the wrapper when `defaultShell` is configured.

## Home Manager Module

### Flake Example

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    mobius.url = "github:GiorgiKavtaradze-prog/mobius";
  };

  outputs = { nixpkgs, home-manager, mobius, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ mobius.overlays.default ];
      };
    in
    {
      homeConfigurations.myuser = home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        modules = [
          mobius.homeManagerModules.default
          ./home.nix
        ];
      };
    };
}
```

```nix
# home.nix
{ pkgs, ... }:

{
  programs.mobius = {
    enable = true;
    defaultShell = pkgs.fish;
    gpuBackend = "vulkan";
    settings = {
      window.opacity = 0.85;
      font = {
        family = "JetBrains Mono";
        size = 13;
      };
      theme = {
        foreground = "#c0caf5";
        background = "#1a1b26";
        cursor = "#7aa2f7";
      };
    };
  };
}
```

When enabled, the Home Manager module:

- Installs Mobius into `home.packages`.
- Writes `$XDG_CONFIG_HOME/mobius/mobius.toml` when `settings` is non-empty.
- Sets `WGPU_BACKEND` and `WGPU_ADAPTER_NAME` in `home.sessionVariables` when
  GPU options are configured.
- Sets `SHELL` in `home.sessionVariables` when `defaultShell` is configured.

Mobius discovers the Home Manager config path automatically.

## Module Options

Both modules expose the same options:

| Option | Type | Default | Description |
| :----- | :--- | :------ | :---------- |
| `programs.mobius.enable` | bool | `false` | Enables Mobius installation and configuration. |
| `programs.mobius.package` | package | `pkgs.mobius` | Package to install. Requires the overlay unless set explicitly. |
| `programs.mobius.settings` | TOML-compatible attrset | `{}` | Configuration written to `mobius.toml`. |
| `programs.mobius.gpuBackend` | null or enum | `null` | Forces the wgpu backend. Linux values: `"vulkan"`, `"gl"`, `"gles"`. Darwin values: `"metal"`, `"gl"`, `"gles"`. |
| `programs.mobius.gpuAdapter` | null or string | `null` | Substring match for selecting a GPU adapter by name. |
| `programs.mobius.defaultShell` | null or package | `null` | Shell package used when Mobius is launched without `-e` / `--command`. |

## GPU Backend Selection

Use `gpuBackend` and `gpuAdapter` when the default wgpu adapter selection is not
appropriate, for example on multi-GPU systems, incompatible Vulkan ICD setups,
remote desktop sessions, or headless/VNC environments.

Linux example:

```nix
{
  programs.mobius = {
    enable = true;
    gpuBackend = "vulkan";    # "vulkan", "gl", or "gles"
    gpuAdapter = "RTX 3060";  # substring match
  };
}
```

Darwin example:

```nix
{
  programs.mobius = {
    enable = true;
    gpuBackend = "metal"; # "metal", "gl", or "gles"
  };
}
```

## Package Architecture

```text
flake.nix          # Flake outputs, overlay, modules, checks, and dev shell
nix/default.nix    # Standalone package definition
```

The package in `nix/default.nix` is structured to be upstreamable to nixpkgs. It
takes standard nixpkgs arguments plus `craneLib`, uses `craneLib.buildDepsOnly`
for dependency caching, and performs the final package build with the full
source tree so assets and configuration can be installed.

Installed package contents include:

- The wrapped `mobius` binary.
- Bundled object assets under `$out/share/mobius/objects`.
- The default config at `$out/share/mobius/mobius.toml`.
- A desktop entry and icon on platforms that support them.

## Development Commands

```bash
# Format Nix files
nix fmt

# Enter dev shell
nix develop

# Build default package
nix build

# Run flake checks
nix flake check
```

## Release Maintenance

Before cutting a release, keep package metadata synchronized:

- `Cargo.toml` version.
- `widget/Cargo.toml` version, when the widget crate changes.
- `nix/default.nix` package version.
- `CHANGELOG.md` release entry.
