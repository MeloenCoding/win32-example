use windows::{Win32::{UI::{WindowsAndMessaging::{DefWindowProcA, CreateWindowExA, WS_CAPTION, WS_SYSMENU, ShowWindow, LoadCursorW, IDC_ARROW, WNDCLASSEXA, RegisterClassExA, WM_CLOSE, PostQuitMessage, WS_MINIMIZEBOX, HICON, WM_DESTROY, DestroyWindow, WNDCLASS_STYLES, MSG, GetMessageA, TranslateMessage, DispatchMessageA, MessageBoxExA, MESSAGEBOX_STYLE, MESSAGEBOX_RESULT, WM_KEYDOWN, WM_KEYUP, WM_CHAR, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_KILLFOCUS, KF_REPEAT}}, Foundation::{HWND, WPARAM, LPARAM, LRESULT, HINSTANCE, BOOL, GetLastError}, System::{LibraryLoader::{GetModuleHandleA}, Diagnostics::Debug::{FormatMessageA, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_ALLOCATE_BUFFER}}, Graphics::Gdi::HBRUSH}, core::{PCSTR, PSTR}, s};
use crate::{loc};

use self::keyboard::Keyboard;

pub mod error;
pub mod mouse;
pub mod keyboard;
pub mod message;

pub mod io {
    // We need some public variables for the wndproc because we can't pass in any other arguments in that function
    use super::mouse::Mouse;
    use super::keyboard::Keyboard;

    // static mut D: Vec<bool> = Vec::with_capacity(256);

    pub static mut MOUSE: Mouse = Mouse { x: 0, y: 0 };
    // pub static mut KEYBOARD: Keyboard = Keyboard { key_type: super::keyboard::KeyType::Idle, key_state: super::keyboard::KeyState::Invalid, key_code: None, char_code: None};
    pub static mut KEYBOARD: Keyboard = Keyboard { 
        // key_type: super::keyboard::KeyType::Idle, key_state: super::keyboard::KeyState::Invalid, 
        // key_buffer: None, char_buffer: None, 
        key_states: None, key_buffer: None, 
        char_buffer: None, auto_repeat_enabled: false
     };
}

pub struct Window<'a> {
    pub instance: HINSTANCE,
    pub class_name: PCSTR,
    pub atom: u16,
    pub class: WNDCLASSEXA,
    pub hwnd: HWND,
    pub msg_buffer: MSG,
    pub last_result: BOOL,
    pub keyboard: &'a mut Keyboard,
    // pub mouse: &'a Mouse
}

