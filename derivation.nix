{
  rustPlatform,
  nix-gitignore,
}:

rustPlatform.buildRustPackage {
  pname = "date-stuff";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-JmHwNKfLmlFYk6yZff/lfwQpPajEMbMVbwxPf5roKwE=";
}
