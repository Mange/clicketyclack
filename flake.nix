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
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              bash
              pkgconf

              # liblinuxadapter (X11, Pipewire, ALSA)
              libx11
              libxext
              libxtst
              alsa-lib
            ];
          };
        }
      );
    };
}
