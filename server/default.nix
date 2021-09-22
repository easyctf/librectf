{ buildPythonApplication, nix-gitignore, python39Packages }:

let
  dontCheck = p: p.overrideAttrs (a: {
    setuptoolsCheckPhase = "true";
    doCheck = false;
  });
  propagatedBuildInputs = with python39Packages; [
    flask
    flask-caching
    flask_login
    pyotp
    raven
    (dontCheck flask_migrate)
  ];
  checkInputs = with python39Packages; [
    pytest
    webtest
  ];
in
buildPythonApplication {
  pname = "librectf-server";
  version = "0.1.0";

  inherit propagatedBuildInputs checkInputs;
  src = nix-gitignore.gitignoreSource [ ../.gitignore ] ./.;
}
