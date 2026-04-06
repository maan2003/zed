{
  pkgs,
  lib,
  stdenv,
  flakeboxLib,
  commitSha ? null,
  toolchainName ? "default",
  withGLES ? false,
  profile ? "release",
  cmake,
  copyDesktopItems,
  envsubst,
  makeFontsConf,
  makeWrapper,
  alsa-lib,
  curl,
  fontconfig,
  freetype,
  git,
  glib,
  libdrm,
  libgbm,
  libgit2,
  libglvnd,
  libva,
  libxcomposite,
  libxdamage,
  libxext,
  libxfixes,
  libxkbcommon,
  libxrandr,
  libx11,
  libxcb,
  nodejs_22,
  openssl,
  perl,
  pkg-config,
  protobuf,
  rustPlatform,
  sqlite,
  vulkan-loader,
  wayland,
  zlib,
  zstd,
}:
assert stdenv.hostPlatform.isLinux;
assert withGLES -> stdenv.hostPlatform.isLinux;
let
  fixedCommitSha = "1234567890123456789012345678901234567890";
  targetPkgs = if toolchainName == "aarch64-linux" then pkgs.pkgsCross.aarch64-multiplatform else pkgs;
  zedCargo = builtins.fromTOML (builtins.readFile ../crates/zed/Cargo.toml);
  gpuLib = if withGLES then targetPkgs.libglvnd else targetPkgs.vulkan-loader;
  buildTarget = toolchain.buildArgs.CARGO_BUILD_TARGET or null;
  linkLibraryPath = lib.makeLibraryPath [
    gpuLib
    targetPkgs.libx11
    targetPkgs.libxcb
    targetPkgs.libxkbcommon
    targetPkgs.wayland
    targetPkgs.libva
  ];
  env = rec {
    ZSTD_SYS_USE_PKG_CONFIG = true;
    FONTCONFIG_FILE = makeFontsConf {
      fontDirectories = [
        ../assets/fonts/lilex
        ../assets/fonts/ibm-plex-sans
      ];
    };
    ZED_UPDATE_EXPLANATION = "Zed has been installed using Nix. Auto-updates have thus been disabled.";
    RELEASE_VERSION = version;
    ZED_COMMIT_SHA = fixedCommitSha;
    PROTOC = "${protobuf}/bin/protoc";
    TARGET_DIR =
      "target/"
      + lib.optionalString (buildTarget != null) "${buildTarget}/"
      + (if profile == "dev" then "debug" else profile);
    LIBRARY_PATH = linkLibraryPath;
    NIX_LDFLAGS = "-L${linkLibraryPath} -rpath ${linkLibraryPath}";
    NIX_OUTPATH_USED_AS_RANDOM_SEED = "norebuilds";
  };
  version =
    zedCargo.package.version
    + "-nightly"
    + "+${builtins.substring 0 7 fixedCommitSha}";
  src = flakeboxLib.filterSubPaths {
    root = builtins.path {
      name = "zed-source";
      path = ../.;
    };
    paths = [
      ".cargo"
      ".config"
      "Cargo.lock"
      "Cargo.toml"
      "assets"
      "crates"
      "extensions"
      "script"
      "tooling"
    ];
  };
  commonArgs = {
    pname = "zed-editor";
    inherit version src;
    cargoLock = ../Cargo.lock;
    strictDeps = true;
    nativeBuildInputs = [
      cmake
      copyDesktopItems
      curl
      makeWrapper
      perl
      pkg-config
      protobuf
      rustPlatform.bindgenHook
    ];
    buildInputs = [
      targetPkgs.alsa-lib
      targetPkgs.curl
      targetPkgs.fontconfig
      targetPkgs.freetype
      targetPkgs.glib
      targetPkgs.libdrm
      targetPkgs.libgbm
      targetPkgs.libgit2
      targetPkgs.libglvnd
      targetPkgs.libva
      targetPkgs.libx11
      targetPkgs.libxcb
      targetPkgs.libxcomposite
      targetPkgs.libxdamage
      targetPkgs.libxext
      targetPkgs.libxfixes
      targetPkgs.libxkbcommon
      targetPkgs.libxrandr
      targetPkgs.openssl
      targetPkgs.sqlite
      targetPkgs.wayland
      gpuLib
      targetPkgs.zlib
      targetPkgs.zstd
    ];
    cargoExtraArgs = "-p zed -p cli --locked --features=gpui_platform/runtime_shaders";
    inherit env;
    passthru.env = env;
    dontPatchELF = true;
    dontUseCmakeConfigure = true;
    doCheck = false;
    preBuild = ''
      echo nightly > crates/zed/RELEASE_CHANNEL
    '';
    installPhase = ''
      runHook preInstall

      mkdir -p $out/bin $out/libexec
      cp $TARGET_DIR/zed $out/libexec/zed-editor
      cp $TARGET_DIR/cli $out/bin/zed
      ln -s $out/bin/zed $out/bin/zeditor

      install -D crates/zed/resources/app-icon-nightly@2x.png \
        "$out/share/icons/hicolor/1024x1024@2x/apps/zed.png"
      install -D crates/zed/resources/app-icon-nightly.png \
        "$out/share/icons/hicolor/512x512/apps/zed.png"

      (
        export DO_STARTUP_NOTIFY="true"
        export APP_CLI="zed"
        export APP_ICON="zed"
        export APP_NAME="Zed Nightly"
        export APP_ARGS="%U"
        mkdir -p "$out/share/applications"
        ${lib.getExe envsubst} < "crates/zed/resources/zed.desktop.in" > "$out/share/applications/dev.zed.Zed-Nightly.desktop"
        chmod +x "$out/share/applications/dev.zed.Zed-Nightly.desktop"
      )

      runHook postInstall
    '';
    postFixup = ''
      wrapProgram $out/libexec/zed-editor --suffix PATH : ${lib.makeBinPath [ nodejs_22 git ]}
    '';
    meta = {
      description = "High-performance, multiplayer code editor from the creators of Atom and Tree-sitter";
      homepage = "https://zed.dev";
      changelog = "https://zed.dev/releases/preview";
      license = lib.licenses.gpl3Only;
      mainProgram = "zed";
      platforms = lib.platforms.linux;
    };
  }
  // lib.optionalAttrs (toolchainName == "aarch64-linux") {
    # flakebox's cross clang target helper hardcodes the prefixed ld path, so
    # override the target rustflags here to force mold instead.
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS =
      "-C link-arg=-fuse-ld=${pkgs.mold-wrapped}/bin/mold -C link-arg=-Wl,--compress-debug-sections=zlib -L native=${targetPkgs.libxcb}/lib -L native=${targetPkgs.libx11}/lib -L native=${targetPkgs.libxkbcommon}/lib";
  };
  toolchains = flakeboxLib.mkStdToolchains {
    stdenv = pkgs': pkgs'.stdenvAdapters.useMoldLinker pkgs'.llvmPackages.stdenv;
  };
  toolchain = toolchains.${toolchainName};
  builds = (flakeboxLib.craneMultiBuild {
    toolchains = { default = toolchain; };
    profiles = [ profile ];
  }) (
    craneLib':
    let
      craneLib = craneLib'.overrideArgs commonArgs;
      cargoArtifacts = craneLib.buildDepsOnly {
        installPhase = "prepareAndInstallCargoArtifactsDir";
        postFixup = "";
      };
    in
    {
      zed-editor = craneLib.buildPackage {
        inherit cargoArtifacts;
      };
    }
  );
in
if profile == "release" then builds.zed-editor else builds.${profile}.zed-editor
