{ stdenvNoCC, mdbook }:

stdenvNoCC.mkDerivation {
  name = "librectf-docs";
  src = ./.;
  nativeBuildInputs = [ mdbook ];

  installPhase = ''
    mkdir -p $out
    mdbook build -d $out .
  '';
}
