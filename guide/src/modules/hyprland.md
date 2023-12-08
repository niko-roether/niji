# Module `hyprland`

The `hyprland` module allows you to theme the window decoration of the
[Hyprland](https://github.com/hyprwm/Hyprland) wayland compositor.

The configuration produced by this module is intentionally minimal, and does not interfere
with your existing hyprland config.

See also the [`hyprpaper` module](./hyprpaper.md).

## Activating

To activate the module, add it to your `config.toml`:

```toml
modules = ["hyprland"]
```

Niji will now output a hyprland configuration file to `~/.local/share/niji/hyprland/theme.conf`.
To enable it, add the following line to the bottom of your `hyprland.conf`:

```
source = ~/.local/share/niji/hyprland/theme.conf
```

You can, of course, override as much of the generated configuration as you like, simply
by adding configuration after the source statement.

## Configuration

The following global configuration options are relevant to this module:

- `cursor_theme`
- `cursor_size`

See [Configuration](../configuration.md) for a detailed explanation.

Additionally, these module-specific configuration options can be added to `config.toml` (shown
here with their default values):

```toml
[hyprland]

# Can be either "background", "surface", "primary" or "secondary".
# This value determines which theme color is used for focused window borders.
focused_color = "surface"
```
