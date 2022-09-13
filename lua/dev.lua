-- ":luafile dev.lua"

-- force reimport
package.loaded['dev'] = nil
package.loaded['prochrome'] = nil
package.loaded['websocket'] = nil
vim.o.rtp = ".," .. vim.o.rtp

vim.keymap.set('n', ',,,', '<cmd>luafile lua/dev.lua<cr>')

Prochrome = require'prochrome'

-- vim.keymap.set('n', ',w', Notes.new_note)
-- vim.keymap.set('n', ',R', function() Notes.add_all_virtual_titles(0) end)
-- vim.keymap.set('n', ',Q', function() Notes.clear_namespace(0) end)
-- vim.keymap.set('n', ',N', function() Notes.new_note() end)
-- vim.keymap.set('n', '<cr>', Notes.enter_link)
