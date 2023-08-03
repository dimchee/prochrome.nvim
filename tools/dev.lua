-- require('prochrome').open {
--   is_app = true,
--   on_start = { 'live-server', { '--no-browser' } },
--   url = 'http://localhost:8080',
-- }
--
require('telescope').load_extension 'prochrome'
Prochrome = require 'prochrome'
A = Prochrome.open {
  url = 'https://github.com',
}
T = A:new_tab 'https://github.com/dimchee'
T = A:new_tab 'https://duckduckgo.com'
