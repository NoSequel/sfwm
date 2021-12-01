use crate::layout::layout::{DragState, WmLayout, WmState};
use x11rb::{CURRENT_TIME, connection::Connection, protocol::xproto::*};

struct FloatingWmLayout;

impl<T: Connection> WmLayout<T> for FloatingWmLayout {
    fn motion_notify(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::MotionNotifyEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if let Some(drag_state) = state.drag_window {
            let x: i32 = (drag_state.x + event.root_x).into();
            let y: i32 = (drag_state.y + event.root_y).into();

            state.connection.configure_window(
                drag_state.window.window,
                &ConfigureWindowAux::new().x(x).y(y),
            )?;
        }

        Ok(())
    }

    fn button_release(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::ButtonReleaseEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if event.detail == 1 {
            state.drag_window = None;
        }

        if let Some(window_state) = state.find_window(event.event) {
            // button event handling
        }

        Ok(())
    }

    fn button_press(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::ButtonPressEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if event.detail == 1 {
            if let Some(window_state) = state.find_window(event.event) {
                state.drag_window = Some(DragState::new(
                    *window_state,
                    super::layout::DragType::Move,
                    -event.event_x,
                    -event.event_y,
                ));
            }
        }

        Ok(())
    }

    fn enter(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::EnterNotifyEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        if let Some(window_state) = state.find_window(event.event) {
            state
                .connection
                .set_input_focus(InputFocus::PARENT, window_state.window, CURRENT_TIME)?;

            state
                .connection
                .configure_window(
                    window_state.frame_window,
                    &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
                )?;
        }

        Ok(())
    }

    fn expose(&self, state: &mut WmState<T>, event: x11rb::protocol::xproto::ExposeEvent) {
        state.pending_expose.push(event.window);
    }

    fn map_request(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::MapRequestEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        self.manage_window(
            state,
            event.window,
            &state.connection.get_geometry(event.window)?.reply()?,
        )?;

        Ok(())
    }

    fn configure_request(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::ConfigureRequestEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> { 
        Ok(())
    }

    fn unmap_notify(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::UnmapNotifyEvent,
    ) {
    }

    fn manage_window(
        &self,
        state: &mut WmState<T>,
        window: x11rb::protocol::xproto::Window,
        geometry: &x11rb::protocol::xproto::GetGeometryReply,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
        Ok(())
    }
}
