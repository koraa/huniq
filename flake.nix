{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:nix-community/naersk";

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk = pkgs.callPackage inputs.naersk { };
      in
      rec {
        packages.default = naersk.buildPackage {
          src = ./.;
        };
        apps.default = flake-utils.lib.mkApp { drv = packages.default; };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ packages.default ];
        };
        checks = {
          test = pkgs.runCommand "test"
            {
              nativeBuildInputs = [
                packages.default
                pkgs.bash
                pkgs.which
              ];
            }
            "bash ${./.}/test.sh; touch $out";
          nixpkgs-fmt = pkgs.runCommand "nixpkgs-fmt" { nativeBuildInputs = [ pkgs.nixpkgs-fmt ]; }
            "nixpkgs-fmt --check ${./.}; touch $out";
          cargo-fmt = pkgs.runCommand "cargo-fmt" { nativeBuildInputs = [ pkgs.cargo pkgs.rustfmt ]; }
            "cargo fmt --check --manifest-path ${./.}/Cargo.toml; touch $out";
        };
      }
    );
}
