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
function Chrome:kill() vim.fn.jobstop(self.jobId) end
function Chrome:cmd(fn, ...) return vim.rpcrequest(self.jobId, fn, ...) end
function Chrome:status()  return self:cmd'status'  end
function Chrome:refresh() return self:cmd'refresh' end
function Chrome:newApp(url) return self:cmd('new_app', url) end
function Chrome:navigateTo(url) return self:cmd('navigate_to', url) end
-- Not usable yet, maybe even unnecesery
-- function Chrome:newChrome(url) return self:cmd('new_chrome', url) end
-- function Chrome:getTabs() return self:cmd'get_tabs' end
-- function Chrome:newTab()  return self:cmd'new_tab' end

local M = {}

local function argsValid(opts)
	for k, v in opts do
		if k ~= 'onStart'
			and k ~= 'onRefresh'
			and k ~= 'url'
		then
			print[[need table arg of shape {
				onStart : list<string>, -- optional
				onRefresh : list<string>, -- optional
				url : string 
			}]]
		end
	end
	if type(opts) ~= 'table'
		or opts.onStart and type(opts.onStart) ~= 'table'
		or opts.onRefresh and type(opts.onRefresh) ~= 'table'
		or type(opts.url) ~= 'string'
	then
		print[[need table arg of shape {
			onStart : list<string>, -- optional
			onRefresh : list<string>, -- optional
			url : string 
		}]]
		return false
	end
	return true
end

function M.newApp(opts)
	if not argsValid(opts) then return end
	return {
		get = function(self)
			if not self.chrome then
				if opts.onStart then vim.fn.jobstart(opts.onStart) end
				self.chrome = Chrome:new()
				if opts.onRefresh then
					self.chrome.refresh = function(s)
						vim.fn.jobstart(opts.onRefresh)
						s:cmd'refresh'
					end
				end
				self.chrome:newApp(opts.url)
			end
			return self.chrome
		end,
	}
end

return M
