#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod player {
    #[ink(storage)]
    pub struct Player {
        x: u32,
        y: u32,
    }
    #[ink(event)]
    pub struct NewPoint {
        x: u32,
        y: u32,
    }
    impl Player {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { x: 0, y: 0 }
        }
        #[ink(message, selector = 0)]
        pub fn your_turn(&mut self) -> Option<(u32, u32)> {
            let old_x = self.x;
            let old_y = self.y;
            if self.x == 25 {
                self.x = 0;
                if self.y == 25 {
                    self.y = 0;
                } else {
                    self.y += 1;
                }
            } else {
                self.x += 1;
            }
            self.env().emit_event(NewPoint { x: old_x, y: old_y });
            Some((old_x, old_y))
        }
    }
}
