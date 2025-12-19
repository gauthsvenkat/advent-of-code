{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config

              cargo-generate
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt

              uv

              z3
              llvmPackages.libclang.lib
            ]
            ++ lib.optional stdenv.isDarwin libiconv;

            env = lib.optionalAttrs stdenv.isLinux {
              LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
            };
          };
      }
    );
}
