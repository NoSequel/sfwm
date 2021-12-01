use std::cmp::Reverse;
use std::collections::BinaryHeap;

use x11rb::{
    rust_connection::ReplyOrIdError,
    connection::Connection,
    protocol::{
        xproto::*,
        Event,
    },
};

/// This struct represents the state of a single window within the window manager.
#[derive(Debug)]
struct WindowState {
    window: Window,
    frame_window: Window,
    x: i16,
    y: i16,
    width: u16,
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

struct DragState {
    window: WindowState,
    x: i16,
    y: i16,
}

impl DragState {
    pub fn new(window: WindowState, x: i16, y: i16) -> Self {
        Self {
            window,
            x,
            y
        }
    }
}

struct WmState<'a, C: Connection> {
    connection: &'a C,
    screen_num: usize,
    black_gc: Gcontext,
    windows: Vec<WindowState>,
    pending_expose: Vec<Window>,
    wm_protocols: Atom,
    wm_delete_window: Atom,
    sequences_to_ignore: BinaryHeap<Reverse<u16>>,
    drag_window: Option<DragState>,
}

impl<'a, C: Connection> WmState<'a, C> {
   pub fn new(connection: &'a C, screen_num: usize) -> Result<WmState<'a, C>, ReplyOrIdError> {
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
            screen_num,
            black_gc,
            windows: vec![],
            pending_expose: vec![],
            wm_protocols: wm_protocols.reply()?.atom,
            wm_delete_window: wm_delete_window.reply()?.atom,
            sequences_to_ignore: Default::default(),
            drag_window: None
        })
   }

   pub fn scan_windows(&mut self) -> Result<(), ReplyOrIdError> {
        let screen = &self.connection.setup().roots[self.screen_num];
        let tree_reply = self.connection.query_tree(screen.root)?.reply()?;
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
///
/// [^note] Inspired by https://github.com/dylanaraps/sowm
trait Layout<T: Connection>: ButtonHandler {
    fn new_window(window: Window);
    fn close_window(window: Window);
}

/// Abstraction for making multiple different types of button handlers.
///
/// Provides an abstraction layer to easily implement several different handlers for
/// button presses.
///
/// A button handler should handle the following:
/// - Key and button presses,
/// - Possibly button mappings
///
/// [^note] Implemented by `Layout`
trait ButtonHandler {
    fn button_press(event: Event);
    fn button_release(event: Event);

    fn input_grab(root: Window);

    fn key_press(event: Event);
}
