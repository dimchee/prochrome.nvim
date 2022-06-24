download() {
    echo "Downloading version: " $1
    curl -fsSL https://github.com/dimchee/prochrome/releases/download/$1/prochrome --output prochrome
    mkdir -p target/release/
    mv -f prochrome target/debug/
    chmod a+x target/debug/prochrome
    echo "All done :D"
}

if [[ $(uname) != "Linux" ]]; then
    echo "Sorry, looks you are not running Linux, try compiling yourself"
else
    download v0.0.2
fi
