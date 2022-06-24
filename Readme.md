# Chrome in your nvim
We all know you can
[embed](https://github.com/glacambre/firenvim)
[neovim](https://github.com/rhysd/NyaoVim)
in your browser, but can you do it the other way around?
Surprisingly, answer is YES :smile:

## Instalation

Using [packer.nvim](https://github.com/wbthomason/packer.nvim)
```lua
use {'dimchee/prochrome', run = 'bash install.sh' }
```
**Don't forget to install chrome or chromium!!!**

## Future Plans

Maybe it is good idea to implement everything in pure lua,
but that would mean we need some kind of websocket library
(maybe we could write one). There is nice implementation
[here](https://github.com/jbyuki/instant.nvim),
but i couldn't make it to work in this context.

- Implement google search + telescope?
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
