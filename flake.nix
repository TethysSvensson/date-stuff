{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        date-stuff = pkgs.callPackage ./derivation.nix { };
      in
      {
        checks = {
          inherit date-stuff;
        };
        packages = {
          inherit date-stuff;
          default = date-stuff;
        };
      }
    );
}
