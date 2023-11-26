local M = {}

local install = require("./install")

function M.apply(config, theme)
	install.install(config, theme)
end

function M.reload()
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"\"")
	os.execute("gsettings set org.gnome.desktop.interface gtk-theme \"niji\"")
end

return M;
