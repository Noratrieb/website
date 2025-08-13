{ pkgs ? import <nixpkgs> { }, ... }:
let
  generator = import ./builder { inherit pkgs; };
in
pkgs.stdenv.mkDerivation {
  name = "website";

  src = ./.;
  unpackSrc = false;

  WEBSITE_RNG_SEED = "99";

  nativeBuildInputs = with pkgs; [ git hugo ];

  buildPhase = ''
    ${generator}/bin/website-builder build
  '';

  installPhase = ''
    mkdir -p $out
    cp -r dist/* $out/
  '';
}
