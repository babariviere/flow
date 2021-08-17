{
  description = "A fast project switcher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.flow = pkgs.rustPlatform.buildRustPackage rec {
          name = "flow";
          version = "git";

          src = ./.;

          cargoSha256 = "sha256-/wHRjHl6Iv1ysqZh+EB78x5KjkqPctkyZmAEY96FaPM=";
        };

        defaultPackage = self.packages.${system}.flow;

      });
}
