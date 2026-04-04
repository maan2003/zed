{ inputs, ... }:
{
  perSystem =
    { pkgs, lib, ... }:
    let
      mkZed = import ../toolchain.nix { inherit inputs; };
      zed-editor = mkZed pkgs;
    in
    {
      packages =
        {
          default = zed-editor;
          debug = zed-editor.override { profile = "dev"; };
        }
        // lib.optionalAttrs (pkgs.stdenv.hostPlatform.system == "x86_64-linux") {
          "cross-aarch64-linux" = zed-editor.override { toolchainName = "aarch64-linux"; };
          "cross-aarch64-linux-debug" = zed-editor.override {
            toolchainName = "aarch64-linux";
            profile = "dev";
          };
        };
    };
}
