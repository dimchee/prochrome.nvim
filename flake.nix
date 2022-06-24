{
  outputs = { self, nixpkgs }: {
    devShell.x86_64-linux = with import nixpkgs { system = "x86_64-linux"; };
      mkShell {
        buildInputs = [ cargo rustc pkg-config openssl chromium ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    defaultPackage.x86_64-linux = with import nixpkgs { system = "x86_64-linux"; };
      rustPlatform.buildRustPackage {
        name = "prochrome";
        nativeBuildInputs = [ pkg-config ];
        cargoLock = {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "headless_chrome-0.9.0" = "sha256-Sj9Qgkj9A7+oqAdn0v9wqmrmK6YD5T5z2ai5+7bBb6s=";
          };
        };
        buildInputs = [ cargo rustc openssl ];
        RUSTFLAGS = "-C target-feature=+crt-static";
        src = self;
      };
  };
}
