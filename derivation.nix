{
  rustPlatform,
  nix-gitignore,
}:

rustPlatform.buildRustPackage {
  pname = "date-stuff";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-omdIN+7tFLuZaJX/ExXicEQ/f+kQ1p5iMF1SOrNSFkg=";
}
