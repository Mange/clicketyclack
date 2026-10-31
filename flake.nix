{
  description = "clicketyclack";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];

      clicketyclack =
        {
          lib,
          rustPlatform,
          pkg-config,
          alsa-lib,
          libinput,
          udev,
        }:
        rustPlatform.buildRustPackage {
          pname = "clicketyclack";
          version = (lib.importTOML ./Cargo.toml).package.version;

          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./build.rs
              ./src
              ./adapters
              ./resources
            ];
          };

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "rodio-0.22.2" = "sha256-ihFMFGWPacxEpH0L4wX9P2GPY6rfZSLzMITFJ65qvvk=";
            };
          };

          nativeBuildInputs = [
            # For audio playback library compilation
            pkg-config
          ];

          buildInputs = [
            alsa-lib

            # liblinuxadapter
            libinput
            udev
          ];

          meta = {
            description = "Plays mechanical keyboard sounds when you type";
            homepage = "https://github.com/Mange/clicketyclack";
            license = lib.licenses.mit;
            mainProgram = "clicketyclack";
            platforms = lib.platforms.linux;
          };
        };
    in
    {
      overlays.default = final: prev: {
        clicketyclack = final.callPackage clicketyclack { };
      };

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          package = pkgs.callPackage clicketyclack { };
        in
        {
          default = package;
          clicketyclack = package;
        }
      );

      checks = forAllSystems (system: {
        default = self.packages.${system}.default;
      });

      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt);

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            hardeningDisable = [ "fortify" ];

            packages = with pkgs; [
              # For generating the resources file
              bash
              rustfmt

              clippy
              rust-analyzer
            ];
          };
        }
      );
    };
}
