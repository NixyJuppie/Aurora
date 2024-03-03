{
  description = "Bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {

          nativeBuildInputs = [
            rust-bin.stable.latest.default
            pkg-config
          ];

          buildInputs = [
            udev
            alsa-lib
            libGL
            vulkan-loader

            # x11 feature
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # wayland feature
            libxkbcommon
            wayland
          ];

          shellHook = "export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
