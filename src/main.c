/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "pico/stdlib.h"
#include <stdio.h>

#ifndef LED_DELAY_MS
#define LED_DELAY_MS 500
#endif

// Initialize the GPIO for the LED
uint pico_led_init(void) {
  uint pico_led_pin_default = 25;
  printf("init default led_pin to %d", pico_led_pin_default);

#ifdef PICO_DEFAULT_LED_PIN
  pico_led_pin_default = PICO_DEFAULT_LED_PIN;
#endif
  gpio_init(pico_led_pin_default);
  gpio_set_dir(pico_led_pin_default, GPIO_OUT);
  return pico_led_pin_default;
}

// Turn the LED on or off
void pico_set_led(uint led_pin, bool led_on) { gpio_put(led_pin, led_on); }

int main() {
  printf("Initializing LED...\n");
  uint target_led = pico_led_init();
  printf("LED initialized.\n");
  while (true) {
    pico_set_led(target_led, true);
    printf("Turning on...");
    sleep_ms(LED_DELAY_MS);
    pico_set_led(target_led, false);
    printf("Turning off...");
    sleep_ms(LED_DELAY_MS);
  }
}
