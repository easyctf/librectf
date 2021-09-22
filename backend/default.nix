{ pkgs }:

let rustProject = import ./Cargo.nix { inherit pkgs; };

in rustProject.rootCrate.build
