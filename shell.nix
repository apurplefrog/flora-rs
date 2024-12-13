let
  pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    rust-analyzer
    rustfmt
  ];

  env = { 
    RUST_BACKTRACE = "full";
  }; 
}
