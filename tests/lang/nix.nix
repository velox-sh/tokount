# 11 lines 7 code 3 comments 1 blank
{ pkgs ? import <nixpkgs> {} }:

/* Build environment
   for development */
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];
}
