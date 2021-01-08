#include "adapter.h"
#include <X11/XKBlib.h>
#include <X11/Xlib.h>
#include <X11/Xproto.h>
#include <X11/extensions/record.h>
#include <math.h>
#include <sys/select.h>

void *instance;
KeyEventCallback on_key_down;
KeyEventCallback on_key_up;

// Internal event callback
void event_callback(XPointer, XRecordInterceptData *);

// Display to receive data on
Display *data_display = NULL;

// Display to send commands on
Display *control_display = NULL;

// Parameters for recording
XRecordRange *record_range = NULL;

// Actual context of recording
XRecordContext record_context;

int max(int a, int b) { return a > b ? a : b; }

void register_on_key_down(KeyEventCallback callback) { on_key_down = callback; }
void register_on_key_up(KeyEventCallback callback) { on_key_up = callback; }

int32_t initialize_adapter(void *_instance) {
  instance = _instance;

  data_display = XOpenDisplay(NULL);
  if (!data_display) {
    cleanup();
    return -1;
  }

  control_display = XOpenDisplay(NULL);
  if (!control_display) {
    cleanup();
    return -1;
  }

  XSynchronize(control_display, True);

  // Verify required extensions are installed
  int _idgaf;
  if (!XkbQueryExtension(control_display, &_idgaf, &_idgaf, &_idgaf, &_idgaf,
                         &_idgaf)) {
    cleanup();
    return -2;
  }
  if (!XRecordQueryVersion(control_display, &_idgaf, &_idgaf)) {
    cleanup();
    return -3;
  }

  // Good, now set up the events that are supposed to be captured.
  record_range = XRecordAllocRange();
  if (!record_range) {
    cleanup();
    return -4;
  }
  record_range->device_events.first = KeyPress;
  record_range->device_events.last = KeyRelease;

  XRecordClientSpec all_clients = XRecordAllClients;

  record_context = XRecordCreateContext(control_display, 0, &all_clients, 1,
                                        &record_range, 1);
  if (!record_context) {
    cleanup();
    return -5;
  }

  if (!XRecordEnableContextAsync(data_display, record_context, event_callback,
                                 NULL)) {
    cleanup();
    return -6;
  }

  return 1;
}

void cleanup() {
  if (control_display != NULL) {
    XRecordDisableContext(control_display, record_context);
    XRecordFreeContext(data_display, record_context);
  }

  if (record_range != NULL) {
    XFree(record_range);
  }

  if (control_display != NULL) {
    XCloseDisplay(control_display);
  }

  if (data_display != NULL) {
    XCloseDisplay(data_display);
  }
}

int32_t blocking_loop() {
  int control_fd = XConnectionNumber(control_display);
  int data_fd = XConnectionNumber(data_display);

  while (1) {
    fd_set fds;
    FD_ZERO(&fds);
    FD_SET(control_fd, &fds);
    FD_SET(data_fd, &fds);

    struct timeval timeout;
    timeout.tv_sec = 2;
    timeout.tv_usec = 0;

    int result =
        select(max(control_fd, data_fd) + 1, &fds, NULL, NULL, &timeout);
    if (result == -1) {
      return -1;
    }

    if (FD_ISSET(data_fd, &fds)) {
      XRecordProcessReplies(data_display);
    }

    if (FD_ISSET(control_fd, &fds)) {
      XEvent event;
      XNextEvent(control_display, &event);
      if (event.type == MappingNotify) {
        XMappingEvent *e = (XMappingEvent *)&event;
        if (e->request == MappingKeyboard) {
          XRefreshKeyboardMapping(e);
        }
      }
    }
  }

  return 1;
}

/*
Type not defined in the X11 libs; copied from Espanso repo, which adapted it
from libxnee.
*/
typedef union {
  unsigned char type;
  xEvent event;
  xResourceReq req;
  xGenericReply reply;
  xError error;
  xConnSetupPrefix setup;
} XRecordDatum;

void event_callback(XPointer p, XRecordInterceptData *data) {
  // Skip if not from server
  if (data->category != XRecordFromServer) {
    XRecordFreeData(data);
    return;
  }

  // I have no idea how this fits together. This is copypasta all over the
  // internet, and no one is figuring it out.
  // X11 people should try to document more, I guess.
  int repeat = data->data[2] & 1;
  if (repeat) {
    return;
  }

  XRecordDatum *inner = (XRecordDatum *)data->data;

  switch (inner->type) {
  case KeyPress:
    on_key_down(instance);
    break;
  case KeyRelease:
    on_key_up(instance);
    break;
  }
}
