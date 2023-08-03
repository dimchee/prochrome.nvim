local Internals = require 'prochrome_internals'
local M = {}

M.get_rust_project_name = function()
  local file = io.open('Cargo.toml', 'rb')
  local package = false
  for line in file and file:lines() do
    if line:find '%[package%]' then
      package = true
    elseif line:find '%[' then
      return nil
    elseif package then
      print(line)
      local _, _, name = line:find 'name%s*=%s*"(.*)"'
      if name then
        return name
      end
    end
  end
  if file then
    file:close()
  end
end

M.get_opened = Internals.get_opened
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
