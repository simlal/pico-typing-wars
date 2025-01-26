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

    echo Ensuring we have the appropirate rust tools for developping...
    rustup component add rust-analyzer
    rustup component add rustfmt

    echo

    echo Ensuring we have gcc for compiling/linking...
    if [ -z "$(which gcc)" ]; then
    echo "gcc is not installed"
    else
    gcc --version
    fi

    echo -e "\n#### Ready to build for RPI-Pico! ####"
  '';
}
