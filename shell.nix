{ pkgs ? import <nixpkgs> { } }: pkgs.mkShell {
  packages = with pkgs; [
    rustup
    hugo
    nodejs
    nodePackages.live-server
  ];
}
