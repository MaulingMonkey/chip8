pub trait Syscalls {
    fn rand(&self) -> u8;
}

#[cfg(feature = "default-syscalls")] impl Syscalls for () {
    fn rand(&self) -> u8 { rand::random() }
}
