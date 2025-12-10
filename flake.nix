{
  description = "aoc2025";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
          config.allowUnfree = true;
        };

        toolchain =
          with fenix.packages.${system};
          combine [
            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
            pkgs.rust-analyzer

            # targets.x86_64-unknown-linux-musl.latest.rust-std
            # targets.aarch64-unknown-linux-musl.latest.rust-std
            # targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        buildPkg =
          arch_pkgs:
          with arch_pkgs;
          (makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          }).buildRustPackage
            {
              pname = "aoc2025";
              version = "0.0.1";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  "aoc-utils-0.1.0" = "";
                };
              };
              strictDeps = true;
              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = fnBuildInputs arch_pkgs;
            };

        fnBuildInputs =
          pkgs: with pkgs; [
            z3
            pkg-config
            clang
          ];
        shellPkgs = with pkgs; [
          cargo-aoc
          hyperfine
        ];
      in
      rec {
        defaultPackage = buildPkg pkgs;
        # packages.x86_64-unknown-linux-musl = buildPkg pkgs.pkgsCross.musl64.pkgsStatic;
        # packages.aarch64-multiplatform-musl = buildPkg pkgs.pkgsCross.aarch64-multiplatform-musl.pkgsStatic;
        # packages.x86_64-pc-windows-gnu = buildPkg pkgs.pkgsCross.mingwW64;

        devShell = pkgs.mkShell {
          inputsFrom = [ defaultPackage ];
          packages = shellPkgs;
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          LD_LIBRARY_PATH = "${pkgs.z3.lib}/lib";
        };
      }
    );
}
