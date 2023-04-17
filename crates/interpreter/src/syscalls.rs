pub trait Syscalls {
    #[cfg(not(feature = "default-syscalls"))]   fn rand(&self) -> u8;
    #[cfg(feature = "default-syscalls")]        fn rand(&self) -> u8 { rand::random() }

    fn get_key(&self) -> Option<u8>;
    fn is_pressed(&self, key: u8) -> bool;
    fn sound_play(&self);
    fn sound_stop(&self);
}

impl Syscalls for () {
    #[cfg(not(feature = "default-syscalls"))] fn rand(&self) -> u8 { 0 }
    fn get_key(&self) -> Option<u8> { None }
    fn is_pressed(&self, _key: u8) -> bool { false }
    fn sound_play(&self) {}
    fn sound_stop(&self) {}
}
