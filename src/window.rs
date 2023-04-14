use windows::{Win32::{UI::{WindowsAndMessaging::{DefWindowProcA, CreateWindowExA, WS_CAPTION, WS_SYSMENU, ShowWindow, LoadCursorW, IDC_ARROW, WNDCLASSEXA, RegisterClassExA, WM_CLOSE, PostQuitMessage, WS_MINIMIZEBOX, HICON, WM_DESTROY, DestroyWindow, WNDCLASS_STYLES, MSG, GetMessageA, TranslateMessage, DispatchMessageA}}, Foundation::{HWND, WPARAM, LPARAM, LRESULT, HINSTANCE, BOOL, GetLastError}, System::{LibraryLoader::{GetModuleHandleA}}, Graphics::Gdi::HBRUSH}, core::{PCSTR}};
pub struct Window {
    pub instance: HINSTANCE,
    pub class_name: PCSTR,
    pub atom: u16,
    pub class: WNDCLASSEXA,
    pub hwnd: HWND,
    pub msg_buffer: MSG,
    last_result: BOOL
}

impl Window {
    pub fn new(class_name: PCSTR, style: WNDCLASS_STYLES) -> Window {
        let class_name: PCSTR = class_name; // id of the program
        
        let instance: HINSTANCE = unsafe { 
            /* 
                hInstance is the handle to an instance or handle to a module. The 
                operating system uses this value to identify the executable or EXE 
                when it's loaded in memory.
             */
            GetModuleHandleA(None).unwrap_or_else(|_| {
                panic!("unable to get module handle")
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
                panic!("unable to load cursor")
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

                more info: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa
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

                More info: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
             */
            CreateWindowExA(windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            class_name, class_name, 
            WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
            200, 200, 896, 672,
            None, None, instance, None) 
        };
        // return the new Window instance
        Window { instance, class_name, atom, class, hwnd, msg_buffer: MSG::default(), last_result: BOOL::default() }

    }

    pub fn show_window(&self) {
        // Sets the specified window's show state.
        // Check for more info: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
        unsafe { ShowWindow(self.hwnd, windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD(1)); };
    }

    pub fn handle_message(&mut self) -> Option<(MSG, i32)> {
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
            return Some((self.msg_buffer.to_owned(), get_result.0))
        }
        
        unsafe { TranslateMessage(&mut self.msg_buffer) };
        unsafe { DispatchMessageA(&mut self.msg_buffer) };
        None
    }

    pub fn get_exit_codes(&self) {
        println!("exit with codes getResult: {:?}, wParam: {}, lastError: {}", self.last_result.0, self.msg_buffer.wParam.0, unsafe { GetLastError().0 });
    }

    extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        /*

            It is very hard to explain how this works without typing a lot of text so i'll just refer you to
            the great video by ChiliTomatoNoodle (https://youtu.be/UUbXK4G_NCM). It explains how the window
            messages work and how to build a good system around it. 

            For more info about wndproc see: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc
            And for a list with all the messages see: https://wiki.winehq.org/List_Of_Windows_Messages 

         */
        
        unsafe {
            match message {
                WM_CLOSE => {
                    println!("WM_CLOSE");
                    DestroyWindow(hwnd);
                    LRESULT(0)
                },
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(69);
                    LRESULT(0)
                },
                _ => {
                    DefWindowProcA(hwnd, message, wparam, lparam)
                },
            }
        }
    }
}