# Pipemenu

Gnome (`GTK4` + `libadwaita`) compatible `dmenu` alternative.

It is **not** application launcher. It accepts input from stdin, allows user to select desired entry, and returns that entry on stdout.

## Usage

```sh
ls | pipemenu | xargs xdg-open
```

![alt text](screenshot.png)

### Help & options

```sh
pipemenu -h
```

## Installation

### Prebuilt binary

Navigate to [Releases](https://github.com/soanvig/pipemenu/releases) and download a released binary (`pipemenu_vx.x.x_linux-64bit`)

After that you need to:

1. rename the binary to `pipemenu`
2. make it executable: `chmod +x pipemenu`
3. move it to a location included in your `$PATH` environment variable (for example: `/usr/local/bin` [requires sudo]. To check available paths use your terminal: `echo $PATH`)

### Rust/Cargo

Install dependencies listed in [Development section](#development) (`cargo` builds package on your system), and then:

```sh
cargo install pipemenu
```

## Roadmap

See [Issues](https://github.com/soanvig/pipemenu/labels/enhancement)

## Development

`pipemenu` uses `gtk4-rs` therefore [GTK building instructions](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html) and [Adwaita building instructions](https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html) apply.

Currently `gtk4` and `libadwaita` development dependencies are required with versions matching what's defined in `Cargo.toml`.

If using NixOS to build proper binary to be run on other systems, the binary has to be patched: `patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 pipemenu`
