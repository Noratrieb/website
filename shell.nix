{ pkgs ? import <nixpkgs> { } }: pkgs.mkShell {
  packages = with pkgs; [
    rustup
    hugo
    nodePackages.live-server
  ];
}
