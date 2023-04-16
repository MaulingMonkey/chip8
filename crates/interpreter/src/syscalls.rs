pub trait Syscalls {
    fn rand(&self) -> u8;
    fn get_key(&self) -> Option<u8>;
    fn is_pressed(&self, key: u8) -> bool;
    fn sound_play(&self);
    fn sound_stop(&self);
}

#[cfg(feature = "default-syscalls")] impl Syscalls for () {
    fn rand(&self) -> u8 { rand::random() }
    fn get_key(&self) -> Option<u8> { None }
    fn is_pressed(&self, _key: u8) -> bool { false }
    fn sound_play(&self) {}
    fn sound_stop(&self) {}
}
