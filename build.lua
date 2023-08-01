local Job = require 'plenary.job'
local Path = require 'plenary.path'
local Curl = require 'plenary.curl'
local function build()
  Job:new({
    command = 'cargo',
    args = { 'build', '--release' },
    on_exit = function()
      local ext = jit.os == 'Linux' and '.so'
        or jit.os == 'OSX' and '.dylib'
        or jit.os == 'Windows' and '.dll'
      Path:new('./target/release/libprochrome_internals' .. ext):copy {
        destination = Path:new './lua/prochrome_internals.so',
        override = true,
      }
      -- Path:new('./target'):rm {
      --   recursive = true,
      -- }
      print 'Build done'
    end,
  }):start()
end

local function download()
  -- TODO add other OS-es
  if jit.os == 'Linux' then
    Curl.get {
      url = 'https://github.com/dimchee/prochrome/releases/download/latest/prochrome_internals.so',
      output = './lua/prochrome_internals.so',
    }
  else
    build()
  end
end
download()
