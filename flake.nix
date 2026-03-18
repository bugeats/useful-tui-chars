{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        pkg = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
        };
      in
      {
        packages.default = pkg;

        packages.readme = pkgs.runCommand "README.md" { } ''
          {
            echo '> Generated from `nix run github:bugeats/useful-tui-chars`'
            echo
            ${pkg}/bin/useful-tui-chars
          } > $out
        '';

        devShells.default = craneLib.devShell {
          packages = [ pkgs.rust-analyzer ];
        };
      }
    );
}
