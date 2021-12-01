use std::cmp::Reverse;
use std::collections::BinaryHeap;

use x11rb::{
    connection::Connection,
    protocol::{xproto::*, Event},
    rust_connection::ReplyOrIdError,
};

/// This struct represents the state of a single window within the window manager.
#[derive(Debug)]
pub struct WindowState {
    pub window: Window,
    pub frame_window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
}

impl WindowState {
    pub fn new(window: Window, frame_window: Window, geom: &GetGeometryReply) -> WindowState {
        Self {
            window,
            frame_window,
            x: geom.x,
            y: geom.y,
            width: geom.width,
        }
    }
}

pub enum DragType {
    Resize,
    Move,
}

pub struct DragState {
    pub window: WindowState,
    pub drag_type: DragType,
    pub x: i16,
    pub y: i16,
}

impl DragState {
    pub fn new(window: WindowState, drag_type: DragType, x: i16, y: i16) -> Self {
        Self {
            window,
            drag_type,
            x,
            y,
        }
    }
}

pub struct WmState<'a, C: Connection> {
    pub connection: &'a C,
    pub screen_num: usize,
    pub black_gc: Gcontext,
    pub windows: Vec<WindowState>,
    pub pending_expose: Vec<Window>,
    pub wm_protocols: Atom,
    pub wm_delete_window: Atom,
    pub sequences_to_ignore: BinaryHeap<Reverse<u16>>,
    pub drag_window: Option<DragState>,
    pub layout: &'a dyn WmLayout<C>,
}

impl<'a, C: Connection> WmState<'a, C> {
    pub fn new(
        connection: &'a C,
        layout: &'a dyn WmLayout<C>,
        screen_num: usize,
    ) -> Result<WmState<'a, C>, ReplyOrIdError> {
        let screen = &connection.setup().roots[screen_num];
        let black_gc = connection.generate_id()?;
        let font = connection.generate_id()?;

        let gc_aux = CreateGCAux::new()
            .graphics_exposures(0)
            .background(screen.white_pixel)
            .foreground(screen.black_pixel)
            .font(font);

        connection.create_gc(black_gc, screen.root, &gc_aux)?;

        let wm_protocols = connection.intern_atom(false, b"WM_PROTOCOLS")?;
        let wm_delete_window = connection.intern_atom(false, b"WM_DELETE_WINDOW")?;

        Ok(Self {
            connection,
            layout,
            screen_num,
            black_gc,
            windows: vec![],
            pending_expose: vec![],
            wm_protocols: wm_protocols.reply()?.atom,
            wm_delete_window: wm_delete_window.reply()?.atom,
            sequences_to_ignore: Default::default(),
            drag_window: None,
        })
    }

    pub fn scan_windows(&mut self) -> Result<(), ReplyOrIdError> {
        let screen = &self.connection.setup().roots[self.screen_num];
        let tree_reply = self.connection.query_tree(screen.root)?.reply()?;

        for window in tree_reply.children {
            let geometry = self.connection.get_geometry(window)?;
            let attributes = self.connection.get_window_attributes(window)?;

            let (attributes, geometry) = (attributes.reply(), geometry.reply());

            if attributes.is_ok() && geometry.is_ok() {
                let (attributes, geometry) = (attributes.unwrap(), geometry.unwrap());

                if !attributes.override_redirect && attributes.map_state != MapState::UNMAPPED {
                    self.layout.manage_window(self, window, &geometry);
                }
            }
        }

        Ok(())
    }

    pub fn find_window(
        &self,
        window: Window,
    ) -> Option<&WindowState> {
        self.windows
            .iter()
            .find(|window_state| self.is_window_id(window, window_state))
    }

    pub fn is_window_id(&self, window: Window, state: &WindowState) -> bool {
        window == state.window || window == state.frame_window
    }

    fn handle_event(&mut self, event: Event) -> Result<(), ReplyOrIdError> {
        if let Some(sequence) = event.wire_sequence_number() {
            while let Some(&Reverse(to_ignore)) = self.sequences_to_ignore.peek() {
                if to_ignore.wrapping_sub(sequence) <= u16::max_value() / 2 {
                    return Ok(());
                }
            }
        }

        let layout = self.layout;

        match event {
            Event::UnmapNotify(event) => layout.unmap_notify(self, event),
            Event::ConfigureRequest(event) => layout.configure_request(self, event)?,
            Event::MapRequest(event) => layout.map_request(self, event)?,
            Event::Expose(event) => layout.expose(self, event),
            Event::EnterNotify(event) => layout.enter(self, event)?,
            Event::ButtonPress(event) => layout.button_press(self, event)?,
            Event::ButtonRelease(event) => layout.button_release(self, event)?,
            Event::MotionNotify(event) => layout.motion_notify(self, event)?,
            _ => println!("Unhandled X11 event, {:?}", event),
        }

        Ok(())
    }
}

/// Abstraction for making multiple layouts easily.
///
/// Provides an abstraction to easily implement several different layout types, such as
/// tiling window managers, floating window managers or others.
///
/// A layout handles the following:
/// - Workspace handling
/// - Handling windows
/// - And much more
pub trait WmLayout<T: Connection> {
    fn manage_window(
        &self,
        state: &mut WmState<T>,
        window: Window,
        geometry: &GetGeometryReply,
    ) -> Result<(), ReplyOrIdError>;

    fn unmap_notify(&self, state: &mut WmState<T>, event: UnmapNotifyEvent);

    fn configure_request(
        &self,
        state: &mut WmState<T>,
        event: ConfigureRequestEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn map_request(
        &self,
        state: &mut WmState<T>,
        event: MapRequestEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn expose(
        &self,
        state: &mut WmState<T>,
        event: ExposeEvent, // fucking exposed dude
    );

    fn enter(&self, state: &mut WmState<T>, event: EnterNotifyEvent) -> Result<(), ReplyOrIdError>;

    fn button_press(
        &self,
        state: &mut WmState<T>,
        event: ButtonPressEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn button_release(
        &self,
        state: &mut WmState<T>,
        event: ButtonReleaseEvent,
    ) -> Result<(), ReplyOrIdError>;

    fn motion_notify(
        &self,
        state: &mut WmState<T>,
        event: MotionNotifyEvent,
    ) -> Result<(), ReplyOrIdError>;
}
