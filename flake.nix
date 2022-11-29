{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        pythonPackages = pkgs.python310Packages;
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            libmysqlclient
            (python310.withPackages (p: with p; [ black poetry ]))
          ];

          SECRET_KEY = "ad88fec19a7641e5de308e45dd4fa1c5";
        };
      });
}
