# Module `hyprland`

The hyprland module outputs a config file that you can include in your
hyprland config. To use it, add this line to your `hyprland.conf`:

```
source = ~/.local/share/niji/hyprland/theme.conf
```

Also, make sure that nothing in your config overrides any settings set by
niji (unless that is explicitly what you want). You can do this either by
adding the line to the bottom of your `hyprland.conf`, or by going through
it deleting anything you think might interfere.

## Configuration options

The following global configuration options are supported by this module:

- `cursor_theme`
- `cursor_size`

See [the main documentation](#module-configuration) for more information.

Additional module-specific configuration options are:

| Name            | Type                                   | Description                                                       |
| --------------- | -------------------------------------- | ----------------------------------------------------------------- |
| `focused_color` | "background" or "surface" or "primary" | Which theme color to use for focused windows (default: "surface") |

## See also

- The [hyprpaper module](../hyprpaper/README.md)
