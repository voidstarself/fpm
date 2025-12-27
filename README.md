# fpm — Fabric Package Manager

`fpm` is a **CLI package manager for Fabric Minecraft mods**, powered by the Modrinth API.

It focuses on:
- reproducible installs
- explicit dependency resolution
- simple, scriptable workflows

No launcher integration. No GUI. Just files and commands.


## Status

**Early development (v0.1.0 target)**

The CLI structure is in place, but most commands are not yet implemented.
Expect breaking changes until `1.0.0`.


## Features (Planned for MVP)

- Initialize a Fabric project
- Search Fabric mods on Modrinth
- Add and remove mods
- Resolve required dependencies
- Generate a lockfile for reproducible installs

## Non-Goals

- CurseForge support
- Modpacks
- GUI or TUI
- Launcher integration (Prism, MultiMC, etc.)
- Fabric Loader installation


## Installation

For now, build from source:

```bash
git clone https://github.com/<your-username>/fpm
cd fpm
cargo build --release
```

The binary will be at:

```text
target/release/fpm
```


## Usage

```bash
fpm init 1.20.1
fpm search sodium
fpm add sodium
fpm remove sodium
fpm list
fpm resolve
```

> [!NOTE]
> Most commands currently abort with `todo!()` while under development.


## Project Layout

```text
fabric.toml     # user intent
fabric.lock    # resolved dependency graph
mods/           # installed mod JARs (not committed)
```
