# Module `gtk`

The GTK module installs a gtk3/gtk4 theme called "niji", and enables it
automatically if reloading is not disabled for it. It modifies the
amazing [Colloid](https://github.com/vinceliuice/Colloid-gtk-theme) theme by
vinceliuise.

In order for this module to work with libadwaita apps like Nautilus, you'll
need a patched version of libadwaita, like
[libadwaita-without-adwaita-git (aur)](https://aur.archlinux.org/packages/libadwaita-without-adwaita-git).

## Configuration options

The following global configuration options are supported by this module:

- `cursor_theme`
- `cursor_size`
- `font_family`
- `font_scale`

See [the main documentation](#module-configuration) for more information. Note that these options only work
if reloads are enabled for this module.

Additional module-specific configuration options are:

| Name            | Type              | Description                                                                        |
| --------------- | ----------------- | ---------------------------------------------------------------------------------- |
| `compact`       | boolean           | Use the compact variant of the theme, which uses smaller paddings (default: false) |
| `rimless`       | boolean           | Disable window outline (default: false)                                            |
| `window_button` | "mac" or "normal" | Which style of window buttons to use (default: mac)                                |
