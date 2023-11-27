local M = {}

local template = niji.template.load("niji.conf.mustache")

function M.apply(config, theme)
	local theme = template:render {
		background = theme.ui.background,
		foreground = theme.ui.text_background,
		url = theme.ui.secondary,
		alert = theme.ui.warning,
		primary = theme.ui.primary,
		text_primary = theme.ui.text_primary,
		surface = theme.ui.surface,
		text_surface = theme.ui.text_surface,
		black = theme.terminal.black,
		red = theme.terminal.red,
		green = theme.terminal.green,
		yellow = theme.terminal.yellow,
		blue = theme.terminal.blue,
		magenta = theme.terminal.magenta,
		cyan = theme.terminal.cyan,
		white = theme.terminal.white,
		bright_black = theme.terminal.bright_black,
		bright_red = theme.terminal.bright_red,
		bright_green = theme.terminal.bright_green,
		bright_yellow = theme.terminal.bright_yellow,
		bright_blue = theme.terminal.bright_blue,
		bright_magenta = theme.terminal.bright_magenta,
		bright_cyan = theme.terminal.bright_cyan,
		bright_white = theme.terminal.bright_white
	}

	niji.fs.write_config("kitty/themes/niji.conf", theme)
end

function M.reload()
	os.execute("kitten themes --reload-in=all niji")
end

return M
