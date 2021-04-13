{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-RpUdKqA5Wc6d7n01pLzeM1GEnGo5ik2Lvzm8nonM7Go=";
}
