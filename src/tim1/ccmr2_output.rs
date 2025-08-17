#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<CCMR2_OUTPUT_SPEC>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<CCMR2_OUTPUT_SPEC>;
#[doc = "Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "0: CC3 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    B_0x1 = 1,
    #[doc = "2: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    B_0x2 = 2,
    #[doc = "3: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC3S_A {}
#[doc = "Field `CC3S` reader - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type CC3S_R = crate::FieldReader<CC3S_A>;
impl CC3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC3S_A {
        match self.bits {
            0 => CC3S_A::B_0x0,
            1 => CC3S_A::B_0x1,
            2 => CC3S_A::B_0x2,
            3 => CC3S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC3S_A::B_0x0
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC3S_A::B_0x1
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC3S_A::B_0x2
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC3S_A::B_0x3
    }
}
#[doc = "Field `CC3S` writer - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S_A, crate::Safe>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x0)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x1)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x2)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x3)
    }
}
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable"]
pub type OC3FE_R = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable"]
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable"]
pub type OC3PE_R = crate::BitReader;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable"]
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "OC3M\\[2:0\\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC3M_A {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR3 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B_0x0 = 0,
    #[doc = "1: Set channel 3 to active level on match. tim_oc3ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    B_0x1 = 1,
    #[doc = "2: Set channel 3 to inactive level on match. tim_oc3ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    B_0x2 = 2,
    #[doc = "3: Toggle - tim_oc3ref toggles when TIMx_CNT=TIMx_CCR3."]
    B_0x3 = 3,
    #[doc = "4: Force inactive level - tim_oc3ref is forced low."]
    B_0x4 = 4,
    #[doc = "5: Force active level - tim_oc3ref is forced high."]
    B_0x5 = 5,
    #[doc = "6: PWM mode 1 - In upcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive. In downcounting, channel 3 is inactive (tim_oc3ref='0') as long as TIMx_CNTTIMx_CCR3 else active (tim_oc3ref='1')."]
    B_0x6 = 6,
    #[doc = "7: PWM mode 2 - In upcounting, channel 3 is inactive as long as TIMx_CNTTIMx_CCR3 else active. In downcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive."]
    B_0x7 = 7,
}
impl From<OC3M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC3M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC3M_A {
    type Ux = u8;
}
impl crate::IsEnum for OC3M_A {}
#[doc = "Field `OC3M` reader - OC3M\\[2:0\\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated."]
pub type OC3M_R = crate::FieldReader<OC3M_A>;
impl OC3M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC3M_A {
        match self.bits {
            0 => OC3M_A::B_0x0,
            1 => OC3M_A::B_0x1,
            2 => OC3M_A::B_0x2,
            3 => OC3M_A::B_0x3,
            4 => OC3M_A::B_0x4,
            5 => OC3M_A::B_0x5,
            6 => OC3M_A::B_0x6,
            7 => OC3M_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR3 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC3M_A::B_0x0
    }
    #[doc = "Set channel 3 to active level on match. tim_oc3ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC3M_A::B_0x1
    }
    #[doc = "Set channel 3 to inactive level on match. tim_oc3ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OC3M_A::B_0x2
    }
    #[doc = "Toggle - tim_oc3ref toggles when TIMx_CNT=TIMx_CCR3."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OC3M_A::B_0x3
    }
    #[doc = "Force inactive level - tim_oc3ref is forced low."]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == OC3M_A::B_0x4
    }
    #[doc = "Force active level - tim_oc3ref is forced high."]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == OC3M_A::B_0x5
    }
    #[doc = "PWM mode 1 - In upcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive. In downcounting, channel 3 is inactive (tim_oc3ref='0') as long as TIMx_CNTTIMx_CCR3 else active (tim_oc3ref='1')."]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == OC3M_A::B_0x6
    }
    #[doc = "PWM mode 2 - In upcounting, channel 3 is inactive as long as TIMx_CNTTIMx_CCR3 else active. In downcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive."]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OC3M_A::B_0x7
    }
}
#[doc = "Field `OC3M` writer - OC3M\\[2:0\\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated."]
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC3M_A, crate::Safe>;
impl<'a, REG> OC3M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR3 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x0)
    }
    #[doc = "Set channel 3 to active level on match. tim_oc3ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x1)
    }
    #[doc = "Set channel 3 to inactive level on match. tim_oc3ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3)."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x2)
    }
    #[doc = "Toggle - tim_oc3ref toggles when TIMx_CNT=TIMx_CCR3."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x3)
    }
    #[doc = "Force inactive level - tim_oc3ref is forced low."]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x4)
    }
    #[doc = "Force active level - tim_oc3ref is forced high."]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x5)
    }
    #[doc = "PWM mode 1 - In upcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive. In downcounting, channel 3 is inactive (tim_oc3ref='0') as long as TIMx_CNTTIMx_CCR3 else active (tim_oc3ref='1')."]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x6)
    }
    #[doc = "PWM mode 2 - In upcounting, channel 3 is inactive as long as TIMx_CNTTIMx_CCR3 else active. In downcounting, channel 3 is active as long as TIMx_CNTTIMx_CCR3 else inactive."]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_A::B_0x7)
    }
}
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable"]
pub type OC3CE_R = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable"]
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S_A {
    #[doc = "0: CC4 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    B_0x1 = 1,
    #[doc = "2: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    B_0x2 = 2,
    #[doc = "3: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC4S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC4S_A {}
#[doc = "Field `CC4S` reader - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type CC4S_R = crate::FieldReader<CC4S_A>;
impl CC4S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC4S_A {
        match self.bits {
            0 => CC4S_A::B_0x0,
            1 => CC4S_A::B_0x1,
            2 => CC4S_A::B_0x2,
            3 => CC4S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC4S_A::B_0x0
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC4S_A::B_0x1
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC4S_A::B_0x2
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC4S_A::B_0x3
    }
}
#[doc = "Field `CC4S` writer - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S_A, crate::Safe>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x0)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x1)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x2)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x3)
    }
}
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable"]
pub type OC4FE_R = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable"]
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable"]
pub type OC4PE_R = crate::BitReader;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable"]
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - OC4M\\[2:0\\]: Output compare 4 mode Refer to OC3M\\[3:0\\] bit description"]
pub type OC4M_R = crate::FieldReader;
#[doc = "Field `OC4M` writer - OC4M\\[2:0\\]: Output compare 4 mode Refer to OC3M\\[3:0\\] bit description"]
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable"]
pub type OC4CE_R = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable"]
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_1` reader - OC3M\\[3\\]"]
pub type OC3M_1_R = crate::BitReader;
#[doc = "Field `OC3M_1` writer - OC3M\\[3\\]"]
pub type OC3M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_1` reader - OC4M\\[3\\]"]
pub type OC4M_1_R = crate::BitReader;
#[doc = "Field `OC4M_1` writer - OC4M\\[3\\]"]
pub type OC4M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC3S(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn OC3FE(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn OC3PE(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC3M\\[2:0\\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated."]
    #[inline(always)]
    pub fn OC3M(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn OC3CE(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC4S(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn OC4FE(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn OC4PE(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC4M\\[2:0\\]: Output compare 4 mode Refer to OC3M\\[3:0\\] bit description"]
    #[inline(always)]
    pub fn OC4M(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn OC4CE(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC3M\\[3\\]"]
    #[inline(always)]
    pub fn OC3M_1(&self) -> OC3M_1_R {
        OC3M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC4M\\[3\\]"]
    #[inline(always)]
    pub fn OC4M_1(&self) -> OC4M_1_R {
        OC4M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC3S(&mut self) -> CC3S_W<'_, CCMR2_OUTPUT_SPEC> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn OC3FE(&mut self) -> OC3FE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC3FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn OC3PE(&mut self) -> OC3PE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC3PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC3M\\[2:0\\]: Output compare 3 mode These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3 and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active level depends on CC3P and CC3NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode. On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits only when a COM event is generated."]
    #[inline(always)]
    pub fn OC3M(&mut self) -> OC3M_W<'_, CCMR2_OUTPUT_SPEC> {
        OC3M_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn OC3CE(&mut self) -> OC3CE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC3CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC4S(&mut self) -> CC4S_W<'_, CCMR2_OUTPUT_SPEC> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn OC4FE(&mut self) -> OC4FE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC4FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn OC4PE(&mut self) -> OC4PE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC4PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC4M\\[2:0\\]: Output compare 4 mode Refer to OC3M\\[3:0\\] bit description"]
    #[inline(always)]
    pub fn OC4M(&mut self) -> OC4M_W<'_, CCMR2_OUTPUT_SPEC> {
        OC4M_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn OC4CE(&mut self) -> OC4CE_W<'_, CCMR2_OUTPUT_SPEC> {
        OC4CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - OC3M\\[3\\]"]
    #[inline(always)]
    pub fn OC3M_1(&mut self) -> OC3M_1_W<'_, CCMR2_OUTPUT_SPEC> {
        OC3M_1_W::new(self, 16)
    }
    #[doc = "Bit 24 - OC4M\\[3\\]"]
    #[inline(always)]
    pub fn OC4M_1(&mut self) -> OC4M_1_W<'_, CCMR2_OUTPUT_SPEC> {
        OC4M_1_W::new(self, 24)
    }
}
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for CCMR2_OUTPUT_SPEC {}
