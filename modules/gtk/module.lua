local M = {}

local install = require("./install")

function M.apply(config, theme)
	install.install(config, theme)
end

function M.reload(config)
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"\"")
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"niji\"")

	if config.cursor ~= nil then
		niji.console.debug("Applying cursor theme " .. config.cursor)
		os.execute("gsettings set org.gnome.desktop.interface cursor-theme " .. config.cursor)
	end
	if config.cursor_size ~= nil then
		niji.console.debug("Setting cursor size " .. config.cursor_size)
		os.execute("gsettings set org.gnome.desktop.interface cursor-size " .. config.cursor_size)
	end
end

return M;
