#include "adapter.h"
#include <errno.h>
#include <fcntl.h>
#include <libinput.h>
#include <libudev.h>
#include <poll.h>
#include <unistd.h>

#include <stdio.h>

#define UNUSED(x) (void)(x)

/** State variables **/

KeyEventCallback on_key_down = NULL;
void *on_key_down_userdata = NULL;

KeyEventCallback on_key_up = NULL;
void *on_key_up_userdata = NULL;

struct udev *udev = NULL;
struct libinput *li = NULL;

/** libinput context **/

static int open_restricted(const char *path, int flags, void *user_data) {
  UNUSED(user_data);
  int fd = open(path, flags);
  return fd < 0 ? -errno : fd;
}

static void close_restricted(int fd, void *user_data) {
  UNUSED(user_data);
  close(fd);
}

static const struct libinput_interface interface = {
    .open_restricted = open_restricted,
    .close_restricted = close_restricted,
};

/** Functions **/

void register_on_key_down(KeyEventCallback callback, void *userdata) {
  on_key_down = callback;
  on_key_down_userdata = userdata;
}
void register_on_key_up(KeyEventCallback callback, void *userdata) {
  on_key_up = callback;
  on_key_up_userdata = userdata;
}

int32_t initialize_adapter() {
  if (udev != NULL || li != NULL) {
    return -99;
  }

  udev = udev_new();
  if (udev == NULL)
    return -1;

  li = libinput_udev_create_context(&interface, NULL, udev);
  if (li == NULL) {
    cleanup();
    return -2;
  }

  libinput_udev_assign_seat(li, "seat0");

  return 1;
}

#define SHOULD_RUN (li != NULL)

void cleanup() {
  register_on_key_up(NULL, NULL);
  register_on_key_down(NULL, NULL);

  if (li != NULL) {
    libinput_unref(li);
    li = NULL;
  }

  if (udev != NULL) {
    udev_unref(udev);
    udev = NULL;
  }
}

void _process_events() {
  struct libinput_event *event;
  struct libinput_event_keyboard *kb_event;
  enum libinput_key_state state;

  while (SHOULD_RUN && (event = libinput_get_event(li)) != NULL) {
    if (libinput_event_get_type(event) != LIBINPUT_EVENT_KEYBOARD_KEY) {
      libinput_event_destroy(event);
      continue;
    }

    kb_event = libinput_event_get_keyboard_event(event);
    if (kb_event == NULL) {
      libinput_event_destroy(event);
      continue;
    }

    state = libinput_event_keyboard_get_key_state(kb_event);
    libinput_event_destroy(event);

    switch (state) {
    case LIBINPUT_KEY_STATE_RELEASED:
      if (on_key_up != NULL)
        on_key_up(on_key_up_userdata);
      break;
    case LIBINPUT_KEY_STATE_PRESSED:
      if (on_key_down != NULL)
        on_key_up(on_key_down_userdata);
      break;
    }
  }
}

int _block_until_events_ready() {
  const int fds = 1;
  const int no_timeout = -1;

  struct pollfd pfds[fds];
  pfds[0].fd = libinput_get_fd(li);
  pfds[0].events = POLLIN; // Ready to read data

  while (SHOULD_RUN && poll(pfds, fds, no_timeout)) {
    // poll might have blocked for some time; check again if we should be
    // running
    if (SHOULD_RUN && (pfds[0].revents & POLLIN)) {
      if (libinput_dispatch(li) < 0) {
        return -1;
      }
      return 1;
    }
  }

  return 0;
}

int32_t blocking_loop() {
  if (libinput_dispatch(li) < 0) {
    return -2;
  }
  _process_events();

  while (SHOULD_RUN) {
    int res = _block_until_events_ready();
    if (res < 1) {
      return res;
    }
    _process_events();
  }

  return 0;
}
