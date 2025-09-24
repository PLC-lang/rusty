use num::{Bounded, Integer, One, Zero};

use crate::utils::Signal;

#[repr(C)]
#[derive(Debug, Default)]
pub struct CTUParams<T> {
    __vtable: usize,
    cu: bool,
    r: bool,
    pv: T,
    q: bool,
    cv: T,
    internal: Signal,
}

#[repr(C)]
pub struct VTableCTU<T> {
    body: extern "C" fn(&mut CTUParams<T>),
}

impl<T> CTUParams<T>
where
    T: Integer + Copy,
{
    fn update_q(&mut self) {
        self.q = self.cv >= self.pv
    }

    fn reset(&mut self) {
        self.cv = Zero::zero()
    }

    fn inc(&mut self) {
        self.cv = self.cv + One::one();
    }

    fn r_edge(&mut self) -> bool {
        self.internal.rising_edge(self.cu)
    }
}

fn ctu<T>(params: &mut CTUParams<T>)
where
    T: Integer + Copy + Bounded,
{
    if params.r {
        params.reset();
    } else if params.r_edge() & (params.cv < T::max_value()) {
        params.inc();
    }
    params.update_q();
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU: VTableCTU<i16> = VTableCTU { body: CTU };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU__init: CTUParams<i16> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU(params: &mut CTUParams<i16>) {
    ctu(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU_INT: VTableCTU<i16> = VTableCTU { body: CTU_INT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU_INT__init: CTUParams<i16> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU_INT(params: &mut CTUParams<i16>) {
    ctu(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU_DINT: VTableCTU<i32> = VTableCTU { body: CTU_DINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU_DINT__init: CTUParams<i32> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU_DINT(params: &mut CTUParams<i32>) {
    ctu(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU_UDINT: VTableCTU<u32> = VTableCTU { body: CTU_UDINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU_UDINT__init: CTUParams<u32> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU_UDINT(params: &mut CTUParams<u32>) {
    ctu(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU_LINT: VTableCTU<i64> = VTableCTU { body: CTU_LINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU_LINT__init: CTUParams<i64> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU_LINT(params: &mut CTUParams<i64>) {
    ctu(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTU_ULINT: VTableCTU<u64> = VTableCTU { body: CTU_ULINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTU_ULINT__init: CTUParams<u64> = CTUParams {
    __vtable: 0,
    cu: false,
    r: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter up for ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTU_ULINT(params: &mut CTUParams<u64>) {
    ctu(params);
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct CTDParams<T> {
    __vtable: usize,
    cd: bool,
    ld: bool,
    pv: T,
    q: bool,
    cv: T,
    internal: Signal,
}

#[repr(C)]
pub struct VTableCTD<T> {
    body: extern "C" fn(&mut CTDParams<T>),
}

impl<T> CTDParams<T>
where
    T: Integer + Copy,
{
    fn update_q(&mut self) {
        self.q = self.cv <= Zero::zero()
    }

    fn load(&mut self) {
        self.cv = self.pv
    }

    fn dec(&mut self) {
        self.cv = self.cv - One::one();
    }

    fn r_edge(&mut self) -> bool {
        self.internal.rising_edge(self.cd)
    }
}

fn ctd<T>(params: &mut CTDParams<T>)
where
    T: Integer + Copy + Bounded,
{
    if params.ld {
        params.load();
    } else if params.r_edge() & (params.cv > T::min_value()) {
        params.dec();
    }
    params.update_q();
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD: VTableCTD<i16> = VTableCTD { body: CTD };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD__init: CTDParams<i16> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD(params: &mut CTDParams<i16>) {
    ctd(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD_INT: VTableCTD<i16> = VTableCTD { body: CTD_INT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD_INT__init: CTDParams<i16> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD_INT(params: &mut CTDParams<i16>) {
    ctd(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD_DINT: VTableCTD<i32> = VTableCTD { body: CTD_DINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD_DINT__init: CTDParams<i32> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD_DINT(params: &mut CTDParams<i32>) {
    ctd(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD_UDINT: VTableCTD<u32> = VTableCTD { body: CTD_UDINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD_UDINT__init: CTDParams<u32> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD_UDINT(params: &mut CTDParams<u32>) {
    ctd(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD_LINT: VTableCTD<i64> = VTableCTD { body: CTD_LINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD_LINT__init: CTDParams<i64> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD_LINT(params: &mut CTDParams<i64>) {
    ctd(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTD_ULINT: VTableCTD<u64> = VTableCTD { body: CTD_ULINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTD_ULINT__init: CTDParams<u64> = CTDParams {
    __vtable: 0,
    cd: false,
    ld: false,
    pv: 0,
    q: false,
    cv: 0,
    internal: Signal { current_value: false },
};

///.
/// Counter down for ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTD_ULINT(params: &mut CTDParams<u64>) {
    ctd(params);
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct CTUDParams<T> {
    __vtable: usize,
    cu: bool,
    cd: bool,
    r: bool,
    ld: bool,
    pv: T,
    qu: bool,
    qd: bool,
    cv: T,
    internal_up: Signal,
    internal_down: Signal,
}

#[repr(C)]
pub struct VTableCTUD<T> {
    body: extern "C" fn(&mut CTUDParams<T>),
}

impl<T> CTUDParams<T>
where
    T: Integer + Copy,
{
    fn update_qu(&mut self) {
        self.qu = self.cv >= self.pv
    }

    fn update_qd(&mut self) {
        self.qd = self.cv <= Zero::zero()
    }

    fn reset(&mut self) {
        self.cv = Zero::zero()
    }

    fn load(&mut self) {
        self.cv = self.pv
    }

    fn inc(&mut self) {
        self.cv = self.cv + One::one();
    }

    fn dec(&mut self) {
        self.cv = self.cv - One::one();
    }

    fn cu_r_edge(&mut self) -> bool {
        self.internal_up.rising_edge(self.cu)
    }

    fn cd_r_edge(&mut self) -> bool {
        self.internal_down.rising_edge(self.cd)
    }
}

fn ctud<T>(params: &mut CTUDParams<T>)
where
    T: Integer + Copy + Bounded,
{
    if params.r {
        params.reset();
    } else if params.ld {
        params.load();
    } else {
        let r_edge_up = params.cu_r_edge();
        let r_edge_down = params.cd_r_edge();
        if !(r_edge_up & r_edge_down) {
            if r_edge_up & (params.cv < T::max_value()) {
                params.inc();
            } else if r_edge_down & (params.cv > T::min_value()) {
                params.dec();
            }
        }
    }
    params.update_qu();
    params.update_qd();
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD: VTableCTUD<i16> = VTableCTUD { body: CTUD };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD__init: CTUDParams<i16> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD(params: &mut CTUDParams<i16>) {
    ctud(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD_INT: VTableCTUD<i16> = VTableCTUD { body: CTUD_INT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD_INT__init: CTUDParams<i16> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD_INT(params: &mut CTUDParams<i16>) {
    ctud(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD_DINT: VTableCTUD<i32> = VTableCTUD { body: CTUD_DINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD_DINT__init: CTUDParams<i32> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD_DINT(params: &mut CTUDParams<i32>) {
    ctud(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD_UDINT: VTableCTUD<u32> = VTableCTUD { body: CTUD_UDINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD_UDINT__init: CTUDParams<u32> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD_UDINT(params: &mut CTUDParams<u32>) {
    ctud(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD_LINT: VTableCTUD<i64> = VTableCTUD { body: CTUD_LINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD_LINT__init: CTUDParams<i64> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD_LINT(params: &mut CTUDParams<i64>) {
    ctud(params);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_CTUD_ULINT: VTableCTUD<u64> = VTableCTUD { body: CTUD_ULINT };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __CTUD_ULINT__init: CTUDParams<u64> = CTUDParams {
    __vtable: 0,
    cu: false,
    cd: false,
    r: false,
    ld: false,
    pv: 0,
    qu: false,
    qd: false,
    cv: 0,
    internal_up: Signal { current_value: false },
    internal_down: Signal { current_value: false },
};

///.
/// Counter up and down for ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CTUD_ULINT(params: &mut CTUDParams<u64>) {
    ctud(params);
}
