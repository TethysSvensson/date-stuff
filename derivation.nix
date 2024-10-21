{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-Nsj9h6BXhUuxHyACEx0v45hd2qHGX8ZLW2AJWjR961I=";
}
