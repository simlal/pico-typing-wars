{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cmake
    gcc-arm-embedded
  ];

  shellHook = ''
    echo Initialzing environment for RPI-Pico toolchain...

    echo Checking if PICO_SDK_PATH available
    if [ -z "$PICO_SDK_PATH" ]; then
      echo "PICO_SDK_PATH not set in environment, exiting!"
      exit 1
    fi
    echo PICO_SDK_PATH="$PICO_SDK_PATH"

    echo -e "\nPlatformIO is already available for debugging in path."
    echo -e "\nReady to build for RPI-Pico!"
  '';
}
