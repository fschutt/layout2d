extern crate glium;

use ui_screen::UiScreen;

/// Determines which keys are pressed currently (modifiers, etc.)
#[derive(Debug)]
pub struct KeyboardState {
    /// Modifier keys that are currently actively pressed during this cycle
    pub modifiers: Vec<ReducedKbModifier>,
    /// Hidden keys, such as the "n" in CTRL + n. Always lowercase
    pub hidden_keys: Vec<char>,
    /// Actual keys pressed during this cycle (i.e. regular text input)
    pub keys: Vec<char>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self { 
            modifiers: Vec::new(), 
            hidden_keys: Vec::new(), 
            keys: Vec::new() 
        }
    }
}
/// Keyboard modifier key, reduced set suited for desktop UIs.
/// Handles things such as AltGr -> split into "Alt" and "Shift"
/// RShift and LShift are generalized to "Shift", same as Ctrl
/// Fn keys have a number attached to them
/// Other keys are ignored and forgotten
/// There may be problems if both Alt keys are pressed and then released
/// Therefore, keys that have a "right" and a "left" method have a number
/// attached to them, how many keys are currently pressed. Currently, this is
/// not in effect (too much work).
#[derive(Debug, PartialEq)]
pub enum ReducedKbModifier {
    Fn(u8),   // Fn1, Fn2, etc.
    Function, // Function key modifier
    Alt,
    Shift,
    AltGr,    // has to be seperate, same function as alt + shift
    Super,    // "Super" or Windows key
    Ctrl,
    RightClickMenu,
    Tab,
    Esc,
    Del, // "Entf" key
    Return,   // Control character because of shift + return options
    Backspace,
    PgUp,
    PgDown,
    Home,
    End,
    Insert,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

/// Mouse position on the screen
#[derive(Debug)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub left_down: bool,
    pub right_down: bool,
    pub middle_down: bool,
    /// How far has the mouse scrolled in x direction
    pub mouse_scroll_x: f32,
    /// How far has the mouse scrolled in y direction
    pub mouse_scroll_y: f32,
    pub scroll_speed_x: f32,
    pub scroll_speed_y: f32,
}

impl MouseState {
    /// Creates a new mouse state
    /// Input: How fast the scroll (mouse) should be converted into pixels
    /// Usually around 10.0 (10 pixels per mouse wheel line)
    pub fn new(scroll_speed_x: f32,scroll_speed_y: f32) -> Self {
        MouseState { 
            x: 0, 
            y: 0,
            left_down: false,
            right_down: false,
            middle_down: false,
            mouse_scroll_x: 0.0,
            mouse_scroll_y: 0.0,
            scroll_speed_x: scroll_speed_x,
            scroll_speed_y: scroll_speed_y,
        }
    }
}
/// State, size, etc of the window, for comparing to the last frame
#[derive(Debug)]
pub struct WindowState {
    width: u32,
    height: u32,
}

