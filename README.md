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

Technically, all other configuration options are optional, but it is a good idea to set
a few other global options, like this:

```toml
modules = ["hyprland", "waybar"]

[global]
font_family = "Fira Sans"
cursor_theme = "Adwaita"
cursor_size = 22
```

You can list available themes using `niji theme list`, and preview them using `niji theme show <name>`.
If you've picked one, apply it using `niji theme set <name>`.

## Configuration

The niji configuration file, `~/.config/niji/config.toml` consists of two main parts; the program configuration and the module configuration.
Any top-level configuration value that is not part of a section is considered to be program configuration.

### Program configuration

You can use the following options to configure niji.

- `modules`: A list of the names of modules you want to activate. See the [list of modules](#built-in-modules) for available names.
- `disable_reloads`: A list of the names of modules that you want to apply the theme to, but not automatically reload. This is
  useful in nonstandard use cases where the side effects of reloading may not be desired.

### Module configuration

Anything in `config.toml` that is part of a section is considered to be module configuration. Config values in the special
`[global]` section apply to all modules; for module-specific configuration use a section with the name of the module, like
`[waybar]` for example.

Common global config options include:

| Name           | Description                                                                                                                                      |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `font_family`  | The UI font to be used when applicable. It is recommended to set this to something, since some modules don't/can't provide a reasonable default. |
| `font_scale`   | A factor used to scale the font size. The default is `1.0`.                                                                                      |
| `cursor_theme` | The cursor theme to use.                                                                                                                         |
| `cursor_size`  | The cursor size to use.                                                                                                                          |
| `wallpaper`    | Used to configure setting wallpapers per theme. See [Setting Wallpapers per Theme](#setting-wallpapers-per-theme).                               |

Any of these config options can be applied per module as well. For module specific settings, see the [list of modules](#built-in-modules), and their respective
documentation.

### Example

A non-trivial example for `config.toml` might look like this:

```toml
modules = ["sway", "hyprland", "hyprpaper", "waybar", "kitty", "swaylock"]
disable_reloads = ["sway"]

[global]
font_family = "Fira Sans"
font_scale = 1.2
cursor_theme = "Bibata-Modern-Classic"
cursor_size = 22

[waybar]
icon_font = "Material Design Icons"
```

### Setting Wallpapers per Theme

Some modules support the global `wallpaper` configuration option, which allows you to configure a wallpaper to use per theme.
If you have one of those modules enabled, you can add a `[global.wallpaper]` section to you `config.toml`, and set themes like this:

```toml
[global.wallpaper]
default = "./wallpapers/wp1.png"
tokyonight = "./wallpapers/wp2.png"
dracula = "./wallpapers/wp3.png"
```

As you might expect, niji will use the wallpaper set for the name of the current theme if it exists, and otherwise the `default`
wallpaper as a fallback.

## Built-in Modules

Niji comes with the following modules built-in:

- [`gtk`](modules/gtk/README.md) Theming gtk3 and gtk4 applications.
- [`hyprland`](modules/hyprland/README.md) Theming Hyprland window decorations.
- [`hyprpaper`](modules/hyprland/README.md) Setting wallpapers with hyprpaper. See [Setting Wallpapers per Theme](#setting-wallpapers-per-theme).
