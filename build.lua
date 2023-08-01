local Job = require 'plenary.job'
local Path = require 'plenary.path'
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
