local M = {}

function M.find_config_file(path, mode)
	return io.open(niji.xdg.config_home .. "/" .. path, mode)
end

function M.find_data_file(path, mode)
	return io.open(niji.xdg.data_home .. "/" .. path, mode)
end

function M.find_state_file(path, mode)
	return io.open(niji.xdg.state_home .. "/" .. path, mode)
end

return M
