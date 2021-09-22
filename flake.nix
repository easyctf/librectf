{
  description = "Open-source CTF administration suite.";

  inputs = { flake-utils.url = "github:numtide/flake-utils"; };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        python39Packages = pkgs.python39Packages;

        myPkgs = rec {
          filestore = pkgs.callPackage ./filestore { };
          judge = python39Packages.callPackage ./judge { };
          frontend = pkgs.callPackage ./frontend {};
          backend = pkgs.callPackage ./backend {};

          # old server
          server = python39Packages.callPackage ./server-old { };
        };
      in {
        packages = flake-utils.lib.flattenTree myPkgs;

        devShell = pkgs.mkShell {
          packages = with pkgs;
            with python39Packages; [
              crate2nix
              black
              nixfmt
            ];
        };
      });
}
