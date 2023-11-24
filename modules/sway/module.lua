local M = {}

local template = niji.template.load("theme.mustache")
template:set_format("color", "{rx}{gx}{bx}")

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
		indicator = theme.ui[indicator_color]
	}

	niji.fs.write_output("theme", theme)
	os.execute("swaymsg reload -q")
end

return M
