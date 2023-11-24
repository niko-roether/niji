local M = {}

local template = niji.template.load("kitty.conf.mustache")
template:set_format("color", "#{rx}{gx}{bx}")

local function get_socket_name(pid)
	local prefix = niji.mod.config.rc_prefix or "unix:/tmp/kitty-rc-"
	return prefix .. pid
end

local function reload(path)
	if niji.mod.config.rc == false then
		return
	end
	local password = niji.mod.config.rc_password or "niji-reload"
	local pids = io.popen("pidof kitty -S \\n", "r")

	local pid = pids:read("*l")
	while pid do
		local socket_name = get_socket_name(pid)
		local socket = io.popen("netcat " .. socket_name)

		socket:write("kitty @ set-colors --password \"" .. password .. "\" " .. path)

		pid = pids:read("*l")
	end
end

function M.apply(config, theme)
	local config = template:render {
		background = theme.ui.background,
		foreground = theme.ui.text_background,
		black = theme.terminal.black,
		red = theme.terminal.red,
		green = theme.terminal.green,
		yellow = theme.terminal.yellow,
		blue = theme.terminal.blue,
		magenta = theme.terminal.magenta,
		cyan = theme.terminal.cyan,
		white = theme.terminal.white,
		bright_black = theme.terminal.bright_black,
		bright_red = theme.terminal.bright_red,
		bright_green = theme.terminal.bright_green,
		bright_yellow = theme.terminal.bright_yellow,
		bright_blue = theme.terminal.bright_blue,
		bright_magenta = theme.terminal.bright_magenta,
		bright_cyan = theme.terminal.bright_cyan,
		bright_white = theme.terminal.bright_white
	}

	local path = niji.fs.write_output("kitty.conf", config);
	reload(path)
end

return M
