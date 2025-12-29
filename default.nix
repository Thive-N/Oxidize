{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "Oxidize";
  version = "0.1.0";
  src = pkgs.lib.cleanSource ./.;
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };
}
