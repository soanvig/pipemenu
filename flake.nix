{
  description = "Pipemenu - Gnome (GTK4 + libadwaita) dmenu alternative";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = cargoToml.package.name;
      version = cargoToml.package.version;

      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      nativeBuildInputs = with pkgs; [
        pkg-config
        gtk4
        libadwaita
        pango
      ];
      buildInputs = with pkgs; [
        gtk4
        libadwaita
      ];
    };

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
