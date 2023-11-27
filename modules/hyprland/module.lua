local M = {}

local template = niji.template.load("theme.conf.mustache")

function M.apply(config, theme)
	local configure_cursor = config.cursor ~= nil and config.cursor_size ~= nil
	if not configure_cursor then
		niji.console.debug("No cursor theme set, skipping cursor config")
	end

	local theme_conf = template:render {
		configure_cursor = configure_cursor,
		cursor_theme = config.cursor,
		cursor_size = config.cursor_size,
		border_color = theme.ui.background,
		active_border_color = theme.ui[config.focused_color or "surface"],
		shadow_color = theme.ui.shadow
	}

	niji.fs.write_output("theme.conf", theme_conf)
end

function M.reload()
	os.execute("hyprctl reload > /dev/null")
end

return M;
