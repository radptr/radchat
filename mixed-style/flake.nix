{
  description = "Rust development shell targeting x86_64 Linux musl";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};

    muslCc = pkgs.pkgsCross.musl64.stdenv.cc;
    muslLinker = "${muslCc}/bin/${muslCc.targetPrefix}gcc";
  in {
    devShells.${system}.default = pkgs.mkShell {
      # Host compiler for build scripts; don't set the cross-compiler as generic `cc`.
      packages = [pkgs.stdenv.cc];

      shellHook = ''
        export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="${muslLinker}"
        echo "Musl linker: $CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER"
      '';
    };
  };
}
