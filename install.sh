download() {
    echo "Downloading binary: " $1
    curl -fsSL https://github.com/dimchee/prochrome/releases/download/$1/prochrome --output prochrome
    mkdir -p target/release/
    mv -f prochrome target/release/
    chmod a+x target/release/prochrome
    echo "All done :D"
}

if [[ $(uname) != "Linux" ]]; then
    echo "Sorry, looks you are not running Linux, try compiling yourself"
else
    download $1
fi
