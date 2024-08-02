{ pkgs ? import <nixpkgs> { }, ... }: pkgs.rustPlatform.buildRustPackage {
  pname = "website-builder";
  version = "0.1.0";

  src = ./.;

  check = false;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
