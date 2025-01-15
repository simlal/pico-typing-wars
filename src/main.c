/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "pico/stdlib.h"
#include <stdio.h>

int main() {
  stdio_init_all();
  while (true) {
    printf("Hello, world!\n");
    sleep_ms(1000);
  }
}
