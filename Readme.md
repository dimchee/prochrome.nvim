# Chrome in your nvim
We all know you can
[embed](https://github.com/glacambre/firenvim)
[neovim](https://github.com/rhysd/NyaoVim)
in your browser, but can you do it the other way around?
Surprisingly, answer is YES :smile:

## Instalation

Using [packer.nvim](https://github.com/wbthomason/packer.nvim):
```lua
use {'dimchee/prochrome.nvim', run = 'bash install.sh' }
```
**Don't forget to install chrome or chromium!!!**

## Usage

### Live Server

You can run your favourite
[live server tool](https://github.com/wking-io/elm-live) directly from prochrome.
I recommend, for example adding following keymap to your `after/ftplugin/elm.lua` config file
```lua
-- Setting everything up for running
local chrome = require'prochrome'.newApp {
  onStart = {'elm-live', 'src/Main.elm', '--', '--debug'},
  url = 'http://localhost:8000'
}
-- Function 'get' runs chrome and server if not already running
-- (elm live server is automaticaly refreshing on change)
vim.keymap.set(
  'n', '<C-a>', function() chrome:get() end,
  { silent = true, desc = 'Start elm live server' }
)
```
If you prefer compiling (or writing) to plain html files, we got you covered too:
```lua
-- Setting everything up for running
local chrome = require'prochrome'.newApp {
  onRefresh = {'pandoc', 'Readme.md', '-o', 'Readme.html'},
  url = 'Readme.html'
}
-- Function 'get' makes sure to run chrome
-- and then 'refresh' (with pandoc hook) is ran
vim.keymap.set(
  'n', '<C-a>', function() chrome:get():refresh() end,
  { silent = true, desc = 'Start or refresh pandoc live server' }
)
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
local chrome = require'prochrome'.newApp {
  cmd = {'gh', 'markdown-preview', '--disable-auto-open'},
  url = 'http://localhost:3333'
}
vim.keymap.set(
  'n', '<C-a>', function() chrome:get() end,
  { silent = true, desc = 'Start github markdown preview' }
)
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

Please help! I can't do this alone. Just open a pull request,
and let's make this thing work.

## Inspirations

- [all started here](https://github.com/atroche/rust-headless-chrome)
- [kind of vim version](https://github.com/carlosrocha/vim-chrome-devtools)
- [bindings auto-gen in go](https://github.com/mafredri/cdp/tree/main/cmd/cdpgen)
- [some](https://github.com/pyppeteer/pyppeteer)
[python](https://github.com/fate0/pychrome)
[implementations](https://github.com/iiSeymour/chromote)
- [rust plugin example](https://github.com/michaelb/sniprun)
