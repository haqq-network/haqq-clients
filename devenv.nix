{ pkgs, inputs, ... }:

{
  # https://devenv.sh/basics/
  # FIXME: trows error, uncomment when fixed
  # env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  # don't clash with rust-analyzer
  env.CARGO_TARGET_DIR = "target/t";

  dotenv.enable = true;

  # https://devenv.sh/packages/
  packages = with pkgs;
    [
      protobuf
      buf

      nodejs

      cargo
      cargo-watch
      cargo-tarpaulin

      grpcurl
      sccache
      openapi-generator-cli
      git

      yq

      openssl
      pkg-config

      (pkgs.callPackage ./nix/protoc-gen-prost-crate.nix { })
    ] ++ (lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
      CoreFoundation
      SystemConfiguration
      Security
    ]));

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

  scripts.ci.exec = ''
    set -e
    
    cargo run -p haqq-build
    if ! git diff --exit-code; then
    echo "Directory is not clean after code generation"
    exit 1
    fi

    cargo test --workspace
  '';
}
