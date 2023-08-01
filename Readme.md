# Chrome in your nvim
We all know you can
[embed](https://github.com/glacambre/firenvim)
[neovim](https://github.com/rhysd/NyaoVim)
in your browser, but can you do it the other way around?
Surprisingly, answer is YES :smile:

https://user-images.githubusercontent.com/41148612/192169781-7a260436-723c-4542-a62e-c6e847883529.mp4

## Instalation

Before next step, you will **need** [plenary.nvim](https://github.com/nvim-lua/plenary.nvim) already installed,
so building step would work.

Using [packer.nvim](https://github.com/wbthomason/packer.nvim):
```lua
use {'dimchee/prochrome.nvim', run = ':luafile build.lua' }
```

Using `lazy.nvim`:
```lua
  { 'dimchee/prochrome.nvim' },
```

You don't need to add [`build` step to lazy](https://github.com/folke/lazy.nvim#-plugin-authors)

**Don't forget to install chrome or chromium!!!**

## Usage

### Live Server

You can run any [live server](https://www.npmjs.com/package/live-server), when your chrome starts:
```lua
vim.api.nvim_create_autocmd('FileType', {
  pattern = { 'html', 'css', 'js', 'ts' },
  callback = function()
    vim.keymap.set('n', '<F5>', function()
      -- live-server is automaticaly refreshing on change
      require('prochrome').open {
        is_app = true,
        on_start = { 'live-server', '--no-browser' },
        url = 'http://localhost:8080',
      }
    end, { silent = true, desc = 'Start live-server' })
  end,
})
```
For example, I have my favourite
[live server tool](https://github.com/wking-io/elm-live) configured like this
```lua
vim.api.nvim_create_autocmd('FileType', {
  pattern = 'elm',
  callback = function()
    vim.keymap.set('n', '<F5>', function()
      require('prochrome').open {
        is_app = true,
        on_start = { 'elm-live', 'src/Main.elm', '--', '--debug' },
        url = 'http://localhost:8000',
      }
    end, { silent = true, desc = 'Start elm-live' })
  end,
})
```
If you prefer compiling (or writing) to plain html files, we got you covered too:
```lua
-- Setting everything up for running
vim.keymap.set('n', '<F5>', function()
  require('prochrome').open {
    is_app = true,
    on_refresh = {'pandoc', 'Readme.md', '-o', 'Readme.html'},
    url = 'file://' .. vim.fn.getcwd() .. '/Readme.html' 
  }
end, { silent = true, desc = 'Start or refresh pandoc live server' })
```

### Preview Github Markdown

It is as simple using
[Github CLI](https://cli.github.com/) and
[this extension](https://github.com/yusukebe/gh-markdown-preview).
If you already have gh installed, you just need to run
```
gh extension install yusukebe/gh-markdown-preview
```
Add binding to your lua config, and you are done:
```lua
vim.api.nvim_create_autocmd('BufEnter', {
  pattern = 'Readme.md',
  callback = function()
    vim.keymap.set('n', '<F5>', function()
      require('prochrome').open {
        is_app = true,
        on_start = { 'gh', 'markdown-preview', '--disable-auto-open' },
        url = 'http://localhost:3333',
      }
    end, { silent = true, desc = 'Start github markdown preview' })
  end,
})
```

### View live Documentation

How to use cargo doc...

### Look web through telescope

#### Change tab from telescope

- telescope.nvim + prochrome.nvim = caboom
- Implement google search + telescope

## Features

- Open new chrome instance in app mode
- Refresh current page
- Change page (navigate to a page)
- Find Dom Element

## Future Plans

Maybe it is good idea to implement everything in pure lua,
but that would mean we need some kind of websocket library
(maybe we could write one). There is nice implementation
[here](https://github.com/jbyuki/instant.nvim),
but i couldn't make it to work in this context.

### Maybe useful to expose from headless-chrome
- [interception](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.enable_request_interception)
- [get url](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.get_url) 
- [new tab](https://docs.rs/headless_chrome/latest/headless_chrome/browser/struct.Browser.html#method.new_tab)

## Contributing

## Inspirations

- [All started here](https://github.com/atroche/rust-headless-chrome)
- [Vim chrome devtools](https://github.com/carlosrocha/vim-chrome-devtools)
- [Another rust plugin](https://github.com/michaelb/sniprun)
- [markdown-preview.nvim](https://github.com/iamcco/markdown-preview.nvim)
