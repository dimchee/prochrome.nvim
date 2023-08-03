-- require('prochrome').open {
--   is_app = true,
--   on_start = { 'live-server', { '--no-browser' } },
--   url = 'http://localhost:8080',
-- }
Prochrome = require 'prochrome'
A = Prochrome.open {
  url = 'https://github.com',
}
T = A:new_tab 'https://github.com/dimchee'
