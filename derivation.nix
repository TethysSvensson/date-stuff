{
  rustPlatform,
  nix-gitignore,
}:

rustPlatform.buildRustPackage {
  pname = "date-stuff";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoSha256 = "sha256-4iRBYgCTLNwxMNm1COwPyuVckHkX4JAigYfckOvu0vg=";
}
