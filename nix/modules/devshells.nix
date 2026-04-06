{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      # NOTE: Duplicated because this is in a separate flake-parts partition
      # than ./packages.nix
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;

      rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
      rustToolchain = rustBin.fromRustupToolchainFile ../../rust-toolchain.toml;

      # Musl cross-compiler for building remote_server
      muslCross = pkgs.pkgsCross.musl64;

      # Cargo build timings wrapper script
      wrappedCargo = pkgs.writeShellApplication {
        name = "cargo";
        runtimeInputs = [ pkgs.nodejs ];
        text =
          let
            pathToCargoScript = ./. + "/../../script/cargo";
          in
          ''
            NIX_WRAPPER=1 CARGO=${rustToolchain}/bin/cargo ${pathToCargoScript} "$@"
          '';
      };
    in
    {
      devShells.default = (pkgs.mkShell.override { inherit (zed-editor) stdenv; }) {
        name = "zed-editor-dev";
        buildInputs = with pkgs; [
          alsa-lib
          curl
          fontconfig
          freetype
          glib
          libdrm
          libgbm
          libgit2
          libglvnd
          libva
          libx11
          libxcb
          libxcomposite
          libxdamage
          libxext
          libxfixes
          libxkbcommon
          libxrandr
          openssl
          sqlite
          wayland
          vulkan-loader
          zlib
          zstd
        ];

        packages = with pkgs; [
          wrappedCargo # must be first, to shadow the `cargo` provided by `rustToolchain`
          rustToolchain # cargo, rustc, and rust-toolchain.toml components included
          cargo-nextest
          cargo-hakari
          cargo-machete
          cargo-zigbuild
          cmake
          # TODO: package protobuf-language-server for editing zed.proto
          # TODO: add other tools used in our scripts

          # `build.nix` adds this to the `zed-editor` wrapper (see `postFixup`)
          # we'll just put it on `$PATH`:
          nodejs_22
          perl
          pkg-config
          protobuf
          zig
        ];

        env = {
          ZSTD_SYS_USE_PKG_CONFIG = true;
          # note: different than `$FONTCONFIG_FILE` in `build.nix` – this refers to relative paths
          # outside the nix store instead of to `$src`
          FONTCONFIG_FILE = pkgs.makeFontsConf {
            fontDirectories = [
              "./assets/fonts/lilex"
              "./assets/fonts/ibm-plex-sans"
            ];
          };
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ZED_ZSTD_MUSL_LIB = "${pkgs.pkgsCross.musl64.pkgsStatic.zstd.out}/lib";
          # For aws-lc-sys musl cross-compilation
          CC_x86_64_unknown_linux_musl = "${muslCross.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc";
        };
      };
    };
}
