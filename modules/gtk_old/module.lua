local M = {}

local gtk3_colors_template = niji.template.load("niji_colors.css.mustache")
gtk3_colors_template:set_format("color", "#{rx}{gx}{bx}")

local themes_dir = os.getenv("HOME") .. "/.themes"
local theme_location = themes_dir .. "/niji"

function prepare_theme_folder()
	niji.console.debug("Clearing and repopulating " .. theme_location);

	os.execute("mkdir -p " .. themes_dir)
	os.execute("rm -r " .. theme_location)
	os.execute("cp -r ./niji " .. themes_dir)
end

local function apply_gtk3(config, theme)
	niji.console.debug("Rendering gtk3 colors")

	local colors = gtk3_colors_template:render(theme.ui)
	io.open(theme_location .. "/gtk-3.0/niji_colors.css", "w"):write(colors)
end

function M.apply(config, theme)
	prepare_theme_folder()
	apply_gtk3(config, theme)

	niji.console.info("Generated Niji gtk theme")
end

return M;
