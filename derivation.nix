{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-GUUz5rYTcHyMiLZ1U6cLTY/KTr3cXPXDPcBWl1oHmT4=";
}
