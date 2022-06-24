-- local Ws = require'websocket'
local binary_path = vim.fn.fnamemodify(
        vim.api.nvim_get_runtime_file("lua/prochrome/init.lua", false)[1], ":h:h:h"
    ) .. "/target/debug/prochrome"

local M = {
}

M.run = function(fn, ...)
    if M.job_id == nil then
        M.job_id = vim.fn.jobstart({ binary_path }, { rpc = true })
    end
    return vim.rpcrequest(M.job_id, fn, ...)
end

M.status = function() return M.run'status' end
M.refresh = function() return M.run'refresh' end
M.newapp = function(link) return M.run('newapp', link) end

return M
