use crate::loc;
use windows::{
    core::{PCSTR, PSTR},
    s,
    Win32::{
        Foundation::{GetLastError, BOOL, HINSTANCE, HWND, LPARAM, LRESULT, POINTS, WPARAM},
        Graphics::Gdi::HBRUSH,
        System::{
            Diagnostics::Debug::{
                FormatMessageA, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            },
            LibraryLoader::GetModuleHandleA,
        },
        UI::{
            Input::KeyboardAndMouse::{ReleaseCapture, SetCapture},
            WindowsAndMessaging::{
                CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, GetMessageA,
                LoadCursorW, MessageBoxExA, PeekMessageA, PostQuitMessage, RegisterClassExA,
                ShowWindow, TranslateMessage, HICON, IDC_ARROW, MESSAGEBOX_RESULT,
                MESSAGEBOX_STYLE, MSG, PM_REMOVE, WM_CHAR, WM_CLOSE, WM_DESTROY, WM_KEYDOWN,
                WM_KEYUP, WM_KILLFOCUS, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
                WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN,
                WM_SYSKEYUP, WNDCLASSEXA, WNDCLASS_STYLES, WS_CAPTION, WS_MINIMIZEBOX, WS_SYSMENU,
            },
        },
    },
};

use self::{keyboard::Keyboard, mouse::Mouse};

pub mod error;
pub mod keyboard;
pub mod message;
pub mod mouse;

/**
    We need some public variables for the wndproc because we can't pass in any other arguments in that function.<br>
    I know public variables are bad but i haven't seen a solution to use variables in [`self::wndproc()`].
*/
pub mod io {
    use super::keyboard::Keyboard;
    use super::mouse::Mouse;

    /// The Mouse state
    pub static mut MOUSE: Mouse = Mouse {
        x: 0,
        y: 0,
        event_queue: vec![],
        left_pressed: false,
        right_pressed: false,
        is_in_window: false,
        wheel_pressed: false,
        wheel_delta_carry: 0,
    };

    /// The keyboard state   
    pub static mut KEYBOARD: Keyboard = Keyboard {
        key_states: vec![],
        key_queue: vec![],
        char_queue: vec![],
        auto_repeat_enabled: false,
    };

    /// Width of the window
    pub static mut MAX_MOUSE_X: i16 = 0;
    /// Height of the window
    pub static mut MAX_MOUSE_Y: i16 = 0;
}

/// The Window class which holds every recieved windowEvent and the window data.
pub struct Window<'a> {
    pub instance: HINSTANCE,
    pub class_name: PCSTR,
    pub atom: u16,
    pub width: i16,
    pub height: i16,
    pub class: WNDCLASSEXA,
    pub hwnd: HWND,
    pub msg_buffer: MSG,
    pub last_result: BOOL,
    pub keyboard: &'a mut Keyboard,
    pub mouse: &'a mut Mouse,
}

/// Create a message box
pub fn create_message_box(
    lptext: PCSTR,
    utype: MESSAGEBOX_STYLE,
    wlanguageid: u16,
) -> MESSAGEBOX_RESULT {
    let lpcaption: PCSTR = match utype {
        MESSAGEBOX_STYLE(16) => {
            s!("Fatal error")
        }
        _ => {
            s!("Warning")
        }
    };

    return unsafe { MessageBoxExA(HWND::default(), lptext, lpcaption, utype, wlanguageid) };
    /*
        Creates, displays, and operates a message box. The message box contains an application-defined message and title, plus any
        combination of predefined icons and push buttons. The buttons are in the language of the system user interface.

        For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxexa
    */
}

