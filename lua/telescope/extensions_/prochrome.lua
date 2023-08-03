local pickers = require 'telescope.pickers'
local finders = require 'telescope.finders'
local conf = require('telescope.config').values
local actions = require 'telescope.actions'
local action_state = require 'telescope.actions.state'

local focus_tab = function(opts, chrome)
  opts = opts or {}
  pickers
    .new(opts, {
      prompt_title = 'tabs:',
      finder = finders.new_table {
        results = chrome and chrome:get_tabs() or {},
        entry_maker = function(tab)
          return {
            value = tab,
            display = tab.title,
            ordinal = tab.title,
          }
        end,
      },
      sorter = conf.generic_sorter(opts),
      attach_mappings = function(prompt_bufnr, _)
        actions.select_default:replace(function()
          actions.close(prompt_bufnr)
          local selection = action_state.get_selected_entry()
          -- print(vim.inspect(selection))
          selection.value:focus()
          -- vim.api.nvim_put({ selection }, '', false, true)
        end)
        return true
      end,
    })
    :find()
end

return require('telescope').register_extension {
  exports = {
    focus_tab = function(opts)
      focus_tab(opts, require 'prochrome')
    end,
  },
}
