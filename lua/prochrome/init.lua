local Internals = require 'prochrome_internals'
local M = {}

M.open = function(args)
  if type(args.on_start) == 'table' then
    args.on_start = { table.remove(args.on_start, 1), args.on_start }
  end
  if type(args.on_refresh) == 'table' then
    args.on_refresh = { table.remove(args.on_refresh, 1), args.on_refresh }
  end
  -- print('args: ', vim.inspect(args))
  Internals.open(args)
end

return M
