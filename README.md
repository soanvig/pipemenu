# Pipemenu

`GTK4` and `libadwaita` compatible `dmenu` alternative.

It is **not** application launcher. It accepts input from stdin, allows user to select desired entry, and returns that entry on stdout.

## Usage

```sh
ls | pipemenu | xargs xdg-open
```

![alt text](screenshot.png)

## Roadmap

See [Issues](https://github.com/soanvig/pipemenu/labels/enhancement)