use x11rb::protocol::Event;

//! Abstraction for making multiple layouts easily.
//! 
//! Provides an abstraction to easily implement several different layout types, such as 
//! tiling window managers, floating window managers or others.
//! 
//! A layout handles the following:
//! - Key and button presses
//! - Workspace handling
//! - Handling windows
//! - And much more
//! 
//! [^note] Heavily inspired by https://github.com/dylanaraps/sowm
trait Layout {
    fn button_press(event: Event);
    fn button_release(event: Event);

    fn key_press(event: Event);

    fn configure_request(event: Event);
}