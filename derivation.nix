{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-uNU4GvUhS2ZUGUfSQ69G+K3dr9rIFY1Ca2f2JhskIXA=";
}
