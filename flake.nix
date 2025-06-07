{
  description = "Pipemenu shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        pkg-config
        gtk4
        libadwaita
      ];

      name = "Pipemenu shell";

      shellHook = ''
        exec fish
      '';
    };
  };
}
