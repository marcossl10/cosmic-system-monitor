{
  description = "Development shell for cosmic-sys-monitor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      overlays.default = final: prev: {
        cosmic-sys-monitor = final.callPackage ./nix/package.nix { };
      };

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlays.default ];
          };
        in
        {
          cosmic-sys-monitor = pkgs.cosmic-sys-monitor;
          default = pkgs.cosmic-sys-monitor;
        }
      );

      nixosModules.default = import ./nix/module.nix { inherit self; };

      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };

          runtimeLibs = with pkgs; [
            dbus
            expat
            fontconfig
            freetype
            gtk3
            libGL
            libxkbcommon
            lm_sensors
            openssl
            vulkan-loader
            wayland
          ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              just
              pkg-config
              rust-analyzer
              rustc
              rustfmt
              wayland-protocols
            ];

            buildInputs = runtimeLibs;

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
            RUST_BACKTRACE = "1";
          };
        }
      );
    };
}
