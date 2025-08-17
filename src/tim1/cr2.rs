#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC_A {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    B_0x0 = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on tim_trgi, depending on the CCUS bit)."]
    B_0x1 = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader<CCPC_A>;
impl CCPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::B_0x0,
            true => CCPC_A::B_0x1,
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CCPC_A::B_0x0
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on tim_trgi, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CCPC_A::B_0x1
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG, CCPC_A>;
impl<'a, REG> CCPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC_A::B_0x0)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on tim_trgi, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC_A::B_0x1)
    }
}
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS_A {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    B_0x0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on tim_trgi"]
    B_0x1 = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader<CCUS_A>;
impl CCUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::B_0x0,
            true => CCUS_A::B_0x1,
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CCUS_A::B_0x0
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on tim_trgi"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CCUS_A::B_0x1
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS_A>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS_A::B_0x0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on tim_trgi"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS_A::B_0x1)
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    B_0x0 = 0,
    #[doc = "1: CCx DMA requests sent when update event occurs"]
    B_0x1 = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS_A>;
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::B_0x0,
            true => CCDS_A::B_0x1,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CCDS_A::B_0x0
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CCDS_A::B_0x1
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS_A>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS_A::B_0x0)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS_A::B_0x1)
    }
}
#[doc = "MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on tim_trgo is delayed compared to the actual reset."]
    B_0x0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0x1 = 1,
    #[doc = "2: Update - The update event is selected as trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    B_0x2 = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (tim_trgo)."]
    B_0x3 = 3,
    #[doc = "4: Compare - tim_oc1refc signal is used as trigger output (tim_trgo)"]
    B_0x4 = 4,
    #[doc = "5: Compare - tim_oc2refc signal is used as trigger output (tim_trgo)"]
    B_0x5 = 5,
    #[doc = "6: Compare - tim_oc3refc signal is used as trigger output (tim_trgo)"]
    B_0x6 = 6,
    #[doc = "7: Compare - tim_oc4refc signal is used as trigger output (tim_trgo)"]
    B_0x7 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS_A {
    type Ux = u8;
}
impl crate::IsEnum for MMS_A {}
#[doc = "Field `MMS` reader - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<MMS_A>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::B_0x0,
            1 => MMS_A::B_0x1,
            2 => MMS_A::B_0x2,
            3 => MMS_A::B_0x3,
            4 => MMS_A::B_0x4,
            5 => MMS_A::B_0x5,
            6 => MMS_A::B_0x6,
            7 => MMS_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on tim_trgo is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MMS_A::B_0x0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MMS_A::B_0x1
    }
    #[doc = "Update - The update event is selected as trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MMS_A::B_0x2
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (tim_trgo)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MMS_A::B_0x3
    }
    #[doc = "Compare - tim_oc1refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MMS_A::B_0x4
    }
    #[doc = "Compare - tim_oc2refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MMS_A::B_0x5
    }
    #[doc = "Compare - tim_oc3refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MMS_A::B_0x6
    }
    #[doc = "Compare - tim_oc4refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MMS_A::B_0x7
    }
}
#[doc = "Field `MMS` writer - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS_A, crate::Safe>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on tim_trgo is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x1)
    }
    #[doc = "Update - The update event is selected as trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x2)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (tim_trgo)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x3)
    }
    #[doc = "Compare - tim_oc1refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x4)
    }
    #[doc = "Compare - tim_oc2refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x5)
    }
    #[doc = "Compare - tim_oc3refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x6)
    }
    #[doc = "Compare - tim_oc4refc signal is used as trigger output (tim_trgo)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x7)
    }
}
#[doc = "tim_ti1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S_A {
    #[doc = "0: The tim_ti1_in\\[15..0\\] multiplexer output is connected to tim_ti1 input"]
    B_0x0 = 0,
    #[doc = "1: tim_ti1_in\\[15..0\\], tim_ti2_in\\[15..0\\] and tim_ti3_in\\[15..0\\] multiplexers outputs are XORed and connected to the tim_ti1 input"]
    B_0x1 = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - tim_ti1 selection"]
pub type TI1S_R = crate::BitReader<TI1S_A>;
impl TI1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::B_0x0,
            true => TI1S_A::B_0x1,
        }
    }
    #[doc = "The tim_ti1_in\\[15..0\\] multiplexer output is connected to tim_ti1 input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TI1S_A::B_0x0
    }
    #[doc = "tim_ti1_in\\[15..0\\], tim_ti2_in\\[15..0\\] and tim_ti3_in\\[15..0\\] multiplexers outputs are XORed and connected to the tim_ti1 input"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TI1S_A::B_0x1
    }
}
#[doc = "Field `TI1S` writer - tim_ti1 selection"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S_A>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The tim_ti1_in\\[15..0\\] multiplexer output is connected to tim_ti1 input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S_A::B_0x0)
    }
    #[doc = "tim_ti1_in\\[15..0\\], tim_ti2_in\\[15..0\\] and tim_ti3_in\\[15..0\\] multiplexers outputs are XORed and connected to the tim_ti1 input"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S_A::B_0x1)
    }
}
#[doc = "Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1_A {
    #[doc = "0: tim_oc1=0 (after a dead-time) when MOE=0"]
    B_0x0 = 0,
    #[doc = "1: tim_oc1=1 (after a dead-time) when MOE=0"]
    B_0x1 = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_R = crate::BitReader<OIS1_A>;
