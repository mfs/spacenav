//! This crate provides an interface to the open source [Spacenav][0] daemon.
//!
//! This daemon communicates with 3D mice made by [3DConnexion][1] such as the
//! SpaceNavigator.
//!
//! The spacenavd daemon supports two protocols. An X11 protocol compatible with
//! the proprietary daemon as well as an alternative communication protocol that
//! does not require an X server. This crate commmunicates via the second, non X
//! protocol. For now the X11 protocol is not implemented.
//!
//! # Examples
//!
//! ```
//! extern crate spacenav;
//!
//! use spacenav::SpaceNav;
//!
//! fn main() {
//!
//!     let mut spcnav = SpaceNav::new().unwrap();
//!
//!     loop {
//!         let event = spcnav.read();
//!
//!         println!("{:?}", event);
//!     }
//! }
//! ```
//!
//! [0]: http://spacenav.sourceforge.net
//! [1]: http://www.3dconnexion.com

extern crate byteorder;

use byteorder::{LittleEndian, ByteOrder};
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::io;

#[derive(Debug)]
pub enum Event {
    /// Button press event with button number.
    ButtonPress(i32),
    /// Button release event with button number.
    ButtonRelease(i32),
    /// Motion event.
    Motion {
        /// Translation in the `x` axis.
        x: i32,
        /// Translation in the `y` axis.
        y: i32,
        /// Translation in the `z` axis.
        z: i32,
        /// Rotation around the `x` axis or pitch.
        rx: i32,
        /// Rotation around the `y` axis or yaw.
        ry: i32,
        /// Rotation around the `z` axis or roll.
        rz: i32,
        /// Milliseconds since the last motion event.
        period: i32,
    },
}

const SOCKET: &'static str = "/var/run/spnav.sock";

pub struct SpaceNav {
    stream: UnixStream,
}

impl SpaceNav {
    /// Create SpaceNav sturct.
    pub fn new() -> io::Result<SpaceNav> {
        let stream = match UnixStream::connect(SOCKET) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(SpaceNav { stream: stream })
    }

    /// Read next event from daemon.
    ///
    /// If operating in blocking mode this function will block until an event
    /// is available. If operating in nonblocking mode the function returns
    /// immediately and a `None` is returned if no event is available.
    ///
    pub fn read(&mut self) -> Option<Event> {
        let mut buffer = [0; 8 * 4];

        match self.stream.read_exact(&mut buffer) {
            Ok(()) => {}
            Err(_) => return None,
        }

        const MOTION: i32 = 0;
        const BUTTON_PRESS: i32 = 1;
        const BUTTON_RELEASE: i32 = 2;

        let event_type = LittleEndian::read_i32(&buffer);

        match event_type {
            MOTION => {
                Some(Event::Motion {
                    x: LittleEndian::read_i32(&buffer[4..8]),
                    y: LittleEndian::read_i32(&buffer[8..12]),
                    z: LittleEndian::read_i32(&buffer[12..16]),
                    rx: LittleEndian::read_i32(&buffer[16..20]),
                    ry: LittleEndian::read_i32(&buffer[20..24]),
                    rz: LittleEndian::read_i32(&buffer[24..28]),
                    period: LittleEndian::read_i32(&buffer[28..32]),
                })
            }
            BUTTON_PRESS => Some(Event::ButtonPress(LittleEndian::read_i32(&buffer[4..8]))),
            BUTTON_RELEASE => Some(Event::ButtonRelease(LittleEndian::read_i32(&buffer[4..8]))),
            _ => None,
        }
    }

    /// Set blocking state to `nonblocking`.
    pub fn set_nonblocking(&mut self, nonblocking: bool) -> io::Result<()> {
        self.stream.set_nonblocking(nonblocking)
    }
}
