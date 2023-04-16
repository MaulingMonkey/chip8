pub trait Syscalls {
    fn rand(&self) -> u8;
    fn get_key(&self) -> Option<u8>;
}

#[cfg(feature = "default-syscalls")] impl Syscalls for () {
    fn rand(&self) -> u8 { rand::random() }
    fn get_key(&self) -> Option<u8> { None }
}
