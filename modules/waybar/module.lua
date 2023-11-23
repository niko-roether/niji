local M = {}

local style_css = niji.template.load("style.css.mustache")
style_css:set_format("color", "rgba({r}, {g}, {b}, {af})")

function get_custom_style(config)
	local custom_style = "";
	if config.custom_style_file ~= nil then
		local file = niji.fs.open_config_asset(config.custom_style_file)
		if file ~= nil then
			custom_style = file:read("*a")
		else
			niji.console.error("Failed to read custom style file " .. file)
		end
	end
	return custom_style
end

function get_font_size(config)
	local base_size = niji.mod.config.font_size or 18
	local scale = config.font_scale or 1

	return (base_size * scale) .. "px"
end

function restart_waybar()
	os.execute("killall waybar")
	niji.os.exec_detached("waybar")
end

function M.apply(config, theme)
	local font_size = get_font_size(config)
	local custom_style = get_custom_style(config)

	print(niji.mod.config.icon_font)


	local style = style_css:render {
		icon_font = niji.mod.config.icon_font,
		font = config.font_family or "sans-serif",
		font_size = font_size,
		transition_duration = niji.mod.config.transition_duration or "200ms",
		hidden_opacity = niji.mod.config.hidden_opacity or 0.5,
		bar_background = "transparent",
		background = theme.ui.background,
		text_background = theme.ui.text_background,
		surface = theme.ui.surface,
		text_surface = theme.ui.text_surface,
		primary = theme.ui.primary,
		text_primary = theme.ui.text_primary,
		secondary = theme.ui.secondary,
		warning = theme.ui.warning,
		text_warning = theme.ui.text_warning,
		padding_x = niji.mod.config.padding_x or "12px",
		padding_y = niji.mod.config.padding_y or "4px",
		margin_x = niji.mod.config.margin_x or "3px",
		margin_y = niji.mod.config.margin_y or "3px",
		workspace_button_margin = niji.mod.config.workspace_button_margin or "6px",
		border_radius = niji.mod.config.border_radius or "4px",
		border_width = niji.mod.config.border_width or "2px",
		custom_modules = niji.mod.config.custom_modules or {},
		custom_style = custom_style
	}

	local output = niji.fs.write_config("waybar/style.css", style)

	restart_waybar()
end

return M
