#[doc = "Register `ATCR1` reader"]
pub type R = crate::R<ATCR1_SPEC>;
#[doc = "Register `ATCR1` writer"]
pub type W = crate::W<ATCR1_SPEC>;
#[doc = "Tamper 1 active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1AM_A {
    #[doc = "0: Tamper 1 detection mode is passive."]
    B_0x0 = 0,
    #[doc = "1: Tamper 1 detection mode is active."]
    B_0x1 = 1,
}
impl From<TAMP1AM_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1AM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1AM` reader - Tamper 1 active mode"]
pub type TAMP1AM_R = crate::BitReader<TAMP1AM_A>;
impl TAMP1AM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1AM_A {
        match self.bits {
            false => TAMP1AM_A::B_0x0,
            true => TAMP1AM_A::B_0x1,
        }
    }
    #[doc = "Tamper 1 detection mode is passive."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP1AM_A::B_0x0
    }
    #[doc = "Tamper 1 detection mode is active."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP1AM_A::B_0x1
    }
}
#[doc = "Field `TAMP1AM` writer - Tamper 1 active mode"]
pub type TAMP1AM_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1AM_A>;
impl<'a, REG> TAMP1AM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 detection mode is passive."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1AM_A::B_0x0)
    }
    #[doc = "Tamper 1 detection mode is active."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1AM_A::B_0x1)
    }
}
#[doc = "Tamper 2 active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2AM_A {
    #[doc = "0: Tamper 2 detection mode is passive."]
    B_0x0 = 0,
    #[doc = "1: Tamper 2 detection mode is active."]
    B_0x1 = 1,
}
impl From<TAMP2AM_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2AM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2AM` reader - Tamper 2 active mode"]
pub type TAMP2AM_R = crate::BitReader<TAMP2AM_A>;
impl TAMP2AM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2AM_A {
        match self.bits {
            false => TAMP2AM_A::B_0x0,
            true => TAMP2AM_A::B_0x1,
        }
    }
    #[doc = "Tamper 2 detection mode is passive."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP2AM_A::B_0x0
    }
    #[doc = "Tamper 2 detection mode is active."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP2AM_A::B_0x1
    }
}
#[doc = "Field `TAMP2AM` writer - Tamper 2 active mode"]
pub type TAMP2AM_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2AM_A>;
impl<'a, REG> TAMP2AM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 detection mode is passive."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2AM_A::B_0x0)
    }
    #[doc = "Tamper 2 detection mode is active."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2AM_A::B_0x1)
    }
}
#[doc = "Active tamper shared output 1 selection The selected output must be available in the package pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL1_A {
    #[doc = "0: TAMPOUTSEL1 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL1 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL1 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: no value"]
    B_0x3 = 3,
}
impl From<ATOSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL1_A {}
#[doc = "Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_R = crate::FieldReader<ATOSEL1_A>;
impl ATOSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL1_A {
        match self.bits {
            0 => ATOSEL1_A::B_0x0,
            1 => ATOSEL1_A::B_0x1,
            2 => ATOSEL1_A::B_0x2,
            3 => ATOSEL1_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL1_A::B_0x0
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL1_A::B_0x1
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL1_A::B_0x2
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL1_A::B_0x3
    }
}
#[doc = "Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ATOSEL1_A, crate::Safe>;
impl<'a, REG> ATOSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL1 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x2)
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x3)
    }
}
#[doc = "Active tamper shared output 2 selection The selected output must be available in the package pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL2_A {
    #[doc = "0: TAMPOUTSEL2 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL2 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL2 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: no value"]
    B_0x3 = 3,
}
impl From<ATOSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL2_A {}
#[doc = "Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_R = crate::FieldReader<ATOSEL2_A>;
impl ATOSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL2_A {
        match self.bits {
            0 => ATOSEL2_A::B_0x0,
            1 => ATOSEL2_A::B_0x1,
            2 => ATOSEL2_A::B_0x2,
            3 => ATOSEL2_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL2_A::B_0x0
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL2_A::B_0x1
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL2_A::B_0x2
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL2_A::B_0x3
    }
}
#[doc = "Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ATOSEL2_A, crate::Safe>;
impl<'a, REG> ATOSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL2 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x2)
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x3)
    }
}
#[doc = "Active tamper shared output 3 selection The selected output must be available in the package pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL3_A {
    #[doc = "0: TAMPOUTSEL3 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL3 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL3 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: no value"]
    B_0x3 = 3,
}
impl From<ATOSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL3_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL3_A {}
#[doc = "Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_R = crate::FieldReader<ATOSEL3_A>;
impl ATOSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL3_A {
        match self.bits {
            0 => ATOSEL3_A::B_0x0,
            1 => ATOSEL3_A::B_0x1,
            2 => ATOSEL3_A::B_0x2,
            3 => ATOSEL3_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL3_A::B_0x0
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL3_A::B_0x1
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL3_A::B_0x2
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL3_A::B_0x3
    }
}
#[doc = "Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ATOSEL3_A, crate::Safe>;
impl<'a, REG> ATOSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL3 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x2)
    }
    #[doc = "no value"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x3)
    }
}
#[doc = "Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f CK_ATPRE = f RTCCLK / 2supATCKSEL /supwhen (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATCKSEL_A {
    #[doc = "0: RTCCLK is selected"]
    B_0x0 = 0,
    #[doc = "1: RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1supst/sup flip flop output)"]
    B_0x1 = 1,
    #[doc = "2: RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2supnd/sup flip flop output)"]
    B_0x2 = 2,
    #[doc = "7: RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7supth/sup flip flop output)"]
    B_0x7 = 7,
}
impl From<ATCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ATCKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATCKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ATCKSEL_A {}
#[doc = "Field `ATCKSEL` reader - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f CK_ATPRE = f RTCCLK / 2supATCKSEL /supwhen (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable."]
pub type ATCKSEL_R = crate::FieldReader<ATCKSEL_A>;
impl ATCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ATCKSEL_A> {
        match self.bits {
            0 => Some(ATCKSEL_A::B_0x0),
            1 => Some(ATCKSEL_A::B_0x1),
            2 => Some(ATCKSEL_A::B_0x2),
            7 => Some(ATCKSEL_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "RTCCLK is selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATCKSEL_A::B_0x0
    }
    #[doc = "RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1supst/sup flip flop output)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATCKSEL_A::B_0x1
    }
    #[doc = "RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2supnd/sup flip flop output)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATCKSEL_A::B_0x2
    }
    #[doc = "RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7supth/sup flip flop output)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATCKSEL_A::B_0x7
    }
}
#[doc = "Field `ATCKSEL` writer - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f CK_ATPRE = f RTCCLK / 2supATCKSEL /supwhen (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable."]
pub type ATCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATCKSEL_A>;
impl<'a, REG> ATCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTCCLK is selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATCKSEL_A::B_0x0)
    }
    #[doc = "RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1supst/sup flip flop output)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATCKSEL_A::B_0x1)
    }
    #[doc = "RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2supnd/sup flip flop output)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATCKSEL_A::B_0x2)
    }
    #[doc = "RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7supth/sup flip flop output)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATCKSEL_A::B_0x7)
    }
}
#[doc = "Field `ATPER` reader - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value."]
pub type ATPER_R = crate::FieldReader;
#[doc = "Field `ATPER` writer - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value."]
pub type ATPER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATOSHARE_A {
    #[doc = "0: Each active tamper input TAMP_INi is compared with its dedicated output TAMP_OUTi"]
    B_0x0 = 0,
    #[doc = "1: Each active tamper input TAMP_INi is compared with TAMPOUTSELx as defined below, with TAMPOUTSELx defined by ATOSELx bits."]
    B_0x1 = 1,
}
impl From<ATOSHARE_A> for bool {
    #[inline(always)]
    fn from(variant: ATOSHARE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATOSHARE` reader - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_R = crate::BitReader<ATOSHARE_A>;
impl ATOSHARE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSHARE_A {
        match self.bits {
            false => ATOSHARE_A::B_0x0,
            true => ATOSHARE_A::B_0x1,
        }
    }
    #[doc = "Each active tamper input TAMP_INi is compared with its dedicated output TAMP_OUTi"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSHARE_A::B_0x0
    }
    #[doc = "Each active tamper input TAMP_INi is compared with TAMPOUTSELx as defined below, with TAMPOUTSELx defined by ATOSELx bits."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSHARE_A::B_0x1
    }
}
#[doc = "Field `ATOSHARE` writer - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_W<'a, REG> = crate::BitWriter<'a, REG, ATOSHARE_A>;
impl<'a, REG> ATOSHARE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each active tamper input TAMP_INi is compared with its dedicated output TAMP_OUTi"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSHARE_A::B_0x0)
    }
    #[doc = "Each active tamper input TAMP_INi is compared with TAMPOUTSELx as defined below, with TAMPOUTSELx defined by ATOSELx bits."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSHARE_A::B_0x1)
    }
}
#[doc = "Active tamper filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTEN_A {
    #[doc = "0: Active tamper filtering disable"]
    B_0x0 = 0,
    #[doc = "1: Active tamper filtering enable: a tamper event is detected when 2 comparison mismatches occur out of 4 consecutive samples."]
    B_0x1 = 1,
}
impl From<FLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTEN` reader - Active tamper filter enable"]
pub type FLTEN_R = crate::BitReader<FLTEN_A>;
impl FLTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLTEN_A {
        match self.bits {
            false => FLTEN_A::B_0x0,
            true => FLTEN_A::B_0x1,
        }
    }
    #[doc = "Active tamper filtering disable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FLTEN_A::B_0x0
    }
    #[doc = "Active tamper filtering enable: a tamper event is detected when 2 comparison mismatches occur out of 4 consecutive samples."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FLTEN_A::B_0x1
    }
}
#[doc = "Field `FLTEN` writer - Active tamper filter enable"]
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG, FLTEN_A>;
impl<'a, REG> FLTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active tamper filtering disable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLTEN_A::B_0x0)
    }
    #[doc = "Active tamper filtering enable: a tamper event is detected when 2 comparison mismatches occur out of 4 consecutive samples."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLTEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    pub fn TAMP1AM(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    pub fn TAMP2AM(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f CK_ATPRE = f RTCCLK / 2supATCKSEL /supwhen (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable."]
    #[inline(always)]
    pub fn ATCKSEL(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value."]
    #[inline(always)]
    pub fn ATPER(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    pub fn ATOSHARE(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    pub fn FLTEN(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    pub fn TAMP1AM(&mut self) -> TAMP1AM_W<'_, ATCR1_SPEC> {
        TAMP1AM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    pub fn TAMP2AM(&mut self) -> TAMP2AM_W<'_, ATCR1_SPEC> {
        TAMP2AM_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL1(&mut self) -> ATOSEL1_W<'_, ATCR1_SPEC> {
        ATOSEL1_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL2(&mut self) -> ATOSEL2_W<'_, ATCR1_SPEC> {
        ATOSEL2_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn ATOSEL3(&mut self) -> ATOSEL3_W<'_, ATCR1_SPEC> {
        ATOSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. f CK_ATPRE = f RTCCLK / 2supATCKSEL /supwhen (PREDIV_A+1) = 128. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable."]
    #[inline(always)]
    pub fn ATCKSEL(&mut self) -> ATCKSEL_W<'_, ATCR1_SPEC> {
        ATCKSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table 239: Minimum ATPER value."]
    #[inline(always)]
    pub fn ATPER(&mut self) -> ATPER_W<'_, ATCR1_SPEC> {
        ATPER_W::new(self, 24)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    pub fn ATOSHARE(&mut self) -> ATOSHARE_W<'_, ATCR1_SPEC> {
        ATOSHARE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    pub fn FLTEN(&mut self) -> FLTEN_W<'_, ATCR1_SPEC> {
        FLTEN_W::new(self, 31)
    }
}
#[doc = "TAMP active tamper control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`atcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATCR1_SPEC;
impl crate::RegisterSpec for ATCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atcr1::R`](R) reader structure"]
impl crate::Readable for ATCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atcr1::W`](W) writer structure"]
impl crate::Writable for ATCR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ATCR1 to value 0x0007_0000"]
impl crate::Resettable for ATCR1_SPEC {
    const RESET_VALUE: u32 = 0x0007_0000;
}
