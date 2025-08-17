#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FLTCR_SPEC>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FLTCR_SPEC>;
#[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    #[doc = "0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    B_0x0 = 0,
    #[doc = "1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    B_0x1 = 1,
    #[doc = "2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    B_0x2 = 2,
    #[doc = "3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    B_0x3 = 3,
    #[doc = "4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    B_0x4 = 4,
    #[doc = "5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    B_0x5 = 5,
    #[doc = "6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    B_0x6 = 6,
    #[doc = "7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    B_0x7 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFREQ_A {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFREQ_A {}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TAMPFREQ_R = crate::FieldReader<TAMPFREQ_A>;
impl TAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::B_0x0,
            1 => TAMPFREQ_A::B_0x1,
            2 => TAMPFREQ_A::B_0x2,
            3 => TAMPFREQ_A::B_0x3,
            4 => TAMPFREQ_A::B_0x4,
            5 => TAMPFREQ_A::B_0x5,
            6 => TAMPFREQ_A::B_0x6,
            7 => TAMPFREQ_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMPFREQ_A::B_0x0
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMPFREQ_A::B_0x1
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TAMPFREQ_A::B_0x2
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TAMPFREQ_A::B_0x3
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == TAMPFREQ_A::B_0x4
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == TAMPFREQ_A::B_0x5
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == TAMPFREQ_A::B_0x6
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == TAMPFREQ_A::B_0x7
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TAMPFREQ_A, crate::Safe>;
impl<'a, REG> TAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x0)
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x1)
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x2)
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x3)
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x4)
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x5)
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x6)
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ_A::B_0x7)
    }
}
#[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    #[doc = "0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    B_0x0 = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level."]
    B_0x1 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level."]
    B_0x2 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level."]
    B_0x3 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFLT_A {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFLT_A {}
#[doc = "Field `TAMPFLT` reader - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TAMPFLT_R = crate::FieldReader<TAMPFLT_A>;
impl TAMPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::B_0x0,
            1 => TAMPFLT_A::B_0x1,
            2 => TAMPFLT_A::B_0x2,
            3 => TAMPFLT_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMPFLT_A::B_0x0
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMPFLT_A::B_0x1
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TAMPFLT_A::B_0x2
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TAMPFLT_A::B_0x3
    }
}
#[doc = "Field `TAMPFLT` writer - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPFLT_A, crate::Safe>;
impl<'a, REG> TAMPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT_A::B_0x0)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT_A::B_0x1)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT_A::B_0x2)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT_A::B_0x3)
    }
}
#[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    #[doc = "0: 1 RTCCLK cycle"]
    B_0x0 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    B_0x1 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    B_0x2 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    B_0x3 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPPRCH_A {
    type Ux = u8;
}
impl crate::IsEnum for TAMPPRCH_A {}
#[doc = "Field `TAMPPRCH` reader - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TAMPPRCH_R = crate::FieldReader<TAMPPRCH_A>;
impl TAMPPRCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::B_0x0,
            1 => TAMPPRCH_A::B_0x1,
            2 => TAMPPRCH_A::B_0x2,
            3 => TAMPPRCH_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMPPRCH_A::B_0x0
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMPPRCH_A::B_0x1
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TAMPPRCH_A::B_0x2
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TAMPPRCH_A::B_0x3
    }
}
#[doc = "Field `TAMPPRCH` writer - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPPRCH_A, crate::Safe>;
impl<'a, REG> TAMPPRCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH_A::B_0x0)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH_A::B_0x1)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH_A::B_0x2)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH_A::B_0x3)
    }
}
#[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS_A {
    #[doc = "0: Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    B_0x0 = 0,
    #[doc = "1: Disable precharge of TAMP_INx pins."]
    B_0x1 = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS_A>;
impl TAMPPUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::B_0x0,
            true => TAMPPUDIS_A::B_0x1,
        }
    }
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMPPUDIS_A::B_0x0
    }
    #[doc = "Disable precharge of TAMP_INx pins."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMPPUDIS_A::B_0x1
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPUDIS_A>;
impl<'a, REG> TAMPPUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS_A::B_0x0)
    }
    #[doc = "Disable precharge of TAMP_INx pins."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    pub fn TAMPFREQ(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn TAMPFLT(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn TAMPPRCH(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    pub fn TAMPPUDIS(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    pub fn TAMPFREQ(&mut self) -> TAMPFREQ_W<'_, FLTCR_SPEC> {
        TAMPFREQ_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn TAMPFLT(&mut self) -> TAMPFLT_W<'_, FLTCR_SPEC> {
        TAMPFLT_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn TAMPPRCH(&mut self) -> TAMPPRCH_W<'_, FLTCR_SPEC> {
        TAMPPRCH_W::new(self, 5)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    pub fn TAMPPUDIS(&mut self) -> TAMPPUDIS_W<'_, FLTCR_SPEC> {
        TAMPPUDIS_W::new(self, 7)
    }
}
#[doc = "TAMP filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FLTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FLTCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FLTCR_SPEC {}
