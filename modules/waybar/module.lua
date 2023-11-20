local M = {}

local style_css = niji.template.load("style.css.mustache")
style_css:set_format("color", "rgba({r}, {g}, {b}, {af})")

function M.apply(config, theme)
end

return M
