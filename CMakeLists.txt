cmake_minimum_required(VERSION 3.13...3.27)

# initialize the SDK based on PICO_SDK_PATH
include(pico_sdk_import.cmake)

project(typing_wars)

pico_sdk_init()

if(TARGET tinyusb_device)
  add_executable(typing_wars src/main.c)

  # pull in common dependencies
  target_link_libraries(typing_wars pico_stdlib)

  # enable usb output, disable uart output
  pico_enable_stdio_usb(typing_wars 1)
  pico_enable_stdio_uart(typing_wars 0)

  # create map/bin/hex/uf2 file etc.
  pico_add_extra_outputs(typing_wars)

elseif(PICO_ON_DEVICE)
  message(
    "Skipping typing_wars because TinyUSB submodule is not initialized in the SDK"
  )
endif()
