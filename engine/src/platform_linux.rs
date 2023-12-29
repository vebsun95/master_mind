pub struct ApplicationState {
    internal_state: InternalState,
}
pub struct InternalState {
    connection: xcb::Connection,
    window: xcb::x::Window,
    wm_protocols: xcb::x::Atom,
    vm_delete_win: xcb::x::Atom,
}

impl ApplicationState {
    #[cfg(target_os = "linux")]
    pub fn new(
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        application_name: &String,
    ) -> ApplicationState {
        use xcb::x;

        let (conn, screen_num) = xcb::Connection::connect(None).expect("Failed to connect to XCB.");
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let window: x::Window = conn.generate_id();
        let cookie = conn.send_request_checked(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x: x, y: y,
            width: width, height: height,
            border_width: 0,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            // this list must be in same order than `Cw` enum order
            value_list: &[
                x::Cw::BackPixel(screen.white_pixel()),
                x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
            ],
        });

        conn.check_request(cookie)
            .expect("Failed to create window.");

        let cookie = conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: application_name.as_bytes(),
        });
        conn.check_request(cookie)
            .expect("Failed to inialize window.");
        conn.send_request(&x::MapWindow { window });

        let (wm_protocols, wm_del_window, _, _, _) = {
            let cookies = (
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_PROTOCOLS",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_DELETE_WINDOW",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MAXIMIZED_VERT",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MAXIMIZED_HORZ",
                }),
            );
            (
                conn.wait_for_reply(cookies.0).unwrap().atom(),
                conn.wait_for_reply(cookies.1).unwrap().atom(),
                conn.wait_for_reply(cookies.2).unwrap().atom(),
                conn.wait_for_reply(cookies.3).unwrap().atom(),
                conn.wait_for_reply(cookies.4).unwrap().atom(),
            )
        };

        conn.check_request(conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[wm_del_window],
        }))
        .expect("Failed to activate the window.");

        conn.flush()
            .expect("Failed to flush connection during initalize procedure.");

        return ApplicationState {
            internal_state: InternalState {
                connection: conn,
                window: window,
                vm_delete_win: wm_del_window,
                wm_protocols: wm_protocols,
            },
        };
    }
    #[cfg(target_os = "linux")]
    pub fn pump_messages(self: &ApplicationState) -> bool {
        let mut quit_flag = false;
        loop {
            match self.internal_state.connection.wait_for_event() {
                Ok(xcb::Event::X(xcb::x::Event::KeyPress(ev))) => {
                    println!("{}", ev.detail());
                }
                Ok(xcb::Event::X(xcb::x::Event::KeyRelease(ev))) => {
                    println!("{}", ev.detail());
                }
                Ok(xcb::Event::X(xcb::x::Event::ClientMessage(ev))) => {
                if let xcb::x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                    if atom == xcb::Xid::resource_id(&self.internal_state.vm_delete_win) {
                        quit_flag = true;
                    }
                } 
                }
                Ok(_) => { break; }
                Err(_) => { break; }
            }
        }

        return quit_flag; 
    }
}
