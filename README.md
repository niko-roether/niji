# Niji

Niji is an extensible theming framework that brings uniform, responsive and comfortable theming
to the tinkerer's desktop. It currently comes with builtin support for gtk apps, sway, hyprland,
kitty, and others, but it also allows you to easily add custom modules for anything you desire.
Happy hacking!

## Preview

https://github.com/nicholas-roether/niji/assets/35772525/1e87d9ac-325b-409e-a1f9-ab95e01cfff8

This preview shows niji theming hyprland, hyprpaper, waybar, kitty, and nautilus. It is also theming
some other applications that aren't visible here, as indicated by the terminal output.

## Installing

Currently, the only way of installing is to build from source. This is due to change when version 0.1 is published.

1. Clone the repository: `git clone https://github.com/nicholas-roether/niji`
2. Enter the repository: `cd niji`
3. Install using cargo: `cargo install --path ./niji`

## Getting started

The first step is picking the modules you'd like to use. A basic configuration for `~/.config/niji/config.toml`
might look like this:

```toml
modules = ["hyprland", "waybar"]
```

technically, all other configuration options are optional, but it is a good idea to set
a few other global options, like this

```toml
modules ["hyprland", "wayland"]

[global]
font_family = "Fira Sans"
cursor_theme = "Adwaita"
cursor_size = 22
```

You can list available themes using `niji theme list`, and preview them using `niji theme show <name>`.
If you've picked one, apply it using `niji theme set <name>`.
