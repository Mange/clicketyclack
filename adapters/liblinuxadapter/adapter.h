#ifndef CLICKETYCLACK_ADAPTER_H
#define CLICKETYCLACK_ADAPTER_H

#include <stdint.h>

typedef void (*KeyEventCallback)(void *);

extern void *instance;
extern KeyEventCallback on_key_down;
extern KeyEventCallback on_key_up;

void register_on_key_down(KeyEventCallback callback);
void register_on_key_up(KeyEventCallback callback);

int32_t initialize_adapter(void *instance);
int32_t blocking_loop();
void cleanup();

#endif

