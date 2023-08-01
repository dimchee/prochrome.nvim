if [[ $(uname) != "Linux" ]]; then
    echo "Sorry, looks you are not running Linux, try compiling yourself"
else
    curl -fsSL https://github.com/dimchee/prochrome.nvim/releases/download/latest/prochrome_internals.so --output lua/prochrome_internals.so
    echo "All done :D"
fi
