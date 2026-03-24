{ pkgs ? import <nixpkgs> {} }:

/* Build environment
   for development */
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];
}
