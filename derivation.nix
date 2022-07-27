{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-Oq2GTXpNM3ztOJCxiZeY+q6E0L08GmW6QM6b4j/lBCE=";
}
