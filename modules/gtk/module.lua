local M = {}

local install = require("./install")

function M.apply(config, theme)
	install.install(config, theme)
end

return M;
