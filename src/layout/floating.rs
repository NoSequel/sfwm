use crate::layout::layout::{DragState, WmLayout, WmState};
use x11rb::{connection::Connection, protocol::xproto::*};

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

        let window_state = state
            .windows
            .iter()
            .find(|window_state| state.is_window_id(event.event, window_state));

        if let Some(window_state) = window_state {
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
            let window_state = state
                .windows
                .iter()
                .find(|window_state| state.is_window_id(event.event, window_state));

            if let Some(window_state) = window_state {
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
        Ok(())
    }

    fn expose(&self, state: &mut WmState<T>, event: x11rb::protocol::xproto::ExposeEvent) {}

    fn map_request(
        &self,
        state: &mut WmState<T>,
        event: x11rb::protocol::xproto::MapRequestEvent,
    ) -> Result<(), x11rb::rust_connection::ReplyOrIdError> {
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
