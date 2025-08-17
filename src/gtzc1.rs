#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    pub TZSC_PRIVCFGR1: TZSC_PRIVCFGR1,
    pub TZSC_PRIVCFGR2: TZSC_PRIVCFGR2,
    pub TZSC_PRIVCFGR3: TZSC_PRIVCFGR3,
    _reserved3: [u8; 0x44],
    pub TZSC_MPCWM4ACFGR: TZSC_MPCWM4ACFGR,
    pub TZSC_MPCWM4AR: TZSC_MPCWM4AR,
    pub TZSC_MPCWM4BCFGR: TZSC_MPCWM4BCFGR,
    pub TZSC_MPCWM4BR: TZSC_MPCWM4BR,
    _reserved7: [u8; 0x0180],
    pub MPCBB1_PRIVCFGR0: MPCBB1_PRIVCFGR0,
    pub MPCBB1_PRIVCFGR1: MPCBB1_PRIVCFGR1,
    pub MPCBB1_PRIVCFGR2: MPCBB1_PRIVCFGR2,
    pub MPCBB1_PRIVCFGR3: MPCBB1_PRIVCFGR3,
    pub MPCBB1_PRIVCFGR4: MPCBB1_PRIVCFGR4,
    pub MPCBB1_PRIVCFGR5: MPCBB1_PRIVCFGR5,
    pub MPCBB1_PRIVCFGR6: MPCBB1_PRIVCFGR6,
    pub MPCBB1_PRIVCFGR7: MPCBB1_PRIVCFGR7,
    pub MPCBB1_PRIVCFGR8: MPCBB1_PRIVCFGR8,
    pub MPCBB1_PRIVCFGR9: MPCBB1_PRIVCFGR9,
    pub MPCBB1_PRIVCFGR10: MPCBB1_PRIVCFGR10,
    pub MPCBB1_PRIVCFGR11: MPCBB1_PRIVCFGR11,
    pub MPCBB1_PRIVCFGR12: MPCBB1_PRIVCFGR12,
    pub MPCBB1_PRIVCFGR13: MPCBB1_PRIVCFGR13,
    pub MPCBB1_PRIVCFGR14: MPCBB1_PRIVCFGR14,
    pub MPCBB1_PRIVCFGR15: MPCBB1_PRIVCFGR15,
    pub MPCBB1_PRIVCFGR16: MPCBB1_PRIVCFGR16,
    pub MPCBB1_PRIVCFGR17: MPCBB1_PRIVCFGR17,
    pub MPCBB1_PRIVCFGR18: MPCBB1_PRIVCFGR18,
    pub MPCBB1_PRIVCFGR19: MPCBB1_PRIVCFGR19,
    pub MPCBB1_PRIVCFGR20: MPCBB1_PRIVCFGR20,
    pub MPCBB1_PRIVCFGR21: MPCBB1_PRIVCFGR21,
    pub MPCBB1_PRIVCFGR22: MPCBB1_PRIVCFGR22,
    pub MPCBB1_PRIVCFGR23: MPCBB1_PRIVCFGR23,
    pub MPCBB1_PRIVCFGR24: MPCBB1_PRIVCFGR24,
    pub MPCBB1_PRIVCFGR25: MPCBB1_PRIVCFGR25,
    pub MPCBB1_PRIVCFGR26: MPCBB1_PRIVCFGR26,
    pub MPCBB1_PRIVCFGR27: MPCBB1_PRIVCFGR27,
    pub MPCBB1_PRIVCFGR28: MPCBB1_PRIVCFGR28,
    pub MPCBB1_PRIVCFGR29: MPCBB1_PRIVCFGR29,
    pub MPCBB1_PRIVCFGR30: MPCBB1_PRIVCFGR30,
    pub MPCBB1_PRIVCFGR31: MPCBB1_PRIVCFGR31,
    _reserved39: [u8; 0x0380],
    pub MPCBB2_PRIVCFGR0: MPCBB2_PRIVCFGR0,
    pub MPCBB2_PRIVCFGR1: MPCBB2_PRIVCFGR1,
    pub MPCBB2_PRIVCFGR2: MPCBB2_PRIVCFGR2,
    pub MPCBB2_PRIVCFGR3: MPCBB2_PRIVCFGR3,
    pub MPCBB2_PRIVCFGR4: MPCBB2_PRIVCFGR4,
    pub MPCBB2_PRIVCFGR5: MPCBB2_PRIVCFGR5,
    pub MPCBB2_PRIVCFGR6: MPCBB2_PRIVCFGR6,
    pub MPCBB2_PRIVCFGR7: MPCBB2_PRIVCFGR7,
    pub MPCBB2_PRIVCFGR8: MPCBB2_PRIVCFGR8,
    pub MPCBB2_PRIVCFGR9: MPCBB2_PRIVCFGR9,
    pub MPCBB2_PRIVCFGR10: MPCBB2_PRIVCFGR10,
    pub MPCBB2_PRIVCFGR11: MPCBB2_PRIVCFGR11,
    pub MPCBB2_PRIVCFGR12: MPCBB2_PRIVCFGR12,
    pub MPCBB2_PRIVCFGR13: MPCBB2_PRIVCFGR13,
    pub MPCBB2_PRIVCFGR14: MPCBB2_PRIVCFGR14,
    pub MPCBB2_PRIVCFGR15: MPCBB2_PRIVCFGR15,
    pub MPCBB2_PRIVCFGR16: MPCBB2_PRIVCFGR16,
    pub MPCBB2_PRIVCFGR17: MPCBB2_PRIVCFGR17,
    pub MPCBB2_PRIVCFGR18: MPCBB2_PRIVCFGR18,
    pub MPCBB2_PRIVCFGR19: MPCBB2_PRIVCFGR19,
    pub MPCBB2_PRIVCFGR20: MPCBB2_PRIVCFGR20,
    pub MPCBB2_PRIVCFGR21: MPCBB2_PRIVCFGR21,
    pub MPCBB2_PRIVCFGR22: MPCBB2_PRIVCFGR22,
    pub MPCBB2_PRIVCFGR23: MPCBB2_PRIVCFGR23,
    pub MPCBB2_PRIVCFGR24: MPCBB2_PRIVCFGR24,
    pub MPCBB2_PRIVCFGR25: MPCBB2_PRIVCFGR25,
    pub MPCBB2_PRIVCFGR26: MPCBB2_PRIVCFGR26,
    pub MPCBB2_PRIVCFGR27: MPCBB2_PRIVCFGR27,
    pub MPCBB2_PRIVCFGR28: MPCBB2_PRIVCFGR28,
    pub MPCBB2_PRIVCFGR29: MPCBB2_PRIVCFGR29,
    pub MPCBB2_PRIVCFGR30: MPCBB2_PRIVCFGR30,
    pub MPCBB2_PRIVCFGR31: MPCBB2_PRIVCFGR31,
}
impl RegisterBlock {
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR1(&self) -> &TZSC_PRIVCFGR1 {
        &self.TZSC_PRIVCFGR1
    }
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR2(&self) -> &TZSC_PRIVCFGR2 {
        &self.TZSC_PRIVCFGR2
    }
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR3(&self) -> &TZSC_PRIVCFGR3 {
        &self.TZSC_PRIVCFGR3
    }
    #[doc = "0x70 - GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4ACFGR(&self) -> &TZSC_MPCWM4ACFGR {
        &self.TZSC_MPCWM4ACFGR
    }
    #[doc = "0x74 - GTZC1 TZSC BKPSRAM sub-region A watermark register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4AR(&self) -> &TZSC_MPCWM4AR {
        &self.TZSC_MPCWM4AR
    }
    #[doc = "0x78 - GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4BCFGR(&self) -> &TZSC_MPCWM4BCFGR {
        &self.TZSC_MPCWM4BCFGR
    }
    #[doc = "0x7c - GTZC1 TZSC BKPSRAM sub-region B watermark register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4BR(&self) -> &TZSC_MPCWM4BR {
        &self.TZSC_MPCWM4BR
    }
    #[doc = "0x200 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR0(&self) -> &MPCBB1_PRIVCFGR0 {
        &self.MPCBB1_PRIVCFGR0
    }
    #[doc = "0x204 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR1(&self) -> &MPCBB1_PRIVCFGR1 {
        &self.MPCBB1_PRIVCFGR1
    }
    #[doc = "0x208 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR2(&self) -> &MPCBB1_PRIVCFGR2 {
        &self.MPCBB1_PRIVCFGR2
    }
    #[doc = "0x20c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR3(&self) -> &MPCBB1_PRIVCFGR3 {
        &self.MPCBB1_PRIVCFGR3
    }
    #[doc = "0x210 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR4(&self) -> &MPCBB1_PRIVCFGR4 {
        &self.MPCBB1_PRIVCFGR4
    }
    #[doc = "0x214 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR5(&self) -> &MPCBB1_PRIVCFGR5 {
        &self.MPCBB1_PRIVCFGR5
    }
    #[doc = "0x218 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR6(&self) -> &MPCBB1_PRIVCFGR6 {
        &self.MPCBB1_PRIVCFGR6
    }
    #[doc = "0x21c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR7(&self) -> &MPCBB1_PRIVCFGR7 {
        &self.MPCBB1_PRIVCFGR7
    }
    #[doc = "0x220 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR8(&self) -> &MPCBB1_PRIVCFGR8 {
        &self.MPCBB1_PRIVCFGR8
    }
    #[doc = "0x224 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR9(&self) -> &MPCBB1_PRIVCFGR9 {
        &self.MPCBB1_PRIVCFGR9
    }
    #[doc = "0x228 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR10(&self) -> &MPCBB1_PRIVCFGR10 {
        &self.MPCBB1_PRIVCFGR10
    }
    #[doc = "0x22c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR11(&self) -> &MPCBB1_PRIVCFGR11 {
        &self.MPCBB1_PRIVCFGR11
    }
    #[doc = "0x230 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR12(&self) -> &MPCBB1_PRIVCFGR12 {
        &self.MPCBB1_PRIVCFGR12
    }
    #[doc = "0x234 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR13(&self) -> &MPCBB1_PRIVCFGR13 {
        &self.MPCBB1_PRIVCFGR13
    }
    #[doc = "0x238 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR14(&self) -> &MPCBB1_PRIVCFGR14 {
        &self.MPCBB1_PRIVCFGR14
    }
    #[doc = "0x23c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR15(&self) -> &MPCBB1_PRIVCFGR15 {
        &self.MPCBB1_PRIVCFGR15
    }
    #[doc = "0x240 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR16(&self) -> &MPCBB1_PRIVCFGR16 {
        &self.MPCBB1_PRIVCFGR16
    }
    #[doc = "0x244 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR17(&self) -> &MPCBB1_PRIVCFGR17 {
        &self.MPCBB1_PRIVCFGR17
    }
    #[doc = "0x248 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR18(&self) -> &MPCBB1_PRIVCFGR18 {
        &self.MPCBB1_PRIVCFGR18
    }
    #[doc = "0x24c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR19(&self) -> &MPCBB1_PRIVCFGR19 {
        &self.MPCBB1_PRIVCFGR19
    }
    #[doc = "0x250 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR20(&self) -> &MPCBB1_PRIVCFGR20 {
        &self.MPCBB1_PRIVCFGR20
    }
    #[doc = "0x254 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR21(&self) -> &MPCBB1_PRIVCFGR21 {
        &self.MPCBB1_PRIVCFGR21
    }
    #[doc = "0x258 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR22(&self) -> &MPCBB1_PRIVCFGR22 {
        &self.MPCBB1_PRIVCFGR22
    }
    #[doc = "0x25c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR23(&self) -> &MPCBB1_PRIVCFGR23 {
        &self.MPCBB1_PRIVCFGR23
    }
    #[doc = "0x260 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR24(&self) -> &MPCBB1_PRIVCFGR24 {
        &self.MPCBB1_PRIVCFGR24
    }
    #[doc = "0x264 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR25(&self) -> &MPCBB1_PRIVCFGR25 {
        &self.MPCBB1_PRIVCFGR25
    }
    #[doc = "0x268 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR26(&self) -> &MPCBB1_PRIVCFGR26 {
        &self.MPCBB1_PRIVCFGR26
    }
    #[doc = "0x26c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR27(&self) -> &MPCBB1_PRIVCFGR27 {
        &self.MPCBB1_PRIVCFGR27
    }
    #[doc = "0x270 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR28(&self) -> &MPCBB1_PRIVCFGR28 {
        &self.MPCBB1_PRIVCFGR28
    }
    #[doc = "0x274 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR29(&self) -> &MPCBB1_PRIVCFGR29 {
        &self.MPCBB1_PRIVCFGR29
    }
    #[doc = "0x278 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR30(&self) -> &MPCBB1_PRIVCFGR30 {
        &self.MPCBB1_PRIVCFGR30
    }
    #[doc = "0x27c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR31(&self) -> &MPCBB1_PRIVCFGR31 {
        &self.MPCBB1_PRIVCFGR31
    }
    #[doc = "0x600 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR0(&self) -> &MPCBB2_PRIVCFGR0 {
        &self.MPCBB2_PRIVCFGR0
    }
    #[doc = "0x604 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR1(&self) -> &MPCBB2_PRIVCFGR1 {
        &self.MPCBB2_PRIVCFGR1
    }
    #[doc = "0x608 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR2(&self) -> &MPCBB2_PRIVCFGR2 {
        &self.MPCBB2_PRIVCFGR2
    }
    #[doc = "0x60c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR3(&self) -> &MPCBB2_PRIVCFGR3 {
        &self.MPCBB2_PRIVCFGR3
    }
    #[doc = "0x610 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR4(&self) -> &MPCBB2_PRIVCFGR4 {
        &self.MPCBB2_PRIVCFGR4
    }
    #[doc = "0x614 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR5(&self) -> &MPCBB2_PRIVCFGR5 {
        &self.MPCBB2_PRIVCFGR5
    }
    #[doc = "0x618 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR6(&self) -> &MPCBB2_PRIVCFGR6 {
        &self.MPCBB2_PRIVCFGR6
    }
    #[doc = "0x61c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR7(&self) -> &MPCBB2_PRIVCFGR7 {
        &self.MPCBB2_PRIVCFGR7
    }
    #[doc = "0x620 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR8(&self) -> &MPCBB2_PRIVCFGR8 {
        &self.MPCBB2_PRIVCFGR8
    }
    #[doc = "0x624 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR9(&self) -> &MPCBB2_PRIVCFGR9 {
        &self.MPCBB2_PRIVCFGR9
    }
    #[doc = "0x628 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR10(&self) -> &MPCBB2_PRIVCFGR10 {
        &self.MPCBB2_PRIVCFGR10
    }
    #[doc = "0x62c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR11(&self) -> &MPCBB2_PRIVCFGR11 {
        &self.MPCBB2_PRIVCFGR11
    }
    #[doc = "0x630 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR12(&self) -> &MPCBB2_PRIVCFGR12 {
        &self.MPCBB2_PRIVCFGR12
    }
    #[doc = "0x634 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR13(&self) -> &MPCBB2_PRIVCFGR13 {
        &self.MPCBB2_PRIVCFGR13
    }
    #[doc = "0x638 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR14(&self) -> &MPCBB2_PRIVCFGR14 {
        &self.MPCBB2_PRIVCFGR14
    }
    #[doc = "0x63c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR15(&self) -> &MPCBB2_PRIVCFGR15 {
        &self.MPCBB2_PRIVCFGR15
    }
    #[doc = "0x640 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR16(&self) -> &MPCBB2_PRIVCFGR16 {
        &self.MPCBB2_PRIVCFGR16
    }
    #[doc = "0x644 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR17(&self) -> &MPCBB2_PRIVCFGR17 {
        &self.MPCBB2_PRIVCFGR17
    }
    #[doc = "0x648 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR18(&self) -> &MPCBB2_PRIVCFGR18 {
        &self.MPCBB2_PRIVCFGR18
    }
    #[doc = "0x64c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR19(&self) -> &MPCBB2_PRIVCFGR19 {
        &self.MPCBB2_PRIVCFGR19
    }
    #[doc = "0x650 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR20(&self) -> &MPCBB2_PRIVCFGR20 {
        &self.MPCBB2_PRIVCFGR20
    }
    #[doc = "0x654 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR21(&self) -> &MPCBB2_PRIVCFGR21 {
        &self.MPCBB2_PRIVCFGR21
    }
    #[doc = "0x658 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR22(&self) -> &MPCBB2_PRIVCFGR22 {
        &self.MPCBB2_PRIVCFGR22
    }
    #[doc = "0x65c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR23(&self) -> &MPCBB2_PRIVCFGR23 {
        &self.MPCBB2_PRIVCFGR23
    }
    #[doc = "0x660 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR24(&self) -> &MPCBB2_PRIVCFGR24 {
        &self.MPCBB2_PRIVCFGR24
    }
    #[doc = "0x664 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR25(&self) -> &MPCBB2_PRIVCFGR25 {
        &self.MPCBB2_PRIVCFGR25
    }
    #[doc = "0x668 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR26(&self) -> &MPCBB2_PRIVCFGR26 {
        &self.MPCBB2_PRIVCFGR26
    }
    #[doc = "0x66c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR27(&self) -> &MPCBB2_PRIVCFGR27 {
        &self.MPCBB2_PRIVCFGR27
    }
    #[doc = "0x670 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR28(&self) -> &MPCBB2_PRIVCFGR28 {
        &self.MPCBB2_PRIVCFGR28
    }
    #[doc = "0x674 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR29(&self) -> &MPCBB2_PRIVCFGR29 {
        &self.MPCBB2_PRIVCFGR29
    }
    #[doc = "0x678 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR30(&self) -> &MPCBB2_PRIVCFGR30 {
        &self.MPCBB2_PRIVCFGR30
    }
    #[doc = "0x67c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR31(&self) -> &MPCBB2_PRIVCFGR31 {
        &self.MPCBB2_PRIVCFGR31
    }
}
#[doc = "TZSC_PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr1`] module"]
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;
#[doc = "TZSC_PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr2`] module"]
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod tzsc_privcfgr2;
#[doc = "TZSC_PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr3`] module"]
pub type TZSC_PRIVCFGR3 = crate::Reg<tzsc_privcfgr3::TZSC_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod tzsc_privcfgr3;
#[doc = "TZSC_MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4acfgr`] module"]
pub type TZSC_MPCWM4ACFGR = crate::Reg<tzsc_mpcwm4acfgr::TZSC_MPCWM4ACFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
pub mod tzsc_mpcwm4acfgr;
#[doc = "TZSC_MPCWM4AR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4ar`] module"]
pub type TZSC_MPCWM4AR = crate::Reg<tzsc_mpcwm4ar::TZSC_MPCWM4AR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register"]
pub mod tzsc_mpcwm4ar;
#[doc = "TZSC_MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4bcfgr`] module"]
pub type TZSC_MPCWM4BCFGR = crate::Reg<tzsc_mpcwm4bcfgr::TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
pub mod tzsc_mpcwm4bcfgr;
#[doc = "TZSC_MPCWM4BR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4br`] module"]
pub type TZSC_MPCWM4BR = crate::Reg<tzsc_mpcwm4br::TZSC_MPCWM4BR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register"]
pub mod tzsc_mpcwm4br;
#[doc = "MPCBB1_PRIVCFGR0 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr0`] module"]
pub type MPCBB1_PRIVCFGR0 = crate::Reg<mpcbb1_privcfgr0::MPCBB1_PRIVCFGR0_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
pub mod mpcbb1_privcfgr0;
#[doc = "MPCBB1_PRIVCFGR1 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr1`] module"]
pub type MPCBB1_PRIVCFGR1 = crate::Reg<mpcbb1_privcfgr1::MPCBB1_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
pub mod mpcbb1_privcfgr1;
#[doc = "MPCBB1_PRIVCFGR2 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr2`] module"]
pub type MPCBB1_PRIVCFGR2 = crate::Reg<mpcbb1_privcfgr2::MPCBB1_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
pub mod mpcbb1_privcfgr2;
#[doc = "MPCBB1_PRIVCFGR3 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr3`] module"]
pub type MPCBB1_PRIVCFGR3 = crate::Reg<mpcbb1_privcfgr3::MPCBB1_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
pub mod mpcbb1_privcfgr3;
#[doc = "MPCBB1_PRIVCFGR4 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr4`] module"]
pub type MPCBB1_PRIVCFGR4 = crate::Reg<mpcbb1_privcfgr4::MPCBB1_PRIVCFGR4_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
pub mod mpcbb1_privcfgr4;
#[doc = "MPCBB1_PRIVCFGR5 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr5`] module"]
pub type MPCBB1_PRIVCFGR5 = crate::Reg<mpcbb1_privcfgr5::MPCBB1_PRIVCFGR5_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
pub mod mpcbb1_privcfgr5;
#[doc = "MPCBB1_PRIVCFGR6 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr6`] module"]
pub type MPCBB1_PRIVCFGR6 = crate::Reg<mpcbb1_privcfgr6::MPCBB1_PRIVCFGR6_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
pub mod mpcbb1_privcfgr6;
#[doc = "MPCBB1_PRIVCFGR7 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr7`] module"]
pub type MPCBB1_PRIVCFGR7 = crate::Reg<mpcbb1_privcfgr7::MPCBB1_PRIVCFGR7_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
pub mod mpcbb1_privcfgr7;
#[doc = "MPCBB1_PRIVCFGR8 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr8`] module"]
pub type MPCBB1_PRIVCFGR8 = crate::Reg<mpcbb1_privcfgr8::MPCBB1_PRIVCFGR8_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
pub mod mpcbb1_privcfgr8;
#[doc = "MPCBB1_PRIVCFGR9 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr9`] module"]
pub type MPCBB1_PRIVCFGR9 = crate::Reg<mpcbb1_privcfgr9::MPCBB1_PRIVCFGR9_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
pub mod mpcbb1_privcfgr9;
#[doc = "MPCBB1_PRIVCFGR10 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr10`] module"]
pub type MPCBB1_PRIVCFGR10 = crate::Reg<mpcbb1_privcfgr10::MPCBB1_PRIVCFGR10_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
pub mod mpcbb1_privcfgr10;
#[doc = "MPCBB1_PRIVCFGR11 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr11`] module"]
pub type MPCBB1_PRIVCFGR11 = crate::Reg<mpcbb1_privcfgr11::MPCBB1_PRIVCFGR11_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
pub mod mpcbb1_privcfgr11;
#[doc = "MPCBB1_PRIVCFGR12 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr12`] module"]
pub type MPCBB1_PRIVCFGR12 = crate::Reg<mpcbb1_privcfgr12::MPCBB1_PRIVCFGR12_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
pub mod mpcbb1_privcfgr12;
#[doc = "MPCBB1_PRIVCFGR13 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr13`] module"]
pub type MPCBB1_PRIVCFGR13 = crate::Reg<mpcbb1_privcfgr13::MPCBB1_PRIVCFGR13_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
pub mod mpcbb1_privcfgr13;
#[doc = "MPCBB1_PRIVCFGR14 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr14`] module"]
pub type MPCBB1_PRIVCFGR14 = crate::Reg<mpcbb1_privcfgr14::MPCBB1_PRIVCFGR14_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
pub mod mpcbb1_privcfgr14;
#[doc = "MPCBB1_PRIVCFGR15 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr15`] module"]
pub type MPCBB1_PRIVCFGR15 = crate::Reg<mpcbb1_privcfgr15::MPCBB1_PRIVCFGR15_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
pub mod mpcbb1_privcfgr15;
#[doc = "MPCBB1_PRIVCFGR16 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr16`] module"]
pub type MPCBB1_PRIVCFGR16 = crate::Reg<mpcbb1_privcfgr16::MPCBB1_PRIVCFGR16_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
pub mod mpcbb1_privcfgr16;
#[doc = "MPCBB1_PRIVCFGR17 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr17`] module"]
pub type MPCBB1_PRIVCFGR17 = crate::Reg<mpcbb1_privcfgr17::MPCBB1_PRIVCFGR17_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
pub mod mpcbb1_privcfgr17;
#[doc = "MPCBB1_PRIVCFGR18 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr18`] module"]
pub type MPCBB1_PRIVCFGR18 = crate::Reg<mpcbb1_privcfgr18::MPCBB1_PRIVCFGR18_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
pub mod mpcbb1_privcfgr18;
#[doc = "MPCBB1_PRIVCFGR19 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr19`] module"]
pub type MPCBB1_PRIVCFGR19 = crate::Reg<mpcbb1_privcfgr19::MPCBB1_PRIVCFGR19_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
pub mod mpcbb1_privcfgr19;
#[doc = "MPCBB1_PRIVCFGR20 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr20`] module"]
pub type MPCBB1_PRIVCFGR20 = crate::Reg<mpcbb1_privcfgr20::MPCBB1_PRIVCFGR20_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
pub mod mpcbb1_privcfgr20;
#[doc = "MPCBB1_PRIVCFGR21 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr21`] module"]
pub type MPCBB1_PRIVCFGR21 = crate::Reg<mpcbb1_privcfgr21::MPCBB1_PRIVCFGR21_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
pub mod mpcbb1_privcfgr21;
#[doc = "MPCBB1_PRIVCFGR22 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr22`] module"]
pub type MPCBB1_PRIVCFGR22 = crate::Reg<mpcbb1_privcfgr22::MPCBB1_PRIVCFGR22_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
pub mod mpcbb1_privcfgr22;
#[doc = "MPCBB1_PRIVCFGR23 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr23`] module"]
pub type MPCBB1_PRIVCFGR23 = crate::Reg<mpcbb1_privcfgr23::MPCBB1_PRIVCFGR23_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
pub mod mpcbb1_privcfgr23;
#[doc = "MPCBB1_PRIVCFGR24 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr24`] module"]
pub type MPCBB1_PRIVCFGR24 = crate::Reg<mpcbb1_privcfgr24::MPCBB1_PRIVCFGR24_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
pub mod mpcbb1_privcfgr24;
#[doc = "MPCBB1_PRIVCFGR25 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr25`] module"]
pub type MPCBB1_PRIVCFGR25 = crate::Reg<mpcbb1_privcfgr25::MPCBB1_PRIVCFGR25_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
pub mod mpcbb1_privcfgr25;
#[doc = "MPCBB1_PRIVCFGR26 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr26`] module"]
pub type MPCBB1_PRIVCFGR26 = crate::Reg<mpcbb1_privcfgr26::MPCBB1_PRIVCFGR26_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
pub mod mpcbb1_privcfgr26;
#[doc = "MPCBB1_PRIVCFGR27 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr27`] module"]
pub type MPCBB1_PRIVCFGR27 = crate::Reg<mpcbb1_privcfgr27::MPCBB1_PRIVCFGR27_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
pub mod mpcbb1_privcfgr27;
#[doc = "MPCBB1_PRIVCFGR28 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr28`] module"]
pub type MPCBB1_PRIVCFGR28 = crate::Reg<mpcbb1_privcfgr28::MPCBB1_PRIVCFGR28_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
pub mod mpcbb1_privcfgr28;
#[doc = "MPCBB1_PRIVCFGR29 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr29`] module"]
pub type MPCBB1_PRIVCFGR29 = crate::Reg<mpcbb1_privcfgr29::MPCBB1_PRIVCFGR29_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
pub mod mpcbb1_privcfgr29;
#[doc = "MPCBB1_PRIVCFGR30 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr30`] module"]
pub type MPCBB1_PRIVCFGR30 = crate::Reg<mpcbb1_privcfgr30::MPCBB1_PRIVCFGR30_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
pub mod mpcbb1_privcfgr30;
#[doc = "MPCBB1_PRIVCFGR31 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr31`] module"]
pub type MPCBB1_PRIVCFGR31 = crate::Reg<mpcbb1_privcfgr31::MPCBB1_PRIVCFGR31_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
pub mod mpcbb1_privcfgr31;
#[doc = "MPCBB2_PRIVCFGR0 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr0`] module"]
pub type MPCBB2_PRIVCFGR0 = crate::Reg<mpcbb2_privcfgr0::MPCBB2_PRIVCFGR0_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
pub mod mpcbb2_privcfgr0;
#[doc = "MPCBB2_PRIVCFGR1 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr1`] module"]
pub type MPCBB2_PRIVCFGR1 = crate::Reg<mpcbb2_privcfgr1::MPCBB2_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
pub mod mpcbb2_privcfgr1;
#[doc = "MPCBB2_PRIVCFGR2 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr2`] module"]
pub type MPCBB2_PRIVCFGR2 = crate::Reg<mpcbb2_privcfgr2::MPCBB2_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
pub mod mpcbb2_privcfgr2;
#[doc = "MPCBB2_PRIVCFGR3 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr3`] module"]
pub type MPCBB2_PRIVCFGR3 = crate::Reg<mpcbb2_privcfgr3::MPCBB2_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
pub mod mpcbb2_privcfgr3;
#[doc = "MPCBB2_PRIVCFGR4 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr4`] module"]
pub type MPCBB2_PRIVCFGR4 = crate::Reg<mpcbb2_privcfgr4::MPCBB2_PRIVCFGR4_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
pub mod mpcbb2_privcfgr4;
#[doc = "MPCBB2_PRIVCFGR5 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr5`] module"]
pub type MPCBB2_PRIVCFGR5 = crate::Reg<mpcbb2_privcfgr5::MPCBB2_PRIVCFGR5_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
pub mod mpcbb2_privcfgr5;
#[doc = "MPCBB2_PRIVCFGR6 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr6`] module"]
pub type MPCBB2_PRIVCFGR6 = crate::Reg<mpcbb2_privcfgr6::MPCBB2_PRIVCFGR6_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
pub mod mpcbb2_privcfgr6;
#[doc = "MPCBB2_PRIVCFGR7 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr7`] module"]
pub type MPCBB2_PRIVCFGR7 = crate::Reg<mpcbb2_privcfgr7::MPCBB2_PRIVCFGR7_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
pub mod mpcbb2_privcfgr7;
#[doc = "MPCBB2_PRIVCFGR8 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr8`] module"]
pub type MPCBB2_PRIVCFGR8 = crate::Reg<mpcbb2_privcfgr8::MPCBB2_PRIVCFGR8_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
pub mod mpcbb2_privcfgr8;
#[doc = "MPCBB2_PRIVCFGR9 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr9`] module"]
pub type MPCBB2_PRIVCFGR9 = crate::Reg<mpcbb2_privcfgr9::MPCBB2_PRIVCFGR9_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
pub mod mpcbb2_privcfgr9;
#[doc = "MPCBB2_PRIVCFGR10 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr10`] module"]
pub type MPCBB2_PRIVCFGR10 = crate::Reg<mpcbb2_privcfgr10::MPCBB2_PRIVCFGR10_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
pub mod mpcbb2_privcfgr10;
#[doc = "MPCBB2_PRIVCFGR11 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr11`] module"]
pub type MPCBB2_PRIVCFGR11 = crate::Reg<mpcbb2_privcfgr11::MPCBB2_PRIVCFGR11_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
pub mod mpcbb2_privcfgr11;
#[doc = "MPCBB2_PRIVCFGR12 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr12`] module"]
pub type MPCBB2_PRIVCFGR12 = crate::Reg<mpcbb2_privcfgr12::MPCBB2_PRIVCFGR12_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
pub mod mpcbb2_privcfgr12;
#[doc = "MPCBB2_PRIVCFGR13 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr13`] module"]
pub type MPCBB2_PRIVCFGR13 = crate::Reg<mpcbb2_privcfgr13::MPCBB2_PRIVCFGR13_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
pub mod mpcbb2_privcfgr13;
#[doc = "MPCBB2_PRIVCFGR14 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr14`] module"]
pub type MPCBB2_PRIVCFGR14 = crate::Reg<mpcbb2_privcfgr14::MPCBB2_PRIVCFGR14_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
pub mod mpcbb2_privcfgr14;
#[doc = "MPCBB2_PRIVCFGR15 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr15`] module"]
pub type MPCBB2_PRIVCFGR15 = crate::Reg<mpcbb2_privcfgr15::MPCBB2_PRIVCFGR15_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
pub mod mpcbb2_privcfgr15;
#[doc = "MPCBB2_PRIVCFGR16 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr16`] module"]
pub type MPCBB2_PRIVCFGR16 = crate::Reg<mpcbb2_privcfgr16::MPCBB2_PRIVCFGR16_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
pub mod mpcbb2_privcfgr16;
#[doc = "MPCBB2_PRIVCFGR17 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr17`] module"]
pub type MPCBB2_PRIVCFGR17 = crate::Reg<mpcbb2_privcfgr17::MPCBB2_PRIVCFGR17_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
pub mod mpcbb2_privcfgr17;
#[doc = "MPCBB2_PRIVCFGR18 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr18`] module"]
pub type MPCBB2_PRIVCFGR18 = crate::Reg<mpcbb2_privcfgr18::MPCBB2_PRIVCFGR18_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
pub mod mpcbb2_privcfgr18;
#[doc = "MPCBB2_PRIVCFGR19 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr19`] module"]
pub type MPCBB2_PRIVCFGR19 = crate::Reg<mpcbb2_privcfgr19::MPCBB2_PRIVCFGR19_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
pub mod mpcbb2_privcfgr19;
#[doc = "MPCBB2_PRIVCFGR20 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr20`] module"]
pub type MPCBB2_PRIVCFGR20 = crate::Reg<mpcbb2_privcfgr20::MPCBB2_PRIVCFGR20_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
pub mod mpcbb2_privcfgr20;
#[doc = "MPCBB2_PRIVCFGR21 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr21`] module"]
pub type MPCBB2_PRIVCFGR21 = crate::Reg<mpcbb2_privcfgr21::MPCBB2_PRIVCFGR21_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
pub mod mpcbb2_privcfgr21;
#[doc = "MPCBB2_PRIVCFGR22 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr22`] module"]
pub type MPCBB2_PRIVCFGR22 = crate::Reg<mpcbb2_privcfgr22::MPCBB2_PRIVCFGR22_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
pub mod mpcbb2_privcfgr22;
#[doc = "MPCBB2_PRIVCFGR23 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr23`] module"]
pub type MPCBB2_PRIVCFGR23 = crate::Reg<mpcbb2_privcfgr23::MPCBB2_PRIVCFGR23_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
pub mod mpcbb2_privcfgr23;
#[doc = "MPCBB2_PRIVCFGR24 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr24`] module"]
pub type MPCBB2_PRIVCFGR24 = crate::Reg<mpcbb2_privcfgr24::MPCBB2_PRIVCFGR24_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
pub mod mpcbb2_privcfgr24;
#[doc = "MPCBB2_PRIVCFGR25 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr25`] module"]
pub type MPCBB2_PRIVCFGR25 = crate::Reg<mpcbb2_privcfgr25::MPCBB2_PRIVCFGR25_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
pub mod mpcbb2_privcfgr25;
#[doc = "MPCBB2_PRIVCFGR26 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr26`] module"]
pub type MPCBB2_PRIVCFGR26 = crate::Reg<mpcbb2_privcfgr26::MPCBB2_PRIVCFGR26_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
pub mod mpcbb2_privcfgr26;
#[doc = "MPCBB2_PRIVCFGR27 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr27`] module"]
pub type MPCBB2_PRIVCFGR27 = crate::Reg<mpcbb2_privcfgr27::MPCBB2_PRIVCFGR27_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
pub mod mpcbb2_privcfgr27;
#[doc = "MPCBB2_PRIVCFGR28 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr28`] module"]
pub type MPCBB2_PRIVCFGR28 = crate::Reg<mpcbb2_privcfgr28::MPCBB2_PRIVCFGR28_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
pub mod mpcbb2_privcfgr28;
#[doc = "MPCBB2_PRIVCFGR29 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr29`] module"]
pub type MPCBB2_PRIVCFGR29 = crate::Reg<mpcbb2_privcfgr29::MPCBB2_PRIVCFGR29_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
pub mod mpcbb2_privcfgr29;
#[doc = "MPCBB2_PRIVCFGR30 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr30`] module"]
pub type MPCBB2_PRIVCFGR30 = crate::Reg<mpcbb2_privcfgr30::MPCBB2_PRIVCFGR30_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
pub mod mpcbb2_privcfgr30;
#[doc = "MPCBB2_PRIVCFGR31 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr31`] module"]
pub type MPCBB2_PRIVCFGR31 = crate::Reg<mpcbb2_privcfgr31::MPCBB2_PRIVCFGR31_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
pub mod mpcbb2_privcfgr31;
