# GTK module

The GTK module installs a gtk3/gtk4 theme called "niji", and enables it
automatically if reloading is not disabled for it. It modifies the
amazing [Colloid](https://github.com/vinceliuice/Colloid-gtk-theme) theme by
vinceliuise.

In order for this module to work with libadwaita apps like Nautilus, you'll
need a patched version of libadwaita, like
[libadwaita-without-adwaita-git (aur)](https://aur.archlinux.org/packages/libadwaita-without-adwaita-git).

## Configuration options

The following global configuration options are relevant:

| Name           | Description                                                          |
| -------------- | -------------------------------------------------------------------- |
| `cursor_theme` | Which cursor theme to apply. Only works if reloading is enabled.     |
| `cursor_size`  | Which cursor size to apply. Only works if reloading is enabled.      |
| `font_family`  | Which font family to use for UI. Only works if reloading is enabled. |
| `font_scale`   | The font scaling factor to use. Only works if reloading is enabled.  |

Additional module-specific configuration options are:
