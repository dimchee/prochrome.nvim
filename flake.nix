{
  outputs = { self, nixpkgs }: {
    devShell.x86_64-linux = with import nixpkgs { system = "x86_64-linux"; };
      mkShell {
        buildInputs = [ cargo rustc pkg-config openssl chromium rust-analyzer taplo rustfmt clippy ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        _PATH = "./tools/";
      };
    defaultPackage.x86_64-linux = with import nixpkgs { system = "x86_64-linux"; };
      rustPlatform.buildRustPackage {
        name = "prochrome";
        nativeBuildInputs = [ pkg-config ];
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = [ cargo rustc openssl ];
        RUSTFLAGS = "-C target-feature=+crt-static";
        src = self;
      };
  };
}
