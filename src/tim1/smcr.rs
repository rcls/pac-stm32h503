#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCR_SPEC>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCR_SPEC>;
#[doc = "SMS\\[2:0\\]: Slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS_A {
    #[doc = "0: Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock."]
    B_0x0 = 0,
    #[doc = "1: Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    B_0x1 = 1,
    #[doc = "2: Quadrature encoder mode 2, x2 mode - Counter counts up/down on tim_ti2fp2 edge depending on tim_ti1fp1 level."]
    B_0x2 = 2,
    #[doc = "3: Quadrature encoder mode 3, x4 mode - Counter counts up/down on both tim_ti1fp1 and tim_ti2fp2 edges depending on the level of the other input."]
    B_0x3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (tim_trgi) reinitializes the counter and generates an update of the registers."]
    B_0x4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (tim_trgi) is high."]
    B_0x5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger tim_trgi (but it is not reset)."]
    B_0x6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (tim_trgi) clock the counter."]
    B_0x7 = 7,
}
impl From<SMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMS_A {
    type Ux = u8;
}
impl crate::IsEnum for SMS_A {}
#[doc = "Field `SMS` reader - SMS\\[2:0\\]: Slave mode selection"]
pub type SMS_R = crate::FieldReader<SMS_A>;
impl SMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMS_A {
        match self.bits {
            0 => SMS_A::B_0x0,
            1 => SMS_A::B_0x1,
            2 => SMS_A::B_0x2,
            3 => SMS_A::B_0x3,
            4 => SMS_A::B_0x4,
            5 => SMS_A::B_0x5,
            6 => SMS_A::B_0x6,
            7 => SMS_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMS_A::B_0x0
    }
    #[doc = "Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMS_A::B_0x1
    }
    #[doc = "Quadrature encoder mode 2, x2 mode - Counter counts up/down on tim_ti2fp2 edge depending on tim_ti1fp1 level."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMS_A::B_0x2
    }
    #[doc = "Quadrature encoder mode 3, x4 mode - Counter counts up/down on both tim_ti1fp1 and tim_ti2fp2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMS_A::B_0x3
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (tim_trgi) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMS_A::B_0x4
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (tim_trgi) is high."]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMS_A::B_0x5
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger tim_trgi (but it is not reset)."]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMS_A::B_0x6
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (tim_trgi) clock the counter."]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMS_A::B_0x7
    }
}
#[doc = "Field `SMS` writer - SMS\\[2:0\\]: Slave mode selection"]
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMS_A, crate::Safe>;
impl<'a, REG> SMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x0)
    }
    #[doc = "Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x1)
    }
    #[doc = "Quadrature encoder mode 2, x2 mode - Counter counts up/down on tim_ti2fp2 edge depending on tim_ti1fp1 level."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x2)
    }
    #[doc = "Quadrature encoder mode 3, x4 mode - Counter counts up/down on both tim_ti1fp1 and tim_ti2fp2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (tim_trgi) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x4)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (tim_trgi) is high."]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x5)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger tim_trgi (but it is not reset)."]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x6)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (tim_trgi) clock the counter."]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_A::B_0x7)
    }
}
#[doc = "OCREF clear selection This bit is used to select the OCREF clear source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCCS_A {
    #[doc = "0: tim_ocref_clr_int is connected to the tim_ocref_clr input"]
    B_0x0 = 0,
    #[doc = "1: tim_ocref_clr_int is connected to tim_etrf"]
    B_0x1 = 1,
}
impl From<OCCS_A> for bool {
    #[inline(always)]
    fn from(variant: OCCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OCCS_R = crate::BitReader<OCCS_A>;
impl OCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCCS_A {
        match self.bits {
            false => OCCS_A::B_0x0,
            true => OCCS_A::B_0x1,
        }
    }
    #[doc = "tim_ocref_clr_int is connected to the tim_ocref_clr input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OCCS_A::B_0x0
    }
    #[doc = "tim_ocref_clr_int is connected to tim_etrf"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OCCS_A::B_0x1
    }
}
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG, OCCS_A>;
impl<'a, REG> OCCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_ocref_clr_int is connected to the tim_ocref_clr input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS_A::B_0x0)
    }
    #[doc = "tim_ocref_clr_int is connected to tim_etrf"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS_A::B_0x1)
    }
}
#[doc = "TS\\[2:0\\]: Trigger selection This bitfield is combined with TS\\[4:3\\] bits. This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 605: TIMx internal trigger connection for more details on tim_itrx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_A {
    #[doc = "0: Internal Trigger 0 (tim_itr0)"]
    B_0x0 = 0,
    #[doc = "1: Internal Trigger 1 (tim_itr1)"]
    B_0x1 = 1,
    #[doc = "2: Internal Trigger 2 (tim_itr2)"]
    B_0x2 = 2,
    #[doc = "3: Internal Trigger 3 (tim_itr3)"]
    B_0x3 = 3,
    #[doc = "4: tim_ti1 Edge Detector (tim_ti1f_ed)"]
    B_0x4 = 4,
    #[doc = "5: Filtered Timer Input 1 (tim_ti1fp1)"]
    B_0x5 = 5,
    #[doc = "6: Filtered Timer Input 2 (tim_ti2fp2)"]
    B_0x6 = 6,
    #[doc = "7: External Trigger input (tim_etrf)"]
    B_0x7 = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS_A {
    type Ux = u8;
}
impl crate::IsEnum for TS_A {}
#[doc = "Field `TS` reader - TS\\[2:0\\]: Trigger selection This bitfield is combined with TS\\[4:3\\] bits. This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 605: TIMx internal trigger connection for more details on tim_itrx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_R = crate::FieldReader<TS_A>;
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS_A {
        match self.bits {
            0 => TS_A::B_0x0,
            1 => TS_A::B_0x1,
            2 => TS_A::B_0x2,
            3 => TS_A::B_0x3,
            4 => TS_A::B_0x4,
            5 => TS_A::B_0x5,
            6 => TS_A::B_0x6,
            7 => TS_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Trigger 0 (tim_itr0)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS_A::B_0x0
    }
    #[doc = "Internal Trigger 1 (tim_itr1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS_A::B_0x1
    }
    #[doc = "Internal Trigger 2 (tim_itr2)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TS_A::B_0x2
    }
    #[doc = "Internal Trigger 3 (tim_itr3)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TS_A::B_0x3
    }
    #[doc = "tim_ti1 Edge Detector (tim_ti1f_ed)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == TS_A::B_0x4
    }
    #[doc = "Filtered Timer Input 1 (tim_ti1fp1)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == TS_A::B_0x5
    }
    #[doc = "Filtered Timer Input 2 (tim_ti2fp2)"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == TS_A::B_0x6
    }
    #[doc = "External Trigger input (tim_etrf)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == TS_A::B_0x7
    }
}
#[doc = "Field `TS` writer - TS\\[2:0\\]: Trigger selection This bitfield is combined with TS\\[4:3\\] bits. This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 605: TIMx internal trigger connection for more details on tim_itrx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS_A, crate::Safe>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (tim_itr0)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x0)
    }
    #[doc = "Internal Trigger 1 (tim_itr1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x1)
    }
    #[doc = "Internal Trigger 2 (tim_itr2)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x2)
    }
    #[doc = "Internal Trigger 3 (tim_itr3)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x3)
    }
    #[doc = "tim_ti1 Edge Detector (tim_ti1f_ed)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x4)
    }
    #[doc = "Filtered Timer Input 1 (tim_ti1fp1)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x5)
    }
    #[doc = "Filtered Timer Input 2 (tim_ti2fp2)"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x6)
    }
    #[doc = "External Trigger input (tim_etrf)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(TS_A::B_0x7)
    }
}
#[doc = "Master/slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: The effect of an event on the trigger input (tim_trgi) is delayed to allow a perfect synchronization between the current timer and its slaves (through tim_trgo). It is useful if we want to synchronize several timers on a single external event."]
    B_0x1 = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master/slave mode"]
