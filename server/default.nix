{ buildPythonApplication, nix-gitignore, python39Packages, callPackage }:

let
  dontCheck = p: p.overrideAttrs (a: {
    setuptoolsCheckPhase = "true";
    doCheck = false;
  });
  extraPypi = callPackage ./extra-pypi.nix {};
  propagatedBuildInputs = with python39Packages; [
    flask
    flask-caching
    flask_login
    flask_wtf
    markdown2
    paramiko
    passlib
    pillow
    pycryptodome
    pyotp
    raven
    requests
    email_validator
    wtforms
    pyqrcode

    # TODO: figure out why these fail tests
    (dontCheck flask_migrate)
  ] ++ (with extraPypi; [
    wtforms-components
  ]);
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
