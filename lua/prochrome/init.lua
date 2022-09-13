-- local Ws = require'websocket'
local binary_path = vim.fn.fnamemodify(
        vim.api.nvim_get_runtime_file("lua/prochrome/init.lua", false)[1], ":h:h:h"
    ) .. "/target/debug/prochrome"


local Chrome = {
}
Chrome.__index = Chrome
function Chrome:new()
	local chrome = {
		jobId = vim.fn.jobstart({ binary_path }, { rpc = true })
	}
	setmetatable(chrome, self)
	return chrome
end
function Chrome:kill() vim.fn.jobstop( self.jobId ) end
function Chrome:run(fn, ...) return vim.rpcrequest(self.jobId, fn, ...) end
function Chrome:status()  return self:run'status'  end
function Chrome:refresh() return self:run'refresh' end
function Chrome:newApp(link) return self:run('new_app', link) end
function Chrome:newChrome(link) return self:run('new_chrome', link) end
function Chrome:getTabs() return self:run'get_tabs' end
function Chrome:newTab()  return self:run'new_tab' end

local M = {}
function M:chrome()
	if not self._chrome then
		self._chrome = Chrome:new()
	end
	return self._chrome
end
function M:killChrome() self._chrome = self._chrome:kill() end

local function argsValid(opts)
	if type(opts) ~= 'table'
		or type(opts.cmd) ~= 'table'
		or type(opts.url) ~= 'string'
	then
		print'need table arg of shape { cmd : list<string>, url : string }'
		return false
	end
	return true
end

function M:runOrRefresh(opts)
	if not argsValid() then return end
  vim.fn.jobstart(opts.cmd)
	return self:chrome() and self:chrome():refresh()
		or self:newChrome():newapp(opts.url)
end

return M
