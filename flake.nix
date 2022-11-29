{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        pythonPackages = pkgs.python310Packages;
      in {
        devShell = pkgs.mkShell {
          buildInputs = (with pkgs; [ libmysqlclient ])
            ++ (with pythonPackages; [ poetry ]);

          SECRET_KEY = "ad88fec19a7641e5de308e45dd4fa1c5";
        };
      });
}
