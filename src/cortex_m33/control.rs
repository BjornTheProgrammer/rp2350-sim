#[derive(PartialEq, Debug, Copy, Clone)]
pub enum NPriv {
    ThreadModePrivileged,
    ThreadModeUnprivileged
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SpSel {
    SpMain,
    SpProcess
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Control {
    pub npriv: NPriv,
    pub spsel: SpSel,
}

impl Control {
    pub fn new() -> Self {
        Self {
            npriv: NPriv::ThreadModePrivileged,
            spsel: SpSel::SpMain
        }
    }
}
