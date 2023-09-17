# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "9f0e5916c8bd9c04dd788abf8c923f385fff217d";
  sha256  = "1i4480sspc2nsynglrwsca8s3dp4g93g4vncfi5w99rs8ldg7b32";
})
