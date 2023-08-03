Prochrome = require 'prochrome'

-- require('prochrome').open {
--   is_app = true,
--   on_start = { 'live-server', { '--no-browser' } },
--   url = 'http://localhost:8080',
-- }
-- A = Prochrome:open {
--   url = 'https://github.com',
--   on_refresh = function()
--     print 'on refresh'
--   end,
-- }
-- T = A:new_tab 'https://github.com/dimchee'

vim.keymap.set('n', '<F5>', function()
  require('prochrome').open {
    on_start = { 'cargo', 'doc' },
    url = './target/doc/' .. require('prochrome').get_rust_project_name(),
  }
end, { silent = true, desc = 'Start live-server' })
