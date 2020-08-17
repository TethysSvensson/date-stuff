{ rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "date-stuff";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "0glr0xd9fmn07p1zap6wpm7cm3sd1fkm6xdni267qw0knvk36i8r";
}