pub fn create_message_box(lptext: PCSTR, utype: MESSAGEBOX_STYLE, wlanguageid: u16 ) -> MESSAGEBOX_RESULT {
    let lpcaption: PCSTR = match utype {
        MESSAGEBOX_STYLE(16) => {
            s!("Fatal error")
        },
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
    pub fn new(class_name: PCSTR, style: WNDCLASS_STYLES) -> Window<'static> {
        let class_name: PCSTR = class_name; // ID of the program
        
        let instance: HINSTANCE = unsafe { 
            /* 
                hInstance is the handle to an instance or handle to a module. The 
                operating system uses this value to identify the executable or EXE 
                when it's loaded in memory.
            */

            GetModuleHandleA(None).unwrap_or_else(|_| {
                error::WindowError::new("Unable to create an hInstance with GetModuleHandle.", None, loc!());
            })
        };

        let class: WNDCLASSEXA = WNDCLASSEXA { 
            /*
                Contains window class information. It is used with the RegisterClassEx 
                and GetClassInfoEx functions.
                For more info about the fields of this class: 
                https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa
            */

            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style,
            lpfnWndProc: Some(Self::wndproc),
            hInstance: instance,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap_or_else(|_| {
                error::WindowError::new("Unable to load cursor.", None, loc!());
            }) },
            lpszClassName: class_name,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON(0isize as _), 
            hbrBackground: HBRUSH(0isize as _), 
            lpszMenuName: PCSTR(0isize as _), 
            hIconSm: HICON(0isize as _),
        };

        let atom: u16 = unsafe { 
            /*
                If you register the window class by using RegisterClassExA, the application tells the system that 
                the windows of the created class expect messages with text or character parameters to use the ANSI 
                character set.

                If the function succeeds, the return value is a class atom that uniquely identifies the class being 
                registered. If the function fails, the return value is zero.

                For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa
            */

            RegisterClassExA(&class) 
        };

        if atom == 0 { // check if the registerClass function failed
            panic!("unable to register class");
        }
        
        let hwnd: HWND = unsafe { 
            /*
                Creates an overlapped, pop-up, or child window. It specifies the window class, window title, window 
                style, and (optionally) the initial position and size of the window. The function also specifies 
                the window's parent or owner, if any, and the window's menu.

                If the function succeeds, the return value is a handle to the new window. If the function fails, the 
                return value is NULL. We can get the error info by calling GetLastError. See GetExitCodes().

                For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
            */

            CreateWindowExA(windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            class_name, class_name, 
            WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
            200, 200, 896, 672,
            None, None, instance, None) 
        };

        unsafe { io::KEYBOARD.reset() };

        // return the new Window instance
        Window { instance, class_name, atom, class, hwnd, msg_buffer: MSG::default(), last_result: BOOL::default(), keyboard: unsafe { &mut io::KEYBOARD }}

    }

    pub fn show_window(&self) {
        // Sets the specified window's show state.
        // Check for more info: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
        unsafe { ShowWindow(self.hwnd, windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD(1)); };
    }

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

        let get_result: BOOL = unsafe { GetMessageA(&mut self.msg_buffer, None, 0, 0) };
        if !(get_result.0 > 0) {
            self.last_result = get_result;
            return Err((self.msg_buffer.to_owned(), get_result.0))
        }
        
        unsafe { TranslateMessage(&mut self.msg_buffer) };
        unsafe { DispatchMessageA(&mut self.msg_buffer) };

        Ok(self.msg_buffer.to_owned())
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
                WM_KILLFOCUS => {
                    io::KEYBOARD.reset();
                    LRESULT(0)
                }
                WM_CLOSE => {
                    println!("WM_CLOSE");
                    DestroyWindow(hwnd);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(69);
                    LRESULT(0)   
                }
                WM_CHAR => {
                    io::KEYBOARD.on_char(wparam.0 as u32);
                    LRESULT(0)
                }
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    // See https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#keystroke-message-flags
                    let auto_repeat: bool = (lparam.0 >> 30) & 1 == 1;

                    if auto_repeat {
                        io::KEYBOARD.enable_auto_repeat();
                    }
                    io::KEYBOARD.on_key_press(wparam.0 as u32);

                    return LRESULT(0);
                }
                WM_KEYUP | WM_SYSKEYUP => {
                    io::KEYBOARD.disable_auto_repeat();
                    io::KEYBOARD.on_key_release(wparam.0 as u32);
                    
                    LRESULT(0)
                }
                _ => {
                    
                    return DefWindowProcA(hwnd, msg, wparam, lparam);
                }
            }
            // println!("{}: {:?}", crate::window::message::_id_to_name(msg), wparam.0);
            
        }
    }

    pub fn get_exit_codes(&self) {
        /*
            Computate the WIN32_ERROR to the description of the error. For more info check: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes
        */
        
        // To test this function you could uncomment this piece of code and see what it returns.
        // unsafe { SetLastError(windows::Win32::Foundation::WIN32_ERROR(5)) };
        
        // If you want to see what langid it would use if you set dwLanguageId to 0, uncomment this:
        // println!("{}", unsafe {windows::Win32::Globalization::GetUserDefaultLangID()});

        let err_code: u32 = unsafe { GetLastError().0 }; // Get the last WIN32_ERROR and get the id from it (u32)
        let mut err_buffer: *mut u8 = std::ptr::null_mut(); // Create a buffer for windows where it should store the error message
        if err_code == 0 {
            // If the error code == 0, there is no error. So there is no need for priting a succes error :)
            return println!("Succesfull exit with codes: last getResult: {:?}, wParam: {}", self.last_result.0, self.msg_buffer.wParam.0)
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
                0, // LCID (language code identifier) ->
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
                    
                None 
                /*
                    An array of values that are used as insert values in the formatted message.
                    For more info see: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage#parameters
                */
            )
            
        };
        
        if err_msg_lenght == 0 { // If the message buffer is empty, there is no available error description
            /*
                Could be caused by an invalid error code or an invalid or not correctly installed LCID
            */
            
            return println!("Unable to find error description. Errorcode: {}", err_code);
        }
        
        // If there is an error, print all the return codes,
        println!("Unsuccesfull exit with codes getResult: {:?}, wParam: {}, lastError: {}", self.last_result.0, self.msg_buffer.wParam.0, unsafe { GetLastError().0 });
        // and print out the description of the code
        let slice = unsafe { std::slice::from_raw_parts(err_buffer, (err_msg_lenght - 2) as _).to_vec()};
        
        return println!("Errorcode {}: {:?}", err_code, String::from_utf8(slice).unwrap());
        
    }

}
