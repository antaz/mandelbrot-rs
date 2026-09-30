{
  pkgs ? import <nixpkgs> { },
}:

with pkgs;

mkShell {
  packages = [
    rustc
    cargo
    ffmpeg
  ];
  env.RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
}
