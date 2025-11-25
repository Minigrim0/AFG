{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    # Add rust toolchain to nix environment for consistency
    rustc
    cargo
    rustfmt
    clippy
  ];

  buildInputs = [
    udev
    alsa-lib-with-plugins
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr # To use the x11 feature
    libxkbcommon
    wayland # To use the wayland feature
    imagemagick # For sprite management
  ];

  # Create consistent library path
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  # Optimize cargo compilation
  CARGO_TARGET_DIR = "target"; # Ensure consistent target directory
  CARGO_INCREMENTAL = "1"; # Enable incremental compilation
  RUST_BACKTRACE = "1"; # Better error reporting

  # Set consistent cargo cache location
  CARGO_HOME = ".cargo";

  # Prevent nix from interfering with cargo's dependency resolution
  CARGO_NET_OFFLINE = "false";

  # Use all available CPU cores for compilation
  CARGO_BUILD_JOBS = toString (lib.min 16 (lib.max 1 pkgs.stdenv.hostPlatform.parsed.cpu.cores or 8));

  shellHook = ''
        echo "Rust development environment loaded"
        echo "Cargo build jobs: $CARGO_BUILD_JOBS"

        # Ensure cargo directories exist and have consistent permissions
        mkdir -p .cargo target

        # Create a cargo config to use consistent settings
        mkdir -p .cargo
        cat > .cargo/config.toml << EOF
    [build]
    jobs = $CARGO_BUILD_JOBS
    incremental = true

    [env]
    PKG_CONFIG_PATH = "${lib.makeSearchPath "lib/pkgconfig" buildInputs}"
    EOF
  '';
}
