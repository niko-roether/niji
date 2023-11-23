local M = {}

local template = niji.template.load("theme.conf.mustache")
template:set_format("color", "rgba({rx}{gx}{bx}{ax})")

function M.apply(config, theme)
	local configure_cursor = config.cursor ~= nil and config.cursor_size ~= nil
	if not configure_cursor then
		niji.console.debug("No cursor theme set, skipping cursor config")
	end

	local config = template:render {
		configure_cursor = configure_cursor,
		cursor_theme = config.cursor,
		cursor_size = config.cursor_size,
		border_color = theme.ui.background,
		active_border_color = theme.ui.surface,
		shadow_color = theme.ui.shadow
	}

	local output = niji.fs.open_output("theme.conf")
	output:write(config)

	os.execute("hyprctl reload")
end

return M;
