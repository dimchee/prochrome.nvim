if [[ $(uname) != "Linux" ]]; then
    echo "Sorry, looks you are not running Linux, try compiling yourself"
else
    curl -fsSL https://github.com/dimchee/prochrome/releases/download/latest/libprochrome_internal.so \ 
        --output lua/prochrome_internal.so
    echo "All done :D"
fi
