use crate::Event;
use std::os::raw::c_void;
use std::sync::mpsc::Sender;

type Callback = extern "C" fn(instance: *mut c_void);

#[link(name = "linuxadapter", kind = "static")]
extern "C" {
    pub fn initialize_adapter(instance: *const c_void) -> i32;
    pub fn cleanup();
    pub fn blocking_loop() -> i32;

    pub fn register_on_key_up(callback: Callback);
    pub fn register_on_key_down(callback: Callback);
}

pub struct Adapter {
    pub channel: Sender<Event>,
}

pub enum ConnectError {
    FailedToOpenDisplay,
    MissingXkbQueryExtension,
    MissingXRecordExtension,
    XRecordAllocationFailed,
    XRecordSetupFailed,
    XRecordEnableFailed,
    UnknownError(i32),
}

impl Adapter {
    pub fn connect(send_channel: Sender<Event>) -> Result<Box<Adapter>, ConnectError> {
        let adapter = Box::new(Adapter {
            channel: send_channel,
        });

        unsafe {
            let instance_pointer = &*adapter as *const Adapter as *const c_void;

            register_on_key_down(keydown_callback);
            register_on_key_up(keyup_callback);

            let result = initialize_adapter(instance_pointer);
            if result < 0 {
                cleanup();
                match result {
                    -1 => return Err(ConnectError::FailedToOpenDisplay),
                    -2 => return Err(ConnectError::MissingXkbQueryExtension),
                    -3 => return Err(ConnectError::MissingXRecordExtension),
                    -4 => return Err(ConnectError::XRecordAllocationFailed),
                    -5 => return Err(ConnectError::XRecordSetupFailed),
                    -6 => return Err(ConnectError::XRecordEnableFailed),
                    n => return Err(ConnectError::UnknownError(n)),
                }
            }
        }

        Ok(adapter)
    }

    pub fn block(&self) -> i32 {
        unsafe { blocking_loop() }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        unsafe {
            cleanup();
        }
    }
}

extern "C" fn keydown_callback(instance: *mut c_void) {
    unsafe {
        let instance = instance as *mut Adapter;
        (*instance).channel.send(Event::KeyDown).unwrap();
    }
}

extern "C" fn keyup_callback(instance: *mut c_void) {
    unsafe {
        let instance = instance as *mut Adapter;
        (*instance).channel.send(Event::KeyUp).unwrap();
    }
}
