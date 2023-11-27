local M = {}

local template = niji.template.load("theme.mustache")

function M.apply(config, theme)
	local focused_color = config.focused_color or "surface";
	local focused_text_color = "text" .. focused_color;
	local indicator_color = config.indicator_color or "surface";

	local theme = template:render {
		unfocused = theme.ui.background,
		text_unfocused = theme.ui.text_background,
		focused = theme.ui[focused_color],
		text_focused = theme.ui[focused_text_color],
		font = config.font_family,
		font_size = (config.font_size or 12) * (config.font_scale or 1),
		notify = theme.ui.warning,
		text_notify = theme.ui.text_warning,
		indicator = theme.ui[indicator_color],
		cursor = config.cursor,
		cursor_size = config.cursor_size
	}

	niji.fs.write_output("theme", theme)
end

function M.reload(config)
	if
		config.cursor ~= nil and config.cursor ~= os.getenv("XCURSOR_THEME") or
		config.cursor_size ~= nil and tostring(config.cursor_size) ~= os.getenv("XCURSOR_SIZE")
	then
		niji.console.warn("Some programs will only reflect cursor theme changes after reopening")
	end

	os.execute("swaymsg reload -q")
end

return M
