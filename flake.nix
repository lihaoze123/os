{
  description = "rCore-Tutorial-v3 development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "llvm-tools-preview"
          "rust-analyzer"
          "rustfmt"
          "clippy"
        ];
        targets = [ "riscv64gc-unknown-none-elf" ];
      };
      riscvGdb = pkgs.writeShellScriptBin "riscv64-unknown-elf-gdb" ''
        exec ${pkgs.gdb}/bin/gdb "$@"
      '';
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          cargo-binutils
          gdb
          git
          gnumake
          qemu
          riscvGdb
          tmux
          pkgsCross."riscv64-embedded".stdenv.cc
        ];

        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
      };
    };
}
