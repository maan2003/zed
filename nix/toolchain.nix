{ inputs, ... }:
pkgs:
let
  rustToolchain = builtins.fromTOML (builtins.readFile ../rust-toolchain.toml);
  toolchainChannel =
    if builtins.match "[0-9].*" rustToolchain.toolchain.channel != null then
      "stable"
    else
      rustToolchain.toolchain.channel;
  flakeboxLib = inputs.flakebox.lib.mkLib pkgs {
    config = {
      toolchain.channel = toolchainChannel;
      toolchain.components = [
        "cargo"
        "rustc"
      ] ++ rustToolchain.toolchain.components;
    };
  };
in
pkgs.callPackage ./build.nix {
  inherit flakeboxLib;
  commitSha = inputs.self.rev or null;
}
