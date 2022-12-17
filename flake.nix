{
  description = "Piper McCorkle's personal blog, \"Computers? Computers.\"";

  inputs.naersk = {
    url = github:nix-community/naersk;
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      naersk' = pkgs.callPackage naersk { };
    in
    rec {
      packages.server = naersk'.buildPackage {
        name = "computers-computers";
        src = ./.;
      };
      packages.website = pkgs.runCommand
        "computers-computers-website"
        {
          nativeBuildInputs = [ packages.server ];
        }
        ''
          mkdir -p $out/share/computers-computers
          computers-computers generate $out/share/computers-computers
        '';
      packages.default = packages.server;

      devShells.server = packages.server;
      devShells.website = packages.website;
      devShells.default = devShells.server;

      checks.server = packages.server;
      checks.website = packages.website;
    });
}