pub type MSM_R = crate::BitReader<MSM_A>;
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::B_0x0,
            true => MSM_A::B_0x1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSM_A::B_0x0
    }
    #[doc = "The effect of an event on the trigger input (tim_trgi) is delayed to allow a perfect synchronization between the current timer and its slaves (through tim_trgo). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSM_A::B_0x1
    }
}
#[doc = "Field `MSM` writer - Master/slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG, MSM_A>;
impl<'a, REG> MSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::B_0x0)
    }
    #[doc = "The effect of an event on the trigger input (tim_trgi) is delayed to allow a perfect synchronization between the current timer and its slaves (through tim_trgo). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::B_0x1)
    }
}
#[doc = "External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF_A {
    #[doc = "0: No filter, sampling is done at f DTS"]
    B_0x0 = 0,
    #[doc = "1: f SAMPLING =f tim_ker_ck , N=2"]
    B_0x1 = 1,
    #[doc = "2: f SAMPLING =f tim_ker_ck , N=4"]
    B_0x2 = 2,
    #[doc = "3: f SAMPLING =f tim_ker_ck , N=8"]
    B_0x3 = 3,
    #[doc = "4: f SAMPLING =f DTS /2, N=6"]
    B_0x4 = 4,
    #[doc = "5: f SAMPLING =f DTS /2, N=8"]
    B_0x5 = 5,
    #[doc = "6: f SAMPLING =f DTS /4, N=6"]
    B_0x6 = 6,
    #[doc = "7: f SAMPLING =f DTS /4, N=8"]
    B_0x7 = 7,
    #[doc = "8: f SAMPLING =f DTS /8, N=6"]
    B_0x8 = 8,
    #[doc = "9: f SAMPLING =f DTS /8, N=8"]
    B_0x9 = 9,
    #[doc = "10: f SAMPLING =f DTS /16, N=5"]
    B_0xA = 10,
    #[doc = "11: f SAMPLING =f DTS /16, N=6"]
    B_0xB = 11,
    #[doc = "12: f SAMPLING =f DTS /16, N=8"]
    B_0xC = 12,
    #[doc = "13: f SAMPLING =f DTS /32, N=5"]
    B_0xD = 13,
    #[doc = "14: f SAMPLING =f DTS /32, N=6"]
    B_0xE = 14,
    #[doc = "15: f SAMPLING =f DTS /32, N=8"]
    B_0xF = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETF_A {
    type Ux = u8;
}
impl crate::IsEnum for ETF_A {}
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_R = crate::FieldReader<ETF_A>;
impl ETF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::B_0x0,
            1 => ETF_A::B_0x1,
            2 => ETF_A::B_0x2,
            3 => ETF_A::B_0x3,
            4 => ETF_A::B_0x4,
            5 => ETF_A::B_0x5,
            6 => ETF_A::B_0x6,
            7 => ETF_A::B_0x7,
            8 => ETF_A::B_0x8,
            9 => ETF_A::B_0x9,
            10 => ETF_A::B_0xA,
            11 => ETF_A::B_0xB,
            12 => ETF_A::B_0xC,
            13 => ETF_A::B_0xD,
            14 => ETF_A::B_0xE,
            15 => ETF_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at f DTS"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETF_A::B_0x0
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETF_A::B_0x1
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ETF_A::B_0x2
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ETF_A::B_0x3
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ETF_A::B_0x4
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ETF_A::B_0x5
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ETF_A::B_0x6
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ETF_A::B_0x7
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == ETF_A::B_0x8
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == ETF_A::B_0x9
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == ETF_A::B_0xA
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == ETF_A::B_0xB
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == ETF_A::B_0xC
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == ETF_A::B_0xD
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == ETF_A::B_0xE
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == ETF_A::B_0xF
    }
}
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETF_A, crate::Safe>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at f DTS"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x0)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x1)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x2)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x3)
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x4)
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x5)
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x6)
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x7)
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x8)
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0x9)
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xA)
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xB)
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xC)
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xD)
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xE)
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0xF)
    }
}
#[doc = "External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS_A {
    #[doc = "0: Prescaler OFF"]
    B_0x0 = 0,
    #[doc = "1: tim_etr_in frequency divided by 2"]
    B_0x1 = 1,
    #[doc = "2: tim_etr_in frequency divided by 4"]
    B_0x2 = 2,
    #[doc = "3: tim_etr_in frequency divided by 8"]
    B_0x3 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETPS_A {
    type Ux = u8;
}
impl crate::IsEnum for ETPS_A {}
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_R = crate::FieldReader<ETPS_A>;
impl ETPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::B_0x0,
            1 => ETPS_A::B_0x1,
            2 => ETPS_A::B_0x2,
            3 => ETPS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETPS_A::B_0x0
    }
    #[doc = "tim_etr_in frequency divided by 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETPS_A::B_0x1
    }
    #[doc = "tim_etr_in frequency divided by 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ETPS_A::B_0x2
    }
    #[doc = "tim_etr_in frequency divided by 8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ETPS_A::B_0x3
    }
}
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETPS_A, crate::Safe>;
impl<'a, REG> ETPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0x0)
    }
    #[doc = "tim_etr_in frequency divided by 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0x1)
    }
    #[doc = "tim_etr_in frequency divided by 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0x2)
    }
    #[doc = "tim_etr_in frequency divided by 8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0x3)
    }
}
#[doc = "External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE_A {
    #[doc = "0: External clock mode 2 disabled"]
    B_0x0 = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the tim_etrf signal."]
    B_0x1 = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_R = crate::BitReader<ECE_A>;
