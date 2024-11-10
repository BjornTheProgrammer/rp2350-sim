pub enum NPriv {
    ThreadModePrivileged,
    ThreadModeUnprivileged
}

pub enum SpSel {
    SpMain,
    SpProcess
}

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
