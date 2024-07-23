let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    clippy
    rustc
    cargo
    rustfmt
    rust-analyzer
    lldb_18

    # Markdown
    marksman
    markdown-oxide
  ];

  GREETING = "Welcome to MITx 6.00.1x with Rust!";

  shellHook = ''
    echo $GREETING
    echo Entering Rust Development environment...
  '';
}