impl WindowState {
    /// Creates a new window state
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

/// Handles the event, updates the UI, then returns if the UI has to be rerendered
pub(crate) fn handle_event(event: &glium::glutin::Event,
                           window: &mut WindowState,
                           keyboard: &mut KeyboardState, 
                           mouse: &mut MouseState, 
                           ui_screen: &mut UiScreen) 
-> bool
{
    // update the state of the input information
    use glium::glutin::Event::*;
    match *event {
        MouseMoved(x, y)                => { handle_mouse_move(mouse, x, y); },
        MouseWheel(delta, phase)        => { handle_mouse_scroll(mouse, delta, phase); },
        KeyboardInput(state, code, _)   => { handle_kb_input(keyboard, state, code); },
        ReceivedCharacter(c)            => { handle_kb_char(keyboard, c); }
        MouseInput(state, button)       => { handle_mouse_click(mouse, state, button); }
        Resized(width, height)          => { handle_resize(window, width, height); }
        _ => {  },
    }

    // now that the state is updated, we have enough information to re-layout the frame
    println!("{:?}", window);

    // println!("{:?}", keyboard);

    // todo: after mouse and events are updated, don't just return, but rather call look for 
    // function pointers, hovering, active state, etc.
    ui_screen.layout()
}

#[inline]
fn handle_resize(window: &mut WindowState, width: u32, height: u32)
{
    window.width = width;
    window.height = height;
}

/// Updates mouse movement
#[inline]
fn handle_mouse_move(mouse: &mut MouseState, x: i32, y: i32) 
{
    mouse.x = x;
    mouse.y = y;
}

/// Updates the mouse state on a click
#[inline]
fn handle_mouse_click(mouse: &mut MouseState, state: glium::glutin::ElementState, button: glium::glutin::MouseButton)
{
    use glium::glutin::MouseButton::*;
    use glium::glutin::ElementState::*;

    match state {
        Pressed => {
            match button {
                Left => { mouse.left_down = true; },
                Right => { mouse.right_down = true; },
                Middle => { mouse.middle_down = true; },
                Other(_) => { },
            }
        },

        Released => {
            match button {
                Left => { mouse.left_down = false; },
                Right => { mouse.right_down = false; },
                Middle => { mouse.middle_down = false; },
                Other(_) => { },
            }
        },
    }
}

/// Updates mouse scroll
#[inline]
fn handle_mouse_scroll(mouse: &mut MouseState, delta: glium::glutin::MouseScrollDelta, phase: glium::glutin::TouchPhase)
{
    if phase == glium::glutin::TouchPhase::Moved {
        match delta {
            glium::glutin::MouseScrollDelta::LineDelta(x, y) => { 
                mouse.mouse_scroll_x = mouse.mouse_scroll_x + (x * mouse.scroll_speed_x); 
                mouse.mouse_scroll_y = mouse.mouse_scroll_y + (y * mouse.scroll_speed_y); 
            }
            glium::glutin::MouseScrollDelta::PixelDelta(x, y) => {
                mouse.mouse_scroll_x += x;
                mouse.mouse_scroll_y += y;
            }
        }
    }
}

/// On keyboard input, we get both a KeyboardInput event as well as a ReceivedCharacter event.
/// This function only checks for possible keyboard modifiers, otherwise forgets about the key
#[inline]
fn handle_kb_input(keyboard: &mut KeyboardState, 
                   state: glium::glutin::ElementState, 
                   code: u8)
{
    let modifier_char = check_modifier_key_u8(code);

    if let Some(cchar) = modifier_char {
        use glium::glutin::ElementState::*;
        match state {
            Pressed => {
                if !keyboard.modifiers.iter().any(|elem| *elem == cchar) {
                    keyboard.modifiers.push(cchar);
                }
            },
            Released => {
                let indices_found = keyboard.modifiers.iter().position(|ref elem| **elem == cchar);
                if let Some(index) = indices_found {
                    keyboard.modifiers.remove(index);
                }
            },
        }
    }
}

/// Checks if the key is really a modifier key, reduces the granularity
#[inline]
fn check_modifier_key_u8(key: u8) -> Option<ReducedKbModifier>
{
    match key {
        9 =>   Some(ReducedKbModifier::Esc),
        23 =>  Some(ReducedKbModifier::Tab),
        67 =>  Some(ReducedKbModifier::Fn(1)),
        68 =>  Some(ReducedKbModifier::Fn(2)),
        69 =>  Some(ReducedKbModifier::Fn(3)),
        70 =>  Some(ReducedKbModifier::Fn(4)),
        71 =>  Some(ReducedKbModifier::Fn(5)),
        72 =>  Some(ReducedKbModifier::Fn(6)),
        73 =>  Some(ReducedKbModifier::Fn(7)),
        74 =>  Some(ReducedKbModifier::Fn(8)),
        75 =>  Some(ReducedKbModifier::Fn(9)),
        76 =>  Some(ReducedKbModifier::Fn(10)),
        95 =>  Some(ReducedKbModifier::Fn(11)),
        96 =>  Some(ReducedKbModifier::Fn(12)),
        108 => Some(ReducedKbModifier::AltGr),
        111 => Some(ReducedKbModifier::ArrowUp),
        116 => Some(ReducedKbModifier::ArrowDown),
        113 => Some(ReducedKbModifier::ArrowLeft),
        114 => Some(ReducedKbModifier::ArrowRight),
        112 => Some(ReducedKbModifier::PgUp),
        117 => Some(ReducedKbModifier::PgDown),
        118 => Some(ReducedKbModifier::Insert),
        119 => Some(ReducedKbModifier::Del),
        110 => Some(ReducedKbModifier::Home),
        115 => Some(ReducedKbModifier::End),
        133 => Some(ReducedKbModifier::Super),
        36 =>  Some(ReducedKbModifier::Return),
        64 =>  Some(ReducedKbModifier::Alt),
        135 =>  Some(ReducedKbModifier::RightClickMenu),
        22 =>  Some(ReducedKbModifier::Backspace),
        50 | 62 => Some(ReducedKbModifier::Shift),
        37 | 105 => Some(ReducedKbModifier::Ctrl),
        151 => Some(ReducedKbModifier::Function),
        _ => { None },
    }
}

/// Handles character input (via string). Some characters are wrongly recognized as characters
/// when in reality, they are control characters.
#[inline]
fn handle_kb_char(keyboard: &mut KeyboardState, key: char)
{
    let modifier_char = check_modifier_key_char(&key, keyboard);

    if let Some(cchar) = modifier_char {
        // key is actually a ctrl + (cchar) key
        if !keyboard.hidden_keys.iter().any(|elem| *elem == cchar) {
            keyboard.hidden_keys.push(cchar);
        }
    } else {
        keyboard.keys.push(key);
    }
}

/// The second parameter tells if the key is both a control key and a character,
/// that were pressed at the same time
#[inline]
fn check_modifier_key_char(key: &char, keyboard: &mut KeyboardState) -> Option<char>
{
    // todo: check if any modifiers are active and fix duplicate key ids

    match *key as u32 {
        25  => { Some('y')},
        24  => { Some('x')},
        22  => { Some('v')},
        3  => { Some('c')},
        13  => { Some('m')},
        2  => { Some('b')},
        14  => { Some('n')},
        1  => { Some('a')},
        19  => { Some('s')},
        4  => { Some('d')},
        6  => { Some('f')},
        7  => { Some('g')},
        8  => { Some('h')},
        10  => { Some('j')},
        11  => { Some('k')},
        12  => { Some('l')},
        39  => { Some('ä')},
        45  => { Some('ß')},
        59  => { Some('ö')},
        27  => { Some('ü')}, // same as Esc??
        16 => { Some('p')},
        15 => { Some('o')},
        9 => { Some('i')},
        21 => { Some('u')},
        26 => { Some('z')},
        20 => { Some('t')},
        18 => { Some('r')},
        5 => { Some('e')},
        23 => { Some('w')},
        17 => { Some('q')},
        0 => { Some('2')},
        27 => { Some('3')},
        28 => { Some('4')},
        29 => { Some('5')},
        30 => { Some('6')},
        31 => { Some('7')},
        32 => { Some('8')},
        33 => { Some('9')},
        _ => { None }
    }
}