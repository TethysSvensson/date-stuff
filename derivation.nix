{
  rustPlatform,
  nix-gitignore,
}:

rustPlatform.buildRustPackage {
  pname = "date-stuff";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-Kt2QEXe7opfxKg0+nzDSGIvSJ32u7d6hDe+3Wug2Lnw=";
}
