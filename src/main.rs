#![windows_subsystem = "windows"]

use winapi::um::winuser::{
    EnumDisplayMonitors, GetMonitorInfoW, MONITORINFO, 
    SetCursorPos,GetAsyncKeyState, VK_CONTROL, VK_END};
use rand::Rng;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::shared::windef::{HMONITOR, LPRECT, RECT};
use winapi::shared::minwindef::BOOL;
use rand::seq::IndexedRandom;
use std::time::Duration;

fn main() {

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Getting info about monitors
    let monitors = get_monitors_info();

    let handle = thread::spawn(move || {
        while r.load(Ordering::Relaxed) {
            let mut rng = rand::rng();
            if let Some(monitor) = monitors.as_slice().choose(&mut rng) {
            
                let rnd_x = rand::rng().random_range(monitor.left..=monitor.right);
                let rnd_y = rand::rng().random_range(monitor.top..=monitor.bottom);
                let rnd_timeout = rand::rng().random_range(1..=10);

                move_cursor(rnd_x, rnd_y);
                thread::sleep(std::time::Duration::from_secs(rnd_timeout));
            }
        }
    });

    // Waiting for Ctrl+End to close the program

    loop {
    
        unsafe {
            let ctrl_pressed = GetAsyncKeyState(VK_CONTROL) as u16 & 0x8000 != 0;
            let end_pressed = GetAsyncKeyState(VK_END) as u16 & 0x8000 != 0;
            
            if ctrl_pressed && end_pressed {
                println!("Обнаружено Ctrl+End. Выход...");
                running.store(false, Ordering::Relaxed);
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    //Waiting for close thread
    handle.join().unwrap();      
}

fn move_cursor(max_x: i32, max_y: i32) {
   
    unsafe {
        SetCursorPos(max_x, max_y); // Move
    }
}

// getting information about all monitors
#[derive(Debug)]
struct MonitorArea {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

fn get_monitors_info() -> Vec<MonitorArea> {
    
    unsafe extern "system" fn callback(
        hmonitor: HMONITOR,
        _hdc: winapi::shared::windef::HDC,
        _rect: LPRECT,
        _lparam: winapi::shared::minwindef::LPARAM,
    ) -> BOOL {
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            rcWork: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwFlags: 0,
        };

        unsafe {GetMonitorInfoW(hmonitor, &mut info);}

        let monitor = MonitorArea {
            left: info.rcMonitor.left,
            top: info.rcMonitor.top,
            right: info.rcMonitor.right - 1,
            bottom: info.rcMonitor.bottom - 1,
        };

        let monitors_ptr = _lparam as *mut Vec<MonitorArea>;
        
        unsafe{(*monitors_ptr).push(monitor);}

        1 // Continue
    }

    let mut monitors_vec = Vec::new();
    unsafe {
        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            Some(callback),
            &mut monitors_vec as *mut Vec<MonitorArea> as winapi::shared::minwindef::LPARAM,
        );
    }

    monitors_vec

}

