{
  rustPlatform,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  src = ./.;
  name = "kickoff-dot-desktop";
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
}