impl OIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::B_0x0,
            true => OIS1_A::B_0x1,
        }
    }
    #[doc = "tim_oc1=0 (after a dead-time) when MOE=0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OIS1_A::B_0x0
    }
    #[doc = "tim_oc1=1 (after a dead-time) when MOE=0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OIS1_A::B_0x1
    }
}
#[doc = "Field `OIS1` writer - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG, OIS1_A>;
impl<'a, REG> OIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_oc1=0 (after a dead-time) when MOE=0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1_A::B_0x0)
    }
    #[doc = "tim_oc1=1 (after a dead-time) when MOE=0"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1_A::B_0x1)
    }
}
#[doc = "Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N_A {
    #[doc = "0: tim_oc1n=0 after a dead-time when MOE=0"]
    B_0x0 = 0,
    #[doc = "1: tim_oc1n=1 after a dead-time when MOE=0"]
    B_0x1 = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_R = crate::BitReader<OIS1N_A>;
impl OIS1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::B_0x0,
            true => OIS1N_A::B_0x1,
        }
    }
    #[doc = "tim_oc1n=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OIS1N_A::B_0x0
    }
    #[doc = "tim_oc1n=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OIS1N_A::B_0x1
    }
}
#[doc = "Field `OIS1N` writer - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N_A>;
impl<'a, REG> OIS1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_oc1n=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N_A::B_0x0)
    }
    #[doc = "tim_oc1n=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N_A::B_0x1)
    }
}
#[doc = "Field `OIS2` reader - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub type OIS2_R = crate::BitReader;
#[doc = "Field `OIS2` writer - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub type OIS2N_R = crate::BitReader;
#[doc = "Field `OIS2N` writer - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub type OIS3_R = crate::BitReader;
#[doc = "Field `OIS3` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub type OIS3N_R = crate::BitReader;
#[doc = "Field `OIS3N` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub type OIS4_R = crate::BitReader;
#[doc = "Field `OIS4` writer - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4N` reader - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub type OIS4N_R = crate::BitReader;
#[doc = "Field `OIS4N` writer - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub type OIS4N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub type OIS5_R = crate::BitReader;
#[doc = "Field `OIS5` writer - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub type OIS6_R = crate::BitReader;
#[doc = "Field `OIS6` writer - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS2_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on tim_trgo2 is delayed compared to the actual reset."]
    B_0x0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0x1 = 1,
    #[doc = "2: Update - the update event is selected as trigger output (tim_trgo2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    B_0x2 = 2,
    #[doc = "3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (tim_trgo2)."]
    B_0x3 = 3,
    #[doc = "4: Compare - tim_oc1refc signal is used as trigger output (tim_trgo2)"]
    B_0x4 = 4,
    #[doc = "5: Compare - tim_oc2refc signal is used as trigger output (tim_trgo2)"]
    B_0x5 = 5,
    #[doc = "6: Compare - tim_oc3refc signal is used as trigger output (tim_trgo2)"]
    B_0x6 = 6,
    #[doc = "7: Compare - tim_oc4refc signal is used as trigger output (tim_trgo2)"]
    B_0x7 = 7,
    #[doc = "8: Compare - tim_oc5refc signal is used as trigger output (tim_trgo2)"]
    B_0x8 = 8,
    #[doc = "9: Compare - tim_oc6refc signal is used as trigger output (tim_trgo2)"]
    B_0x9 = 9,
    #[doc = "10: Compare Pulse - tim_oc4refc rising or falling edges generate pulses on tim_trgo2"]
    B_0xA = 10,
    #[doc = "11: Compare pulse - tim_oc6refc rising or falling edges generate pulses on tim_trgo2"]
    B_0xB = 11,
    #[doc = "12: Compare pulse - tim_oc4refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    B_0xC = 12,
    #[doc = "13: Compare pulse - tim_oc4refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    B_0xD = 13,
    #[doc = "14: Compare pulse - tim_oc5refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    B_0xE = 14,
    #[doc = "15: Compare pulse - tim_oc5refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    B_0xF = 15,
}
impl From<MMS2_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS2_A {
    type Ux = u8;
}
impl crate::IsEnum for MMS2_A {}
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::FieldReader<MMS2_A>;
impl MMS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMS2_A {
        match self.bits {
            0 => MMS2_A::B_0x0,
            1 => MMS2_A::B_0x1,
            2 => MMS2_A::B_0x2,
            3 => MMS2_A::B_0x3,
            4 => MMS2_A::B_0x4,
            5 => MMS2_A::B_0x5,
            6 => MMS2_A::B_0x6,
            7 => MMS2_A::B_0x7,
            8 => MMS2_A::B_0x8,
            9 => MMS2_A::B_0x9,
            10 => MMS2_A::B_0xA,
            11 => MMS2_A::B_0xB,
            12 => MMS2_A::B_0xC,
            13 => MMS2_A::B_0xD,
            14 => MMS2_A::B_0xE,
            15 => MMS2_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on tim_trgo2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MMS2_A::B_0x0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MMS2_A::B_0x1
    }
    #[doc = "Update - the update event is selected as trigger output (tim_trgo2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MMS2_A::B_0x2
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (tim_trgo2)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MMS2_A::B_0x3
    }
    #[doc = "Compare - tim_oc1refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MMS2_A::B_0x4
    }
    #[doc = "Compare - tim_oc2refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MMS2_A::B_0x5
    }
    #[doc = "Compare - tim_oc3refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MMS2_A::B_0x6
    }
    #[doc = "Compare - tim_oc4refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MMS2_A::B_0x7
    }
    #[doc = "Compare - tim_oc5refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == MMS2_A::B_0x8
    }
    #[doc = "Compare - tim_oc6refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == MMS2_A::B_0x9
    }
    #[doc = "Compare Pulse - tim_oc4refc rising or falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == MMS2_A::B_0xA
    }
    #[doc = "Compare pulse - tim_oc6refc rising or falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == MMS2_A::B_0xB
    }
    #[doc = "Compare pulse - tim_oc4refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == MMS2_A::B_0xC
    }
    #[doc = "Compare pulse - tim_oc4refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == MMS2_A::B_0xD
    }
    #[doc = "Compare pulse - tim_oc5refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == MMS2_A::B_0xE
    }
    #[doc = "Compare pulse - tim_oc5refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MMS2_A::B_0xF
    }
}
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MMS2_A, crate::Safe>;
impl<'a, REG> MMS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (tim_trgo2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on tim_trgo2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (tim_trgo2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x1)
    }
    #[doc = "Update - the update event is selected as trigger output (tim_trgo2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x2)
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (tim_trgo2)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x3)
    }
    #[doc = "Compare - tim_oc1refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x4)
    }
    #[doc = "Compare - tim_oc2refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x5)
    }
    #[doc = "Compare - tim_oc3refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x6)
    }
    #[doc = "Compare - tim_oc4refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x7)
    }
    #[doc = "Compare - tim_oc5refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x8)
    }
    #[doc = "Compare - tim_oc6refc signal is used as trigger output (tim_trgo2)"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0x9)
    }
    #[doc = "Compare Pulse - tim_oc4refc rising or falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xA)
    }
    #[doc = "Compare pulse - tim_oc6refc rising or falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xB)
    }
    #[doc = "Compare pulse - tim_oc4refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xC)
    }
    #[doc = "Compare pulse - tim_oc4refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xD)
    }
    #[doc = "Compare pulse - tim_oc5refc or tim_oc6refc rising edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xE)
    }
    #[doc = "Compare pulse - tim_oc5refc rising or tim_oc6refc falling edges generate pulses on tim_trgo2"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0xF)
    }
}
#[doc = "Field `MMS_1` reader - MMS\\[3\\]"]
pub type MMS_1_R = crate::BitReader;
#[doc = "Field `MMS_1` writer - MMS\\[3\\]"]
pub type MMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn CCPC(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn CCUS(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn CCDS(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    pub fn TI1S(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OIS1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OIS1N(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS2N(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS3N(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS4N(&self) -> OIS4N_R {
        OIS4N_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    pub fn MMS_1(&self) -> MMS_1_R {
        MMS_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn CCPC(&mut self) -> CCPC_W<'_, CR2_SPEC> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn CCUS(&mut self) -> CCUS_W<'_, CR2_SPEC> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn CCDS(&mut self) -> CCDS_W<'_, CR2_SPEC> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS(&mut self) -> MMS_W<'_, CR2_SPEC> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    pub fn TI1S(&mut self) -> TI1S_W<'_, CR2_SPEC> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OIS1(&mut self) -> OIS1_W<'_, CR2_SPEC> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OIS1N(&mut self) -> OIS1N_W<'_, CR2_SPEC> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS2(&mut self) -> OIS2_W<'_, CR2_SPEC> {
        OIS2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS2N(&mut self) -> OIS2N_W<'_, CR2_SPEC> {
        OIS2N_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS3(&mut self) -> OIS3_W<'_, CR2_SPEC> {
        OIS3_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS3N(&mut self) -> OIS3N_W<'_, CR2_SPEC> {
        OIS3N_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS4(&mut self) -> OIS4_W<'_, CR2_SPEC> {
        OIS4_W::new(self, 14)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn OIS4N(&mut self) -> OIS4N_W<'_, CR2_SPEC> {
        OIS4N_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS5(&mut self) -> OIS5_W<'_, CR2_SPEC> {
        OIS5_W::new(self, 16)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn OIS6(&mut self) -> OIS6_W<'_, CR2_SPEC> {
        OIS6_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS2(&mut self) -> MMS2_W<'_, CR2_SPEC> {
        MMS2_W::new(self, 20)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    pub fn MMS_1(&mut self) -> MMS_1_W<'_, CR2_SPEC> {
        MMS_1_W::new(self, 25)
    }
}
#[doc = "TIM1 control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}
