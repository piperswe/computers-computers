{
  description = "Piper McCorkle's personal blog, \"Computers? Computers.\"";

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    rec {
      packages.website = pkgs.stdenv.mkDerivation {
        name = "computers-computers";

        src = ./.;

        buildInputs = [
          pkgs.hugo
        ];
      };
      packages.default = packages.website;

      devShells.website = packages.website;
      devShells.default = devShells.website;

      checks.website = packages.website;
    });
}
