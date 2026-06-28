pub mod key;

use std::{cell::RefCell, rc::Rc};

use log::info;
use strum::EnumCount;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{EventTarget, KeyboardEvent, MouseEvent, Window};

use crate::input::key::Key;

#[derive(Debug)]
struct InputState
{
    keys_held: [bool; Key::COUNT],
    keys_down: [bool; Key::COUNT],
    keys_up: [bool; Key::COUNT],
    mouse_pos: (i32, i32),
    mouse_buttons: [bool; 3],
    mouse_delta: (i32, i32),
}

impl InputState
{
    pub fn flush(&mut self)
    {
        self.mouse_delta = (0, 0);
        self.keys_up = [false; Key::COUNT];
        self.keys_down = [false; Key::COUNT];
    }

    pub fn attach_listeners(
        state: Rc<RefCell<Self>>,
        window: &mut Window,
        target: &EventTarget,
    )
    {
        // keydown listener
        {
            let state = state.clone();
            let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
                // ignore any unknown keys
                if let Ok(key) = Key::try_from(e.code().as_str())
                {
                    let mut s = state.borrow_mut();
                    s.keys_down[key as usize] = true;
                    s.keys_held[key as usize] = true;
                }
            });

            window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                .expect("Failed to attach keydown listener");

            info!("Attached keydown listener");
            cb.forget();
        }

        // keyup listener
        {
            let state = state.clone();
            let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
                if let Ok(key) = Key::try_from(e.code().as_str())
                {
                    let mut s = state.borrow_mut();
                    s.keys_up[key as usize] = true;
                    s.keys_held[key as usize] = false;
                }
            });

            window.add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref())
                .expect("Failed to attach keyup listener");

            info!("Attached keyup listener");
            cb.forget();
        }


        // mousemove listener
        {
            let state = state.clone();
            let cb = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
                let mut s = state.borrow_mut();
                s.mouse_delta = (e.movement_x(), e.movement_y());
                s.mouse_pos = (e.offset_x(), e.offset_y());
            });

            window.add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
                .expect("Failed to attach mousemove listener");

            info!("Attached mousemove listener");
            cb.forget();
        }

        // mousedown
        {
            let state = state.clone();
            let cb = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {

                // button: 0=left, 1=middle, 2=right
            });
            target.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        // mouseup
        {
            let state = state.clone();
            let cb = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {

            });
            target.add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }
    }
}
