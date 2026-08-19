# Nix Packaging

This flake provides a Nix package for [Mobius](https://github.com/GiorgiKavtaradze-prog/mobius), a GPU-rendered terminal emulator with inline 3D graphics.

## Supported Systems

- `x86_64-linux`
- `aarch64-linux`
- `x86_64-darwin`
- `aarch64-darwin`

## Quick Start

### Direct usage

```bash
# Run directly
nix run github:GiorgiKavtaradze-prog/mobius

# Install to profile
nix profile install github:GiorgiKavtaradze-prog/mobius
```

### As a flake input

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mobius.url = "github:GiorgiKavtaradze-prog/mobius";
  };

  outputs = { nixpkgs, mobius, ... }: {
    # Use in your configuration
  };
}
```

## NixOS System Configuration

Add mobius to your system packages with optional declarative configuration:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mobius.url = "github:GiorgiKavtaradze-prog/mobius";
  };

  outputs = { nixpkgs, mobius, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        mobius.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
```

```nix
# configuration.nix
{
  programs.mobius = {
    enable = true;
    settings = {
      window = {
        opacity = 0.9;
        width = 1200;
        height = 800;
      };
      shell = {
        program = "zsh";
      };
      font = {
        family = "JetBrains Mono";
        size = 14;
      };
    };
  };
}
```

This will:

- Install the Mobius package
- Write configuration to `/etc/mobius/mobius.toml` (only when `settings` is non-empty)
- Wrap the binary to use `--config-file /etc/mobius/mobius.toml` (only when `settings` is non-empty)

### GPU Backend Selection

On systems with multiple GPUs or where the default Vulkan device creation fails
(e.g. NVIDIA 580.x drivers reporting unsupported features), set `gpuBackend` and
`gpuAdapter` to control wgpu device selection:

```nix
{
  programs.mobius = {
    enable = true;
    gpuBackend = "vulkan";    # or "gl" / "gles"
    gpuAdapter = "RTX 3060";  # substring match against adapter name
  };
}
```

When set, the NixOS module wraps the binary with `WGPU_BACKEND` and
`WGPU_ADAPTER_NAME` environment variables. When both `settings` and GPU options
are set, a single wrapper applies all flags.

## Home Manager Configuration

For user-level configuration without root:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    mobius.url = "github:GiorgiKavtaradze-prog/mobius";
  };

  outputs = { nixpkgs, home-manager, mobius, ... }: {
    homeConfigurations.myuser = home-manager.lib.homeManagerConfiguration {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
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
{
  programs.mobius = {
    enable = true;
    settings = {
      window = {
        opacity = 0.85;
      };
      shell = {
        program = "fish";
      };
      theme = {
        foreground = "#c0caf5";
        background = "#1a1b26";
      };
    };
  };
}
```

This will:

- Install the Mobius package to your user profile
- Write configuration to `$XDG_CONFIG_HOME/mobius/mobius.toml` (typically `~/.config/mobius/mobius.toml`) (only when `settings` is non-empty)
- Set `WGPU_BACKEND` and `WGPU_ADAPTER_NAME` in the user session when GPU options are configured
- Mobius discovers this path automatically

### GPU Backend Selection (Home Manager)

Same options as NixOS, but applied via `home.sessionVariables` instead of a
binary wrapper:

```nix
{
  programs.mobius = {
    enable = true;
    gpuBackend = "vulkan";
    gpuAdapter = "RTX 3060";
  };
}
```

## Module Options

Both `nixosModules.default` and `homeManagerModules.default` expose:

| Option                      | Type         | Default                        | Description                                                                              |
| --------------------------- | ------------ | ------------------------------ | ---------------------------------------------------------------------------------------- |
| `programs.mobius.enable`     | bool         | `false`                        | Enable Mobius installation                                                                |
| `programs.mobius.package`    | package      | `self.packages.<system>.mobius` | The Mobius package to use                                                                 |
| `programs.mobius.settings`   | attrset      | `{}`                           | Configuration written to `mobius.toml`                                                    |
| `programs.mobius.gpuBackend` | null or enum | `null`                         | Force wgpu backend: `"vulkan"`, `"gl"`, or `"gles"`. null = auto-detect                  |
| `programs.mobius.gpuAdapter` | null or str  | `null`                         | Substring match to select a specific GPU adapter (e.g. `"RTX 3060"`). null = auto-detect |

## Package Architecture

```
flake.nix          — Orchestration, modules, devShell
nix/default.nix    — Standalone package (upstreamable to nixpkgs)
```

The package definition in `nix/default.nix` is designed to be upstreamed to nixpkgs as `pkgs/by-name/ra/mobius/package.nix`. It takes only standard nixpkgs arguments — no flake-specific constructs.

## Development

```bash
# Enter dev shell
nix develop

# Build package
nix build

# Run checks (build + tests)
nix flake check
```

## Maintainer

- Giorgi Kavtardaze <giorgikavtaradze@example.com>
