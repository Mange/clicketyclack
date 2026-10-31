#ifndef CLICKETYCLACK_ADAPTER_H
#define CLICKETYCLACK_ADAPTER_H

#include <stdint.h>

typedef void (*KeyEventCallback)(void *userdata);

void register_on_key_down(KeyEventCallback callback, void *userdata);
void register_on_key_up(KeyEventCallback callback, void *userdata);

int32_t initialize_adapter();
int32_t blocking_loop();
void cleanup();

#endif
