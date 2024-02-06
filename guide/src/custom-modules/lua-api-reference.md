# Lua API Reference

The niji api resides in the global `niji` namespace. There is no need to import it.
The niji namespace contains several submodules and classes for different purposes, which are listed in
this document.

For more complicated or nonstandard use cases, you can always use the [Lua standard library](https://www.lua.org/manual/5.3/)
which is fully supported. If one of the functions in the niji api fits what you want to do however,
you should always prefer using the niji api, as it provides better integration and safety features.

Contents:

- [Class `niji.Color`](#class-nijicolor)
- [Module `niji.console`](#module-nijiconsole)

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

#### Parameters

- `color_1`: The first of the two colors to mix together (`string` or `niji.Color`)
- `color_2`: The second of the two colors to mix together (`string` or `niji.Color`)

#### Example

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

## Module `niji.console`

The `niji.console` module provides niji-flavored functions for interacting with
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

Sends a confirmation prompt to the user. If `default` is not nil, pressing enter on the
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
