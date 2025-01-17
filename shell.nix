{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    probe-rs
  ];

  shellHook = ''
    echo Initializing 'nix-shell' environment for rust-embassy RPI-Pico development...

    echo Checking probe-rs
    probe-rs --version

    echo

    echo Checking available Rust toolchain\(s\)...
    rustup toolchain list

    echo

    echo Making sure we have the appropirate rust tools for developping...
    rustup component add rust-analyzer
    rustup component add rustfmt

    echo -e "\nReady to build for RPI-Pico!"
  '';
}
