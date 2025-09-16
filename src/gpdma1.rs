#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    pub PRIVCFGR: PRIVCFGR,
    _reserved1: [u8; 0x04],
    pub MISR: MISR,
    _reserved2: [u8; 0x40],
    _reserved_2_C: [u8; 0x0400],
}
impl RegisterBlock {
    #[doc = "0x04 - GPDMA privileged configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
    #[doc = "0x0c - GPDMA masked interrupt status register"]
    #[inline(always)]
    pub const fn MISR(&self) -> &MISR {
        &self.MISR
    }
    #[doc = "0x50..0x450 - Cluster for C\\[%s\\]"]
    #[inline(always)]
    pub const fn C(&self, n: usize) -> &C {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x450 - Cluster for C\\[%s\\]"]
    #[inline(always)]
    pub fn C_iter(&self) -> impl Iterator<Item = &C> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x5c - GPDMA channel 0 flag clear register"]
    #[inline(always)]
    pub const fn C0FCR(&self) -> &C0FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - GPDMA channel 0 status register"]
    #[inline(always)]
    pub const fn C0SR(&self) -> &C0SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x64 - GPDMA channel 0 control register"]
    #[inline(always)]
    pub const fn C0CR(&self) -> &C0CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0xdc - GPDMA channel 1 flag clear register"]
    #[inline(always)]
    pub const fn C1FCR(&self) -> &C1FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xe0 - GPDMA channel 1 status register"]
    #[inline(always)]
    pub const fn C1SR(&self) -> &C1SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe4 - GPDMA channel 1 control register"]
    #[inline(always)]
    pub const fn C1CR(&self) -> &C1CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0x15c - GPDMA channel 2 flag clear register"]
    #[inline(always)]
    pub const fn C2FCR(&self) -> &C2FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x160 - GPDMA channel 2 status register"]
    #[inline(always)]
    pub const fn C2SR(&self) -> &C2SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x164 - GPDMA channel 2 control register"]
    #[inline(always)]
    pub const fn C2CR(&self) -> &C2CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x1dc - GPDMA channel 3 flag clear register"]
    #[inline(always)]
    pub const fn C3FCR(&self) -> &C3FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1e0 - GPDMA channel 3 status register"]
    #[inline(always)]
    pub const fn C3SR(&self) -> &C3SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e4 - GPDMA channel 3 control register"]
    #[inline(always)]
    pub const fn C3CR(&self) -> &C3CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x25c - GPDMA channel 4 flag clear register"]
    #[inline(always)]
    pub const fn C4FCR(&self) -> &C4FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x260 - GPDMA channel 4 status register"]
    #[inline(always)]
    pub const fn C4SR(&self) -> &C4SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x264 - GPDMA channel 4 control register"]
    #[inline(always)]
    pub const fn C4CR(&self) -> &C4CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x2dc - GPDMA channel 5 flag clear register"]
    #[inline(always)]
    pub const fn C5FCR(&self) -> &C5FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(732).cast() }
    }
    #[doc = "0x2e0 - GPDMA channel 5 status register"]
    #[inline(always)]
    pub const fn C5SR(&self) -> &C5SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(736).cast() }
    }
    #[doc = "0x2e4 - GPDMA channel 5 control register"]
    #[inline(always)]
    pub const fn C5CR(&self) -> &C5CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(740).cast() }
    }
    #[doc = "0x35c - GPDMA channel 6 flag clear register"]
    #[inline(always)]
    pub const fn C6FCR(&self) -> &C6FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(860).cast() }
    }
    #[doc = "0x360 - GPDMA channel 6 status register"]
    #[inline(always)]
    pub const fn C6SR(&self) -> &C6SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(864).cast() }
    }
    #[doc = "0x364 - GPDMA channel 6 control register"]
    #[inline(always)]
    pub const fn C6CR(&self) -> &C6CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(868).cast() }
    }
    #[doc = "0x3a4 - GPDMA channel 6 transfer register 3"]
    #[inline(always)]
    pub const fn C6TR3(&self) -> &C6TR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(932).cast() }
    }
    #[doc = "0x3a8 - GPDMA channel 6 block register 2"]
    #[inline(always)]
    pub const fn C6BR2(&self) -> &C6BR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(936).cast() }
    }
    #[doc = "0x3dc - GPDMA channel 7 flag clear register"]
    #[inline(always)]
    pub const fn C7FCR(&self) -> &C7FCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(988).cast() }
    }
    #[doc = "0x3e0 - GPDMA channel 7 status register"]
    #[inline(always)]
    pub const fn C7SR(&self) -> &C7SR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(992).cast() }
    }
    #[doc = "0x3e4 - GPDMA channel 7 control register"]
    #[inline(always)]
    pub const fn C7CR(&self) -> &C7CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(996).cast() }
    }
    #[doc = "0x424 - GPDMA channel 7 transfer register 3"]
    #[inline(always)]
    pub const fn C7TR3(&self) -> &C7TR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1060).cast() }
    }
    #[doc = "0x428 - GPDMA channel 7 block register 2"]
    #[inline(always)]
    pub const fn C7BR2(&self) -> &C7BR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1064).cast() }
    }
}
#[doc = "PRIVCFGR (rw) register accessor: GPDMA privileged configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "GPDMA privileged configuration register"]
pub mod privcfgr;
#[doc = "MISR (r) register accessor: GPDMA masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "GPDMA masked interrupt status register"]
pub mod misr;
#[doc = "Cluster for C\\[%s\\]"]
pub use self::c::C;
#[doc = r"Cluster"]
#[doc = "Cluster for C\\[%s\\]"]
pub mod c;
#[doc = "C0FCR (w) register accessor: GPDMA channel 0 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0fcr`] module"]
pub type C0FCR = crate::Reg<c0fcr::C0FCR_SPEC>;
#[doc = "GPDMA channel 0 flag clear register"]
pub mod c0fcr;
#[doc = "C0SR (r) register accessor: GPDMA channel 0 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0sr`] module"]
pub type C0SR = crate::Reg<c0sr::C0SR_SPEC>;
#[doc = "GPDMA channel 0 status register"]
pub mod c0sr;
#[doc = "C0CR (rw) register accessor: GPDMA channel 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cr`] module"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "GPDMA channel 0 control register"]
pub mod c0cr;
#[doc = "C1FCR (w) register accessor: GPDMA channel 1 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1fcr`] module"]
pub type C1FCR = crate::Reg<c1fcr::C1FCR_SPEC>;
#[doc = "GPDMA channel 1 flag clear register"]
pub mod c1fcr;
#[doc = "C1SR (r) register accessor: GPDMA channel 1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1sr`] module"]
pub type C1SR = crate::Reg<c1sr::C1SR_SPEC>;
#[doc = "GPDMA channel 1 status register"]
pub mod c1sr;
#[doc = "C1CR (rw) register accessor: GPDMA channel 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cr`] module"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "GPDMA channel 1 control register"]
pub mod c1cr;
#[doc = "C2FCR (w) register accessor: GPDMA channel 2 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2fcr`] module"]
pub type C2FCR = crate::Reg<c2fcr::C2FCR_SPEC>;
#[doc = "GPDMA channel 2 flag clear register"]
pub mod c2fcr;
#[doc = "C2SR (r) register accessor: GPDMA channel 2 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2sr`] module"]
pub type C2SR = crate::Reg<c2sr::C2SR_SPEC>;
#[doc = "GPDMA channel 2 status register"]
pub mod c2sr;
#[doc = "C2CR (rw) register accessor: GPDMA channel 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`] module"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "GPDMA channel 2 control register"]
pub mod c2cr;
#[doc = "C3FCR (w) register accessor: GPDMA channel 3 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3fcr`] module"]
pub type C3FCR = crate::Reg<c3fcr::C3FCR_SPEC>;
#[doc = "GPDMA channel 3 flag clear register"]
pub mod c3fcr;
#[doc = "C3SR (r) register accessor: GPDMA channel 3 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3sr`] module"]
pub type C3SR = crate::Reg<c3sr::C3SR_SPEC>;
#[doc = "GPDMA channel 3 status register"]
pub mod c3sr;
#[doc = "C3CR (rw) register accessor: GPDMA channel 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3cr`] module"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "GPDMA channel 3 control register"]
pub mod c3cr;
#[doc = "C4FCR (w) register accessor: GPDMA channel 4 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4fcr`] module"]
pub type C4FCR = crate::Reg<c4fcr::C4FCR_SPEC>;
#[doc = "GPDMA channel 4 flag clear register"]
pub mod c4fcr;
#[doc = "C4SR (r) register accessor: GPDMA channel 4 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4sr`] module"]
pub type C4SR = crate::Reg<c4sr::C4SR_SPEC>;
#[doc = "GPDMA channel 4 status register"]
pub mod c4sr;
#[doc = "C4CR (rw) register accessor: GPDMA channel 4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4cr`] module"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "GPDMA channel 4 control register"]
pub mod c4cr;
#[doc = "C5FCR (w) register accessor: GPDMA channel 5 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5fcr`] module"]
pub type C5FCR = crate::Reg<c5fcr::C5FCR_SPEC>;
#[doc = "GPDMA channel 5 flag clear register"]
pub mod c5fcr;
#[doc = "C5SR (r) register accessor: GPDMA channel 5 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5sr`] module"]
pub type C5SR = crate::Reg<c5sr::C5SR_SPEC>;
#[doc = "GPDMA channel 5 status register"]
pub mod c5sr;
#[doc = "C5CR (rw) register accessor: GPDMA channel 5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5cr`] module"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "GPDMA channel 5 control register"]
pub mod c5cr;
#[doc = "C6FCR (w) register accessor: GPDMA channel 6 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6fcr`] module"]
pub type C6FCR = crate::Reg<c6fcr::C6FCR_SPEC>;
#[doc = "GPDMA channel 6 flag clear register"]
pub mod c6fcr;
#[doc = "C6SR (r) register accessor: GPDMA channel 6 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6sr`] module"]
pub type C6SR = crate::Reg<c6sr::C6SR_SPEC>;
#[doc = "GPDMA channel 6 status register"]
pub mod c6sr;
#[doc = "C6CR (rw) register accessor: GPDMA channel 6 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6cr`] module"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "GPDMA channel 6 control register"]
pub mod c6cr;
#[doc = "C6TR3 (rw) register accessor: GPDMA channel 6 transfer register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c6tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6tr3`] module"]
pub type C6TR3 = crate::Reg<c6tr3::C6TR3_SPEC>;
#[doc = "GPDMA channel 6 transfer register 3"]
pub mod c6tr3;
#[doc = "C6BR2 (rw) register accessor: GPDMA channel 6 block register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c6br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6br2`] module"]
pub type C6BR2 = crate::Reg<c6br2::C6BR2_SPEC>;
#[doc = "GPDMA channel 6 block register 2"]
pub mod c6br2;
#[doc = "C7FCR (w) register accessor: GPDMA channel 7 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7fcr`] module"]
pub type C7FCR = crate::Reg<c7fcr::C7FCR_SPEC>;
#[doc = "GPDMA channel 7 flag clear register"]
pub mod c7fcr;
#[doc = "C7SR (r) register accessor: GPDMA channel 7 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7sr`] module"]
pub type C7SR = crate::Reg<c7sr::C7SR_SPEC>;
#[doc = "GPDMA channel 7 status register"]
pub mod c7sr;
#[doc = "C7CR (rw) register accessor: GPDMA channel 7 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7cr`] module"]
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
#[doc = "GPDMA channel 7 control register"]
pub mod c7cr;
#[doc = "C7TR3 (rw) register accessor: GPDMA channel 7 transfer register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c7tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7tr3`] module"]
pub type C7TR3 = crate::Reg<c7tr3::C7TR3_SPEC>;
#[doc = "GPDMA channel 7 transfer register 3"]
pub mod c7tr3;
#[doc = "C7BR2 (rw) register accessor: GPDMA channel 7 block register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c7br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7br2`] module"]
pub type C7BR2 = crate::Reg<c7br2::C7BR2_SPEC>;
#[doc = "GPDMA channel 7 block register 2"]
pub mod c7br2;
