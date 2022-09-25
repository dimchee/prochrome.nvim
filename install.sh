if [[ $(uname) != "Linux" ]]; then
    echo "Sorry, looks you are not running Linux, try compiling yourself"
else
    echo "Downloading latest binary"
    curl -fsSL https://github.com/dimchee/prochrome/releases/download/latest/prochrome --output prochrome
    mkdir -p target/debug/
    mv -f prochrome target/debug/
    chmod a+x target/debug/prochrome
    echo "All done :D"
fi