impl Window<'_> {
    /// Create a window instance
    pub fn new(
        class_name: PCSTR,
        style: WNDCLASS_STYLES,
        window_width: i16,
        window_height: i16,
    ) -> Window<'static> {
        let class_name: PCSTR = class_name; // ID of the program

        /*
            hInstance is the handle to an instance or handle to a module. The
            operating system uses this value to identify the executable or EXE
            when it's loaded in memory.
        */
        let instance: HINSTANCE = unsafe {
            GetModuleHandleA(None).unwrap_or_else(|_| {
                error::WindowError::new(
                    "Unable to create an hInstance with GetModuleHandle.",
                    None,
                    loc!(),
                );
            })
        };

        /*
            Contains window class information. It is used with the RegisterClassEx
            and GetClassInfoEx functions.
            For more info about the fields of this class:
            https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa
        */
        let class: WNDCLASSEXA = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style,
            lpfnWndProc: Some(Self::wndproc),
            hInstance: instance,
            hCursor: unsafe {
                LoadCursorW(None, IDC_ARROW).unwrap_or_else(|_| {
                    error::WindowError::new("Unable to load cursor.", None, loc!());
                })
            },
            lpszClassName: class_name,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON(0isize as _),
            hbrBackground: HBRUSH(0isize as _),
            lpszMenuName: PCSTR(0isize as _),
            hIconSm: HICON(0isize as _),
        };

        /*
            If you register the window class by using RegisterClassExA, the application tells the system that
            the windows of the created class expect messages with text or character parameters to use the ANSI
            character set.

            If the function succeeds, the return value is a class atom that uniquely identifies the class being
            registered. If the function fails, the return value is zero.

            For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa
        */
        let atom: u16 = unsafe { RegisterClassExA(&class) };

        if atom == 0 {
            // check if the registerClass function failed
            panic!("unable to register class");
        }

        /*
            Creates an overlapped, pop-up, or child window. It specifies the window class, window title, window
            style, and (optionally) the initial position and size of the window. The function also specifies
            the window's parent or owner, if any, and the window's menu.

            If the function succeeds, the return value is a handle to the new window. If the function fails, the
            return value is NULL. We can get the error info by calling GetLastError. See GetExitCodes().

            For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
        */
        let hwnd: HWND = unsafe {
            CreateWindowExA(
                windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
                class_name,
                class_name,
                WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
                200,
                200,
                window_width.into(),
                window_height.into(),
                None,
                None,
                instance,
                None,
            )
        };

        unsafe {
            io::KEYBOARD.reset();
            io::MOUSE.reset();
            io::MAX_MOUSE_X = window_width;
            io::MAX_MOUSE_Y = window_height;
        };

        // return the new Window instance
        Window {
            instance,
            class_name,
            atom,
            class,
            hwnd,
            msg_buffer: MSG::default(),
            last_result: BOOL::default(),
            keyboard: unsafe { &mut io::KEYBOARD },
            mouse: unsafe { &mut io::MOUSE },
            width: window_width,
            height: window_height,
        }
    }

    pub fn show_window(&self) {
        // Sets the specified window's show state.
        // Check for more info: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
        unsafe {
            ShowWindow(
                self.hwnd,
                windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD(1),
            );
        };
    }

    /**
        A function to handle all of the Window Events.
        # Example
        ```rust
        let mut input_str: String = "".to_string();
        loop { // has to be in a loop because you want to handle more then one event
            match window.handle_message() {
                Err((msg, result)) => {
                    // result =  (0 = there is an exit without an error) |
                    //           ( -1 = there is an exit with an error)

                    // In this example i'll create an WindowError wich creates an MessageBox with
                    // the error_desc. The error code and the location of the error.
                    if result == -1 {
                        let (error_desc, error_code) = window.get_error_desc();
                        window::error::WindowError::new(&error_desc, Some(error_code as i32), loc!());
                    }

                    break;
                },
                Ok(_msg) => {
                    if let Some(ch) = window.keyboard.read_char() {
                        input_str.push(ch);
                    }
                    if window.keyboard.key_is_pressed_clear(VK_RETURN.0) {
                        println!("{:?}", input_str);
                        input_str = "".to_string();
                    }
                },
            }
        }

        // print the exit codes on a exit without errors
        window.get_exit_codes();
    */
    pub fn handle_message(&mut self) -> Result<MSG, (MSG, i32)> {
        /*
            For info about this i really recommend the video of ChilliTomatoNoodle (https://youtu.be/Fx5bGZ3B_CI?t=152)
            to see how this function works. He explains it better then i ever could. It is in C++ tho.
              ---
            This function needs to be called in a loop in 'main.rs'. It reads the message from the message queue
            and if it returns 0 (exit without an error) or -1 (exit with an error). If returns either of those,
            signal to 'main.rs' to terminate the loop.

            Else translate and dispatch the message. Like i said in the beginning, watch the video, it really
            makes things clear.
        */

        let get_message_result: BOOL = unsafe { GetMessageA(&mut self.msg_buffer, None, 0, 0) };

        if !(get_message_result.0 > 0) {
            self.last_result = get_message_result;
            return Err((self.msg_buffer.to_owned(), get_message_result.0));
        }

        unsafe { TranslateMessage(&mut self.msg_buffer) };
        unsafe { DispatchMessageA(&mut self.msg_buffer) };

        Ok(self.msg_buffer.to_owned())
    }

    pub fn handle_messages(&mut self) -> Option<usize> {
        while unsafe { PeekMessageA(&mut self.msg_buffer, None, 0, 0, PM_REMOVE).as_bool() } {
            if self.msg_buffer.message == WM_QUIT {
                return Some(self.msg_buffer.wParam.0);
            }
            unsafe { TranslateMessage(&mut self.msg_buffer) };
            unsafe { DispatchMessageA(&mut self.msg_buffer) };
        }

        return None;
    }

    extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        /*
            It is very hard to explain how this works without typing a lot of text so i'll just refer you to
            the great video by ChiliTomatoNoodle (https://youtu.be/UUbXK4G_NCM). It explains how the window
            messages work and how to build a good system around it.

            For more info about wndproc see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc
            And for a list with all the messages see: https://wiki.winehq.org/List_Of_Windows_Messages
        */

        unsafe {
            match msg {
                // General window messages
                WM_KILLFOCUS => {
                    io::KEYBOARD.reset();
                }
                WM_CLOSE => {
                    println!("WM_CLOSE");
                    DestroyWindow(hwnd);
                }
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(0);
                }

                // Keyboard messages
                WM_CHAR => {
                    io::KEYBOARD.on_char(wparam.0 as u32);
                }
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    // See https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#keystroke-message-flags
                    let auto_repeat: bool = (lparam.0 >> 30) & 1 == 1;

                    if auto_repeat {
                        io::KEYBOARD.enable_auto_repeat();
                    }
                    io::KEYBOARD.on_key_press(wparam.0 as u32);
                }
                WM_KEYUP | WM_SYSKEYUP => {
                    io::KEYBOARD.disable_auto_repeat();
                    io::KEYBOARD.on_key_release(wparam.0 as u32);
                }

                // Mouse messages
                WM_MOUSEMOVE => {
                    let points: POINTS = make_points(lparam);
                    if points.x >= 0
                        && points.x <= io::MAX_MOUSE_X
                        && points.y >= 0
                        && points.y <= io::MAX_MOUSE_Y
                    {
                        io::MOUSE.on_mouse_move(points);

                        if !io::MOUSE.is_in_window {
                            SetCapture(hwnd);
                            io::MOUSE.on_mouse_enter();
                        }
                    } else {
                        const MK_LBUTTON: i32 = 0x0001;
                        const MK_MBUTTON: i32 = 0x0010;
                        const MK_RBUTTON: i32 = 0x0002;

                        if (wparam.0 as i32) & (MK_LBUTTON | MK_MBUTTON | MK_RBUTTON) > 0 {
                            io::MOUSE.on_mouse_move(points);
                        } else {
                            ReleaseCapture();
                            io::MOUSE.on_mouse_leave();
                        }
                    }
                }
                WM_LBUTTONDOWN => {
                    io::MOUSE.on_left_press();
                }
                WM_LBUTTONUP => {
                    io::MOUSE.on_left_release();
                }
                WM_RBUTTONUP => {
                    io::MOUSE.on_right_release();
                }
                WM_RBUTTONDOWN => {
                    io::MOUSE.on_right_press();
                }

                WM_MBUTTONDOWN => {
                    io::MOUSE.on_wheel_press();
                }
                WM_MBUTTONUP => {
                    io::MOUSE.on_wheel_release();
                }

                WM_MOUSEWHEEL => {
                    let points: POINTS = make_points(lparam);
                    let delta: i16 = get_wheel_delta_wparam(wparam);
                    io::MOUSE.on_wheel_delta(points.x, points.y, delta);
                    println!("{}", io::MOUSE.event_queue.len());
                    println!("{:?}\n", io::MOUSE.event_queue);
                }

                _ => {
                    return DefWindowProcA(hwnd, msg, wparam, lparam);
                }
            }
            LRESULT(0)
        }
    }

    pub fn print_exit_codes(&self) {
        return println!("{}", self.get_error_desc());
    }

    pub fn get_error_desc(&self) -> String {
        let err_code: u32 = unsafe { GetLastError().0 }; // Get the last WIN32_ERROR and get the id from it (u32)
        let mut err_buffer: *mut u8 = std::ptr::null_mut(); // Create a buffer for windows where it should store the error message
        if err_code == 0 {
            // If the error code == 0, there is no error. So there is no need for priting a succes error :)
            return format!(
                "Succesfull exit with codes: last getResult: {:?}, wParam: {}",
                self.last_result.0, self.msg_buffer.wParam.0
            );
        }

        let err_msg_lenght: u32 = unsafe {
            FormatMessageA(
                /*
                    Formats a message string. The function requires a message definition as input.

                    The function finds the message definition in a message table resource based on
                    a message identifier (HRESULT/GetLastError()) and a language identifier (LCID). The function copies the
                    formatted message text to an output buffer, processing any embedded insert
                    sequences if requested.

                    For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage
                */
                FORMAT_MESSAGE_FROM_SYSTEM | // Use system message tables to retrieve error text
                FORMAT_MESSAGE_ALLOCATE_BUFFER, // Allocate buffer on local heap for error text
                None, // Location of the message definition. We use the systems error table so it has to be None
                err_code, // The Errorcode you want a description about
                0,    // LCID (language code identifier) ->
                /*
                This one is a bit weird. In the description the FormatMessage
                function it says we need an LANGID but there is nothing like that in the windows crate. This crate uses a LCID.
                0 means that it will use your system languague. 1033 means US.
                For more info see: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f
                */
                PSTR(&mut err_buffer as *mut _ as *mut _), // Man... this took me ages to get working. ->
                /*
                A pointer to a buffer that receives the null-terminated string that specifies the formatted message.
                    This buffer cannot be larger than 64K bytes.
                    ---
                    We first create a mutable null pointer and set the type to a u8 like this:
                    let mut err_buffer: *mut u8 = std::ptr::null_mut();
                    Then we use the PSTR constructor to create a pointer to a null-terminated string of 8-bit Windows (ANSI) characters.
                    like this:
                        PSTR();
                    Then we put in a mutable reference to the error_buffer and cast it to an mutable pointer (I have no clue how and why this works)
                        PSTR(&mut err_buffer as *mut _ as *mut _);
                */
                0,
                /*
                If the FORMAT_MESSAGE_ALLOCATE_BUFFER flag is not set, this parameter specifies the size of the output buffer, in TCHARs. If
                FORMAT_MESSAGE_ALLOCATE_BUFFER is set, this parameter specifies the minimum number of TCHARs to allocate for an output buffer.
                */
                None, /*
                          An array of values that are used as insert values in the formatted message.
                          For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage#parameters
                      */
            )
        };

        if err_msg_lenght == 0 {
            /*
                If the message buffer is empty, there is no available error description
                Could be caused by an invalid error code or an invalid or not correctly installed LCID
            */

            return format!("Code: {}: Unable to find error description", err_code);
        }

        /*
            If there is an error, print all the return codes,
            println!("Unsuccesfull exit with codes getResult: {:?}, wParam: {}, lastError: {}", self.last_result.0, self.msg_buffer.wParam.0, unsafe { GetLastError().0 });
            and print out the description of the code
        */
        let slice: Vec<u8> =
            unsafe { std::slice::from_raw_parts(err_buffer, (err_msg_lenght - 2) as _).to_vec() };

        return format!("Code {}: {}", err_code, String::from_utf8(slice).unwrap());
    }
}

/**
    This function is not in the windows crate so i made it my self. For more info <br>
    see [this](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-makepoints?source=recommendations)
*/
pub fn make_points(lparam: LPARAM) -> POINTS {
    let coords: i32 = lparam.0 as i32;

    let x: i16 = (coords & 0x0000_FFFF) as i16;
    let y: i16 = ((coords & -0x10000) >> 16) as i16;

    return POINTS { x, y };
}

/**
    This function is not in the windows crate so i made it my self. For more info <br>
    see [this](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel)
*/
pub fn get_wheel_delta_wparam(wparam: WPARAM) -> i16 {
    let wheel_info: i32 = wparam.0 as i32;

    let delta: i16 = ((wheel_info & -0x10000) >> 16) as i16;

    return delta;
}
