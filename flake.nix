{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource ./.;

        buildInputs = [
          # Add additional build inputs here
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src buildInputs;
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        aoc2022 = craneLib.buildPackage {
          inherit cargoArtifacts src buildInputs;
        };
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit aoc2022;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          aoc2022-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src buildInputs;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          aoc2022-doc = craneLib.cargoDoc {
            inherit cargoArtifacts src;
          };

          # Check formatting
          aoc2022-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          aoc2022-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `aoc2022` if you do not want
          # the tests to run twice
          aoc2022-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src buildInputs;
            partitions = 1;
            partitionType = "count";
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          aoc2022-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = aoc2022;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
            pre-commit
          ];
        };
      });
}
