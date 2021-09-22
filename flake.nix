{
  description = "Open-source CTF administration server.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
    python39Packages = pkgs.python39Packages;

    myPkgs = rec {
      filestore = pkgs.callPackage ./filestore {};
      server = python39Packages.callPackage ./server {};
    };
  in
  {
    packages = flake-utils.lib.flattenTree myPkgs;

    devShell = pkgs.mkShell {
      packages = with pkgs; with python39Packages; [
        crate2nix
        black
        nixfmt
      ];
    };
  });
}
