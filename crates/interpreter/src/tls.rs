//! Thread Local Storage
//!
//! High level static/tls entry points and lifetime management.

use crate::*;

use instant::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Read;
use std::marker::PhantomData;

const IDS_BEFORE_REUSE  : usize     = 1000; // Allocate this many IDs before reusing a previously allocated ID (better detection of UAF bugs)
const MAX_STEP          : Duration  = Duration::from_secs(1);
const INSTRUCTION_HZ    : u16       = 500; // instructions per second
const CLOCK_HZ          : u16       = 60; // clock steps per second



/// A [`Context`] identifier unique for this thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ContextId(usize, PhantomData<*const ()>); // !Send, !Sync
impl ContextId { pub const fn new(id: usize) -> Self { Self(id, PhantomData) } }

/// Use `syscalls` for all <code>[tls]::*</code> contexts
pub fn set_syscalls_static(syscalls: &'static dyn Syscalls) {
    SYSCALLS.with(|sc| *sc.borrow_mut() = syscalls);
}

/// Create a new [`Context`] and return an opaque identifier for it.
pub fn create_context(program: impl Read) -> ContextId {
    let mut ctx = Context::default();
    ctx.registers.pc = Addr::PROGRAM_START_TYPICAL;
    ctx.memory.copy_from_io(ctx.registers.pc, program).expect("failed to copy ROM into memory");
    ctx.memory.copy_from_slice(Addr::TYPICAL_FONTS_START, bytemuck::cast_slice(font::DEFAULT)).expect("failed to copy font into memory"); // ≈ pointless?

    ContextId::new(TLS.with(|tls| {
        let mut tls = tls.borrow_mut();
        if let Some(ContextId(idx, _)) = (tls.contexts_free_list.len() > IDS_BEFORE_REUSE).then(|| tls.contexts_free_list.pop_front()).flatten() {
            tls.contexts[idx].replace(ctx);
            idx
        } else {
            let idx = tls.contexts.len();
            tls.contexts.push(Some(ctx));
            idx
        }
    }))
}

/// Destroy a [`Context`] by `id`.  Panics if that context was already destroyed.
pub fn destroy_context(id: ContextId) {
    TLS.with(|tls| {
        let mut tls = tls.borrow_mut();
        let ctx = tls.contexts.get_mut(id.0).and_then(|c| c.take()).expect("context doesn't exist or already destroyed");
        tls.contexts_free_list.push_back(id);
        ctx
    });
}

/// Update all [`tls`]-owned [`Context`]s for this thread.
pub fn update() {
    let now = Instant::now();
    TLS.with(move |tls| {
        let mut tls = tls.borrow_mut();

        // Clamp time(s)
        if let Some(min) = tls.next_step.checked_sub(MAX_STEP) { tls.next_step = tls.next_step.max(min); }

        // Step logic
        while now >= tls.next_step {
            for ctx in tls.contexts.iter_mut().flatten() {
                ctx.try_step_many((INSTRUCTION_HZ/CLOCK_HZ).into());
                ctx.step_clocks();
            }
            tls.next_step += Duration::from_millis((1000/CLOCK_HZ).into());
        }
    });
}



thread_local! {
    static TLS      : RefCell<Tls> = Default::default();
    static SYSCALLS : RefCell<&'static dyn Syscalls> = RefCell::new(&PanicSyscalls);
}

struct Tls {
    contexts:           Vec<Option<Context<TlsSyscalls>>>,
    contexts_free_list: VecDeque<ContextId>,
    next_step:          Instant,
}

impl Default for Tls {
    fn default() -> Self {
        Self {
            contexts:               Default::default(),
            contexts_free_list:     Default::default(),
            next_step:              Instant::now(),
        }
    }
}

fn panic() -> ! { panic!("tls::* invoked syscalls before tls::set_syscalls* was called") }

#[derive(Default)] struct PanicSyscalls;
impl Syscalls for PanicSyscalls {
    fn get_key(&self) -> Option<u8>                 { panic() }
    fn is_pressed(&self, _key: u8) -> bool          { panic() }
    fn rand(&self) -> u8                            { panic() }
    fn sound_play(&self)                            { panic() }
    fn sound_stop(&self)                            { panic() }
    fn render(&self, _: &ScreenMonochrome64x32)     { panic() }
}

#[derive(Default)] struct TlsSyscalls;
impl Syscalls for TlsSyscalls {
    fn get_key(&self) -> Option<u8>                     { SYSCALLS.with(|sc| sc.borrow().get_key()) }
    fn is_pressed(&self, key: u8) -> bool               { SYSCALLS.with(|sc| sc.borrow().is_pressed(key)) }
    fn rand(&self) -> u8                                { SYSCALLS.with(|sc| sc.borrow().rand()) }
    fn sound_play(&self)                                { SYSCALLS.with(|sc| sc.borrow().sound_play()) }
    fn sound_stop(&self)                                { SYSCALLS.with(|sc| sc.borrow().sound_stop()) }
    fn render(&self, screen: &ScreenMonochrome64x32)    { SYSCALLS.with(|sc| sc.borrow().render(screen)) }
}
