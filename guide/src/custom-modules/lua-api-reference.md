# Lua API Reference

The niji api resides in the global `niji` namespace. There is no need to import it.
The niji namespace contains several sub-namespaces and classes for different purposes, which are listed in
this document.

For more complicated or nonstandard use cases, you can always use the [Lua standard library](https://www.lua.org/manual/5.3/)
which is fully supported. If one of the functions in the niji api fits what you want to do however,
you should always prefer using the niji api, as it provides better integration and safety features.

Contents:

- [Class `niji.Color`](#class-nijicolor)
- [Namespace `niji.console`](#namepsace-nijiconsole)
- [Namespace `niji.fs`](#namepsace-nijifs)
- [Namespace `niji.mod`](#namepsace-nijimod)

## Class `niji.Color`

The class `niji.Color` represents an RGBA color. it can be used to perform certain manipulations
on colors. All color manipulations use the Oklab perceptual color space, so while some results
may appear unexpected through an RGB lens, they should look good.

All functions that accept colors also accept strings of the format `"#RRGGBB"` and `"#RRGGBBAA"`.

### Static `niji.Color:new(color_string)`

Constructs a new `niji.Color` object.

- `color_string`: A string representing the desired color (`string`)
- returns: The resulting color (`niji.Color`)

```lua
local my_color = niji.Color:new("#ab38a3ff")

-- Prints "#ab3aa3ff"
niji.console.debug(my_color)
```

### Static `niji.Color:blend(color_1, color_2, t)`

Interpolates between two colors.

- `color_1`: The first of the two colors to interpolate between (`string` or `niji.Color`)
- `color_2`: The second of the two colors to interpolate between (`string` or `niji.Color`)
- `t`: A number between 0 and 1 that controls the interpolation (`float`)
- returns: The resulting color (`niji.Color`)

```lua
local my_color = niji.Color:blend("#ff0000", "#00ff00", 0.3)

-- Prints "#ff6300ff"
niji.console.debug(my_color)

```

### Static `niji.Color:mix(color_1, color_2)`

Mixes two colors together evenly. Equivalent to calling `niji.Color:blend` with a `t` of 0.5.

- `color_1`: The first of the two colors to mix together (`string` or `niji.Color`)
- `color_2`: The second of the two colors to mix together (`string` or `niji.Color`)

```lua
local my_color = niji.Color:mix("#ff0000", "#00ff00")

-- Prints "#f99500ff"
niji.console.debug(my_color)

```

### `niji.Color:lighten(amount)`

Lightens the color by the given amount. "Amount" here refers to relative perceived
lightness, which means that the change in lightness for a given amount parameter
should look the same for any base color, unless the resulting color would fall outside
the RGB color gamut.

- `amount`: The desired relative perceived lightness, ranging between -1 and 1 (`float`)

```lua
local base_color = niji.Color:new("#123faa")
local lightened_color = base_color:lighten(0.2)

-- Prints "#4a7eeeff"
niji.console.debug(lightened_color)
```

### `niji.Color:darken(amount)`

Darkens the color by the given amount. Equivalent to calling `niji.Color:lighten`
with `-amount`.

- `amount`: The desired relative perceived lightness, ranging between -1 and 1 (`float`)

```lua
local base_color = niji.Color:new("#c670f9")
local lightened_color = base_color:darken(0.2)

-- Prints "#872eb5ff"
niji.console.debug(lightened_color)
```

### `niji.Color:shade(lightness)`

Selects a shade of the color that has the provided absolute perceived lightness. As with other
operations, if that color falls outside the RGB gamut, it gets gamut-clipped.

- `lightness`: The desired perceived lightness ranging, between 0 and 1 (`float`)

```lua
local base_color = niji.Color:new("#cb9174")
local shade = base_color:shade(0.4)

-- Prints "#6c3a1fff"
niji.console.debug(shade)
```

### `niji.Color:with_alpha(alpha)`

Returns the same color with the provided alpha value.

- `alpha`: The desired alpha value ranging, between 0 and 1 (`float`)

```lua
local base_color = niji.Color:new("#abcdef")
local transparent_color = base_color:with_alpha(0.5)

-- Prints "#abcdef80"
niji.console.debug(transparent_color)
```

## Namespace `niji.console`

The `niji.console` namespace provides niji-flavored functions for interacting with
the console.

### `niji.console.debug(message)`

Sends a debug message to the console. Note that these messages are only visible
if `--verbose` is passed as an argument to niji.

- `message`: The message to send (any type)

### `niji.console.info(message)`

Sends an info message to the console.

- `message`: The message to send (any type)

### `niji.console.warn(message)`

Sends a warning message to the console.

- `message`: The message to send (any type)

### `niji.console.error(message)`

Sends an error message to the console.

- `message`: The message to send (any type)

### `niji.console.prompt(message, default)`

Sends a confirmation prompt to the user. If `default` is not nil, pressing enter #6c3a1fon the
prompt without entering a response will return that value. If `default` is nil, pressing
enter without entering a response will trigger a reprompt.

- `prompt`: The message to show in the prompt (any type)
- `default`: The default value for the prompt (`bool` or `nil`)
- returns: The response from the user (`bool`)

```lua
if niji.console.prompt("Do the thing?", true) then
    doTheThing()
end
```

## Namespace `niji.fs`

The namespace `niji.fs` contains functions for interacting with the file system. While it is much
more restrictive than the filesystem api built into lua, it is strongly recommended to use
`niji.fs` functions over raw lua functions whenever possible, because they have a lot of
extra safety features, such as automatically checking for conflicts with preexisting files.

### `niji.fs.write(path, content)`

This function should be used when you have to write to a file that might already
exist on the system, and which you might not want to silently overwrite if it does;
the major example for this is configuration files which don't support including files
from other locations.

If you just want to output a file that can then be included/imported by another program,
consider using `niji.fs.output`. This is often a better approach, because it is a lot less
invasive.

Calling `niji.fs.write` will cause niji to check if the file already exists, and contains
data that wasn't written to it by niji. If that is the case, niji will inform the user via a prompt,
and create a backup of the previous version if necessary. Ultimately, it writes `content` to file
at the given `path`.

- `path`: The absolute path of the file to write to (`string`). You can use "~" to refer to the current user's home directory.
- `content`: The string to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_config(path, content)`

A version of `niji.fs.write` that takes paths relative to `~/.config`.

- `path`: The relative path to the config file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_state(path, content)`

A version of `niji.fs.write` that takes paths relative to `~/.local/state`.

- `path`: The relative path of the state file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.write_data(path, content)`

A version of `niji.fs.write` that takes path relative to `~/.local/share`.

- `path`: The relative path of the data file to write to (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute, canonical path of the file written to (`string`)

### `niji.fs.output(path, content)`

This function should be used if you want to output a file that is then actively imported/included
by another program. An example for this is the hyprland module, which outputs a partial hyprland config file
which you can then include in your config. In many cases, this is the recommended approach over `niji.fs.write`,
because it is less invasive and makes it easier to manage separate config options. Which approach fits each module
better is up to the disgression of the module author however.

The `path` argument for this functions is relative to your module's output folder, which,
by default, is located at `~/.local/share/niji/<module name>`.

- `path`: The relative path of the output file within the output folde (`string`)
- `content`: The content to write to the file (`string`)
- returns: The absolute path of the file that was written to (`string`)

### `niji.fs.read_config_asset(path)`

Reads a file with a path relative to your module's module folder. This is often used for things
like templates, or other assets that are included within the module files.

- `path`: The relative path of the asset file within the module folder (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_config(path)`

Reads a file with a path relative to `~/.config`.

- `path`: The relative path of the config file (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_state(path)`

Reads a file with a path relative to `~/.local/state`.

- `path`: The relative path of the state file (`string`)
- returns: The contents of the file (`string`)

### `niji.fs.read_data(path)`

Reads a file with a path relative to `~/.local/share`.

- `path`: The relative path of the state file (`string`)
- returns: The contents of the file (`string`)

## Namespace `niji.mod`

The namespace `niji.mod` can be used to obtain metadata about the current module.

### `niji.mod.name`

The name of the current module

### `niji.mod.path`

The absolute path to the module folder of the current module
