{
  rustPlatform,
  nix-gitignore,
}:

rustPlatform.buildRustPackage {
  pname = "date-stuff";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoSha256 = "sha256-Nsj9h6BXhUuxHyACEx0v45hd2qHGX8ZLW2AJWjR961I=";
}