impl ECE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::B_0x0,
            true => ECE_A::B_0x1,
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECE_A::B_0x0
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the tim_etrf signal."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECE_A::B_0x1
    }
}
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG, ECE_A>;
impl<'a, REG> ECE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECE_A::B_0x0)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the tim_etrf signal."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECE_A::B_0x1)
    }
}
#[doc = "External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP_A {
    #[doc = "0: tim_etr_in is non-inverted, active at high level or rising edge."]
    B_0x0 = 0,
    #[doc = "1: tim_etr_in is inverted, active at low level or falling edge."]
    B_0x1 = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_R = crate::BitReader<ETP_A>;
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::B_0x0,
            true => ETP_A::B_0x1,
        }
    }
    #[doc = "tim_etr_in is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETP_A::B_0x0
    }
    #[doc = "tim_etr_in is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETP_A::B_0x1
    }
}
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG, ETP_A>;
impl<'a, REG> ETP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_etr_in is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::B_0x0)
    }
    #[doc = "tim_etr_in is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::B_0x1)
    }
}
#[doc = "SMS\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMS_1_A {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B_0x0 = 0,
    #[doc = "1: Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    B_0x1 = 1,
}
impl From<SMS_1_A> for bool {
    #[inline(always)]
    fn from(variant: SMS_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS_1` reader - SMS\\[3\\]"]
pub type SMS_1_R = crate::BitReader<SMS_1_A>;
impl SMS_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMS_1_A {
        match self.bits {
            false => SMS_1_A::B_0x0,
            true => SMS_1_A::B_0x1,
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMS_1_A::B_0x0
    }
    #[doc = "Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMS_1_A::B_0x1
    }
}
#[doc = "Field `SMS_1` writer - SMS\\[3\\]"]
pub type SMS_1_W<'a, REG> = crate::BitWriter<'a, REG, SMS_1_A>;
impl<'a, REG> SMS_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_1_A::B_0x0)
    }
    #[doc = "Quadrature encoder mode 1, x2 mode- Counter counts up/down on tim_ti1fp1 edge depending on tim_ti2fp2 level."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS_1_A::B_0x1)
    }
}
#[doc = "TS\\[4:3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_1_A {
    #[doc = "0: Internal Trigger 0 (tim_itr0)"]
    B_0x0 = 0,
    #[doc = "1: Internal Trigger 1 (tim_itr1)"]
    B_0x1 = 1,
    #[doc = "2: Internal Trigger 2 (tim_itr2)"]
    B_0x2 = 2,
    #[doc = "3: Internal Trigger 3 (tim_itr3)"]
    B_0x3 = 3,
}
impl From<TS_1_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS_1_A {
    type Ux = u8;
}
impl crate::IsEnum for TS_1_A {}
#[doc = "Field `TS_1` reader - TS\\[4:3\\]"]
pub type TS_1_R = crate::FieldReader<TS_1_A>;
impl TS_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS_1_A {
        match self.bits {
            0 => TS_1_A::B_0x0,
            1 => TS_1_A::B_0x1,
            2 => TS_1_A::B_0x2,
            3 => TS_1_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Trigger 0 (tim_itr0)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS_1_A::B_0x0
    }
    #[doc = "Internal Trigger 1 (tim_itr1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS_1_A::B_0x1
    }
    #[doc = "Internal Trigger 2 (tim_itr2)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TS_1_A::B_0x2
    }
    #[doc = "Internal Trigger 3 (tim_itr3)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TS_1_A::B_0x3
    }
}
#[doc = "Field `TS_1` writer - TS\\[4:3\\]"]
pub type TS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TS_1_A, crate::Safe>;
impl<'a, REG> TS_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (tim_itr0)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS_1_A::B_0x0)
    }
    #[doc = "Internal Trigger 1 (tim_itr1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS_1_A::B_0x1)
    }
    #[doc = "Internal Trigger 2 (tim_itr2)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TS_1_A::B_0x2)
    }
    #[doc = "Internal Trigger 3 (tim_itr3)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TS_1_A::B_0x3)
    }
}
#[doc = "SMS preload enable This bit selects whether the SMS\\[3:0\\] bitfield is preloaded\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMSPE_A {
    #[doc = "0: SMS\\[3:0\\] bitfield is not preloaded"]
    B_0x0 = 0,
    #[doc = "1: SMS\\[3:0\\] preload is enabled"]
    B_0x1 = 1,
}
impl From<SMSPE_A> for bool {
    #[inline(always)]
    fn from(variant: SMSPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMSPE` reader - SMS preload enable This bit selects whether the SMS\\[3:0\\] bitfield is preloaded"]
pub type SMSPE_R = crate::BitReader<SMSPE_A>;
impl SMSPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSPE_A {
        match self.bits {
            false => SMSPE_A::B_0x0,
            true => SMSPE_A::B_0x1,
        }
    }
    #[doc = "SMS\\[3:0\\] bitfield is not preloaded"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMSPE_A::B_0x0
    }
    #[doc = "SMS\\[3:0\\] preload is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMSPE_A::B_0x1
    }
}
#[doc = "Field `SMSPE` writer - SMS preload enable This bit selects whether the SMS\\[3:0\\] bitfield is preloaded"]
pub type SMSPE_W<'a, REG> = crate::BitWriter<'a, REG, SMSPE_A>;
impl<'a, REG> SMSPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMS\\[3:0\\] bitfield is not preloaded"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPE_A::B_0x0)
    }
    #[doc = "SMS\\[3:0\\] preload is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPE_A::B_0x1)
    }
}
#[doc = "SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\] bitfield transfer from preload to active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMSPS_A {
    #[doc = "0: The transfer is triggered by the Timer's Update event"]
    B_0x0 = 0,
    #[doc = "1: The transfer is triggered by the Index event"]
    B_0x1 = 1,
}
impl From<SMSPS_A> for bool {
    #[inline(always)]
    fn from(variant: SMSPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMSPS` reader - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\] bitfield transfer from preload to active"]
pub type SMSPS_R = crate::BitReader<SMSPS_A>;
impl SMSPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSPS_A {
        match self.bits {
            false => SMSPS_A::B_0x0,
            true => SMSPS_A::B_0x1,
        }
    }
    #[doc = "The transfer is triggered by the Timer's Update event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMSPS_A::B_0x0
    }
    #[doc = "The transfer is triggered by the Index event"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMSPS_A::B_0x1
    }
}
#[doc = "Field `SMSPS` writer - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\] bitfield transfer from preload to active"]
pub type SMSPS_W<'a, REG> = crate::BitWriter<'a, REG, SMSPS_A>;
impl<'a, REG> SMSPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transfer is triggered by the Timer's Update event"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPS_A::B_0x0)
    }
    #[doc = "The transfer is triggered by the Index event"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - SMS\\[2:0\\]: Slave mode selection"]
    #[inline(always)]
    pub fn SMS(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    pub fn OCCS(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - TS\\[2:0\\]: Trigger selection This bitfield is combined with TS\\[4:3\\] bits. This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 605: TIMx internal trigger connection for more details on tim_itrx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn TS(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn MSM(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ETF(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    pub fn ETPS(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    pub fn ECE(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    pub fn ETP(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SMS\\[3\\]"]
    #[inline(always)]
    pub fn SMS_1(&self) -> SMS_1_R {
        SMS_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - TS\\[4:3\\]"]
    #[inline(always)]
    pub fn TS_1(&self) -> TS_1_R {
        TS_1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\] bitfield is preloaded"]
    #[inline(always)]
    pub fn SMSPE(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\] bitfield transfer from preload to active"]
    #[inline(always)]
    pub fn SMSPS(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS\\[2:0\\]: Slave mode selection"]
    #[inline(always)]
    pub fn SMS(&mut self) -> SMS_W<'_, SMCR_SPEC> {
        SMS_W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    pub fn OCCS(&mut self) -> OCCS_W<'_, SMCR_SPEC> {
        OCCS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - TS\\[2:0\\]: Trigger selection This bitfield is combined with TS\\[4:3\\] bits. This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 605: TIMx internal trigger connection for more details on tim_itrx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn TS(&mut self) -> TS_W<'_, SMCR_SPEC> {
        TS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn MSM(&mut self) -> MSM_W<'_, SMCR_SPEC> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ETF(&mut self) -> ETF_W<'_, SMCR_SPEC> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    pub fn ETPS(&mut self) -> ETPS_W<'_, SMCR_SPEC> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    pub fn ECE(&mut self) -> ECE_W<'_, SMCR_SPEC> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    pub fn ETP(&mut self) -> ETP_W<'_, SMCR_SPEC> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - SMS\\[3\\]"]
    #[inline(always)]
    pub fn SMS_1(&mut self) -> SMS_1_W<'_, SMCR_SPEC> {
        SMS_1_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - TS\\[4:3\\]"]
    #[inline(always)]
    pub fn TS_1(&mut self) -> TS_1_W<'_, SMCR_SPEC> {
        TS_1_W::new(self, 20)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\] bitfield is preloaded"]
    #[inline(always)]
    pub fn SMSPE(&mut self) -> SMSPE_W<'_, SMCR_SPEC> {
        SMSPE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\] bitfield transfer from preload to active"]
    #[inline(always)]
    pub fn SMSPS(&mut self) -> SMSPS_W<'_, SMCR_SPEC> {
        SMSPS_W::new(self, 25)
    }
}
#[doc = "TIM1 slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {}
