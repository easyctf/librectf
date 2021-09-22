{ buildPythonApplication, nix-gitignore, python39Packages }:

let propagatedBuildInputs = with python39Packages; [ ];
in buildPythonApplication {
  pname = "librectf-judge";
  version = "0.1.0";

  inherit propagatedBuildInputs;
  src = nix-gitignore.gitignoreSource [ ../.gitignore ] ./.;
}
