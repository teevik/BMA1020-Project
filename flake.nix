{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let pkgs = nixpkgs.legacyPackages.${system}; in
    {
      # TODO runner for # cross build --target x86_64-pc-windows-gnu --release && wine ./target/x86_64-pc-windows-gnu/release/bma1020-project.exe

      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustup
          cargo-cross
          wineWowPackages.wayland
        ];
      };
    });
}
