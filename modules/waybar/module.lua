local M = {}

local style_css = niji.template.load("style.css.mustache")

local function get_custom_style(config)
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

local function get_font_size(config)
	local base_size = config.font_size or 18
	local scale = config.font_scale or 1

	return (base_size * scale) .. "px"
end

function M.apply(config, theme)
	local font_size = get_font_size(config)
	local custom_style = get_custom_style(config)

	local style = style_css:render {
		icon_font = config.icon_font,
		font = config.font_family,
		font_size = font_size,
		transition_duration = config.transition_duration or "200ms",
		hidden_opacity = config.hidden_opacity or 0.5,
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
		padding_x = config.padding_x or "12px",
		padding_y = config.padding_y or "4px",
		margin_x = config.margin_x or "3px",
		margin_y = config.margin_y or "3px",
		workspace_button_margin = config.workspace_button_margin or "6px",
		border_radius = config.border_radius or "4px",
		border_width = config.border_width or "2px",
		custom_modules = config.custom_modules or {},
		custom_style = custom_style
	}

	local output = niji.fs.write_config("waybar/style.css", style)
end

function M.reload()
	os.execute("killall waybar")
	niji.os.exec_detached("waybar &> /dev/null")
end

return M
