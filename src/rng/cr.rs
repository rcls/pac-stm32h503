#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "True random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    #[doc = "0: True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    B_0x0 = 0,
    #[doc = "1: True random number generator is enabled."]
    B_0x1 = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - True random number generator enable"]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::B_0x0,
            true => RNGEN_A::B_0x1,
        }
    }
    #[doc = "True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGEN_A::B_0x0
    }
    #[doc = "True random number generator is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGEN_A::B_0x1
    }
}
#[doc = "Field `RNGEN` writer - True random number generator enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN_A>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN_A::B_0x0)
    }
    #[doc = "True random number generator is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN_A::B_0x1)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE_A {
    #[doc = "0: RNG Interrupt is disabled"]
    B_0x0 = 0,
    #[doc = "1: RNG Interrupt is enabled. An interrupt is pending as soon as DRDY = 1, SEIS = 1 or CEIS = 1 in the RNG_SR register."]
    B_0x1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<IE_A>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::B_0x0,
            true => IE_A::B_0x1,
        }
    }
    #[doc = "RNG Interrupt is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IE_A::B_0x0
    }
    #[doc = "RNG Interrupt is enabled. An interrupt is pending as soon as DRDY = 1, SEIS = 1 or CEIS = 1 in the RNG_SR register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IE_A::B_0x1
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE_A>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG Interrupt is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IE_A::B_0x0)
    }
    #[doc = "RNG Interrupt is enabled. An interrupt is pending as soon as DRDY = 1, SEIS = 1 or CEIS = 1 in the RNG_SR register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IE_A::B_0x1)
    }
}
#[doc = "Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED_A {
    #[doc = "0: Clock error detection is enable"]
    B_0x0 = 0,
    #[doc = "1: Clock error detection is disable"]
    B_0x1 = 1,
}
impl From<CED_A> for bool {
    #[inline(always)]
    fn from(variant: CED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type CED_R = crate::BitReader<CED_A>;
impl CED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CED_A {
        match self.bits {
            false => CED_A::B_0x0,
            true => CED_A::B_0x1,
        }
    }
    #[doc = "Clock error detection is enable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CED_A::B_0x0
    }
    #[doc = "Clock error detection is disable"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CED_A::B_0x1
    }
}
#[doc = "Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG, CED_A>;
impl<'a, REG> CED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock error detection is enable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CED_A::B_0x0)
    }
    #[doc = "Clock error detection is disable"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CED_A::B_0x1)
    }
}
#[doc = "Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARDIS_A {
    #[doc = "0: When a noise source error occurs RNG performs an automatic reset to clear SECS bit."]
    B_0x0 = 0,
    #[doc = "1: When a noise source error occurs application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    B_0x1 = 1,
}
impl From<ARDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ARDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARDIS` reader - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type ARDIS_R = crate::BitReader<ARDIS_A>;
impl ARDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARDIS_A {
        match self.bits {
            false => ARDIS_A::B_0x0,
            true => ARDIS_A::B_0x1,
        }
    }
    #[doc = "When a noise source error occurs RNG performs an automatic reset to clear SECS bit."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ARDIS_A::B_0x0
    }
    #[doc = "When a noise source error occurs application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ARDIS_A::B_0x1
    }
}
#[doc = "Field `ARDIS` writer - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type ARDIS_W<'a, REG> = crate::BitWriter<'a, REG, ARDIS_A>;
impl<'a, REG> ARDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When a noise source error occurs RNG performs an automatic reset to clear SECS bit."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARDIS_A::B_0x0)
    }
    #[doc = "When a noise source error occurs application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARDIS_A::B_0x1)
    }
}
#[doc = "Field `RNG_CONFIG3` reader - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RNG_CONFIG3_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG3` writer - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RNG_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Non NIST compliant two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISTC_A {
    #[doc = "0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    B_0x0 = 0,
    #[doc = "1: Custom values for NIST compliant RNG. See Section 23.6: RNG entropy source validation for proposed configuration."]
    B_0x1 = 1,
}
impl From<NISTC_A> for bool {
    #[inline(always)]
    fn from(variant: NISTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NISTC` reader - Non NIST compliant two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type NISTC_R = crate::BitReader<NISTC_A>;
impl NISTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NISTC_A {
        match self.bits {
            false => NISTC_A::B_0x0,
            true => NISTC_A::B_0x1,
        }
    }
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NISTC_A::B_0x0
    }
    #[doc = "Custom values for NIST compliant RNG. See Section 23.6: RNG entropy source validation for proposed configuration."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NISTC_A::B_0x1
    }
}
#[doc = "Field `NISTC` writer - Non NIST compliant two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type NISTC_W<'a, REG> = crate::BitWriter<'a, REG, NISTC_A>;
impl<'a, REG> NISTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC_A::B_0x0)
    }
    #[doc = "Custom values for NIST compliant RNG. See Section 23.6: RNG entropy source validation for proposed configuration."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NISTC_A::B_0x1)
    }
}
#[doc = "Field `RNG_CONFIG2` reader - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
pub type RNG_CONFIG2_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG2` writer - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
pub type RNG_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: internal RNG clock after divider is similar to incoming RNG clock."]
    B_0x0 = 0,
    #[doc = "1: two RNG clock cycles per internal RNG clock."]
    B_0x1 = 1,
    #[doc = "2: 2sup2/sup (= 4) RNG clock cycles per internal RNG clock."]
    B_0x2 = 2,
    #[doc = "15: 2sup15/sup RNG clock cycles per internal clock (for example. an incoming 48 MHz RNG clock becomes a 1.5 kHz internal RNG clock)"]
    B_0xF = 15,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKDIV_A {}
#[doc = "Field `CLKDIV` reader - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type CLKDIV_R = crate::FieldReader<CLKDIV_A>;
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::B_0x0),
            1 => Some(CLKDIV_A::B_0x1),
            2 => Some(CLKDIV_A::B_0x2),
            15 => Some(CLKDIV_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "internal RNG clock after divider is similar to incoming RNG clock."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CLKDIV_A::B_0x0
    }
    #[doc = "two RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CLKDIV_A::B_0x1
    }
    #[doc = "2sup2/sup (= 4) RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CLKDIV_A::B_0x2
    }
    #[doc = "2sup15/sup RNG clock cycles per internal clock (for example. an incoming 48 MHz RNG clock becomes a 1.5 kHz internal RNG clock)"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == CLKDIV_A::B_0xF
    }
}
#[doc = "Field `CLKDIV` writer - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKDIV_A>;
impl<'a, REG> CLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal RNG clock after divider is similar to incoming RNG clock."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::B_0x0)
    }
    #[doc = "two RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::B_0x1)
    }
    #[doc = "2sup2/sup (= 4) RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::B_0x2)
    }
    #[doc = "2sup15/sup RNG clock cycles per internal clock (for example. an incoming 48 MHz RNG clock becomes a 1.5 kHz internal RNG clock)"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::B_0xF)
    }
}
#[doc = "Field `RNG_CONFIG1` reader - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section 23.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type RNG_CONFIG1_R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG1` writer - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section 23.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
pub type RNG_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CONDRST` reader - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\] must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CONDRST_R = crate::BitReader;
#[doc = "Field `CONDRST` writer - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\] must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CONDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFIGLOCK_A {
    #[doc = "0: Writes to the RNG_CR configuration bits \\[29:4\\] are allowed."]
    B_0x0 = 0,
    #[doc = "1: Writes to the RNG_CR configuration bits \\[29:4\\] are ignored until the next RNG reset."]
    B_0x1 = 1,
}
impl From<CONFIGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: CONFIGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONFIGLOCK` reader - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type CONFIGLOCK_R = crate::BitReader<CONFIGLOCK_A>;
impl CONFIGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONFIGLOCK_A {
        match self.bits {
            false => CONFIGLOCK_A::B_0x0,
            true => CONFIGLOCK_A::B_0x1,
        }
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\] are allowed."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CONFIGLOCK_A::B_0x0
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\] are ignored until the next RNG reset."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CONFIGLOCK_A::B_0x1
    }
}
#[doc = "Field `CONFIGLOCK` writer - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG, CONFIGLOCK_A>;
impl<'a, REG> CONFIGLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\] are allowed."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK_A::B_0x0)
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\] are ignored until the next RNG reset."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIGLOCK_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn RNGEN(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn IE(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn CED(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn ARDIS(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    pub fn RNG_CONFIG3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Non NIST compliant two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn NISTC(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    pub fn RNG_CONFIG2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn CLKDIV(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section 23.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn RNG_CONFIG1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\] must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    pub fn CONDRST(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    pub fn CONFIGLOCK(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn RNGEN(&mut self) -> RNGEN_W<'_, CR_SPEC> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn IE(&mut self) -> IE_W<'_, CR_SPEC> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn CED(&mut self) -> CED_W<'_, CR_SPEC> {
        CED_W::new(self, 5)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled application still need to clear SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn ARDIS(&mut self) -> ARDIS_W<'_, CR_SPEC> {
        ARDIS_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    pub fn RNG_CONFIG3(&mut self) -> RNG_CONFIG3_W<'_, CR_SPEC> {
        RNG_CONFIG3_W::new(self, 8)
    }
    #[doc = "Bit 12 - Non NIST compliant two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn NISTC(&mut self) -> NISTC_W<'_, CR_SPEC> {
        NISTC_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Refer to RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    pub fn RNG_CONFIG2(&mut self) -> RNG_CONFIG2_W<'_, CR_SPEC> {
        RNG_CONFIG2_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN = 0). ... Writing these bits is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn CLKDIV(&mut self) -> CLKDIV_W<'_, CR_SPEC> {
        CLKDIV_W::new(self, 16)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section 23.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1."]
    #[inline(always)]
    pub fn RNG_CONFIG1(&mut self) -> RNG_CONFIG1_W<'_, CR_SPEC> {
        RNG_CONFIG1_W::new(self, 20)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_NSCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\] must also be written. When CONDRST is set to 0 by software its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    pub fn CONDRST(&mut self) -> CONDRST_W<'_, CR_SPEC> {
        CONDRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    pub fn CONFIGLOCK(&mut self) -> CONFIGLOCK_W<'_, CR_SPEC> {
        CONFIGLOCK_W::new(self, 31)
    }
}
#[doc = "RNG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x0080_00d0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x0080_00d0;
}
