use x11rb::protocol::xproto::Keycode;

/// This is the key used for all key combinations, the default
/// mod key is 133 (super key).
///
/// Other common mod keys are:
/// - ALT (?)
/// - CTRL (?)
pub const MOD_KEY: Keycode = 133;
