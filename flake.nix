{
  description = "heap";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShell = pkgs.mkShell {
        name = "heap";
        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
          clippy
        ];
      };
    });
}
