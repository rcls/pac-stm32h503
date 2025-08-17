#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: adc_ker_ck (x = 1) (Asynchronous clock mode), generated at product level (refer to Section 6: Reset and clock control (RCC))"]
    B_0x0 = 0,
    #[doc = "1: adc_hclk/1 (Synchronous clock mode). This configuration must be enabled only if the AHB clock prescaler is set to 1 (HPRE\\[3:0\\] = 0XXX in RCC_CFGR register) and if the system clock has a 50% duty cycle."]
    B_0x1 = 1,
    #[doc = "2: adc_hclk/2 (Synchronous clock mode)"]
    B_0x2 = 2,
    #[doc = "3: adc_hclk/4 (Synchronous clock mode)"]
    B_0x3 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for CKMODE_A {}
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_R = crate::FieldReader<CKMODE_A>;
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::B_0x0,
            1 => CKMODE_A::B_0x1,
            2 => CKMODE_A::B_0x2,
            3 => CKMODE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "adc_ker_ck (x = 1) (Asynchronous clock mode), generated at product level (refer to Section 6: Reset and clock control (RCC))"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKMODE_A::B_0x0
    }
    #[doc = "adc_hclk/1 (Synchronous clock mode). This configuration must be enabled only if the AHB clock prescaler is set to 1 (HPRE\\[3:0\\] = 0XXX in RCC_CFGR register) and if the system clock has a 50% duty cycle."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKMODE_A::B_0x1
    }
    #[doc = "adc_hclk/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CKMODE_A::B_0x2
    }
    #[doc = "adc_hclk/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CKMODE_A::B_0x3
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE_A, crate::Safe>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "adc_ker_ck (x = 1) (Asynchronous clock mode), generated at product level (refer to Section 6: Reset and clock control (RCC))"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0x0)
    }
    #[doc = "adc_hclk/1 (Synchronous clock mode). This configuration must be enabled only if the AHB clock prescaler is set to 1 (HPRE\\[3:0\\] = 0XXX in RCC_CFGR register) and if the system clock has a 50% duty cycle."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0x1)
    }
    #[doc = "adc_hclk/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0x2)
    }
    #[doc = "adc_hclk/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0x3)
    }
}
#[doc = "ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\] = 0b00.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: input ADC clock not divided"]
    B_0x0 = 0,
    #[doc = "1: input ADC clock divided by 2"]
    B_0x1 = 1,
    #[doc = "2: input ADC clock divided by 4"]
    B_0x2 = 2,
    #[doc = "3: input ADC clock divided by 6"]
    B_0x3 = 3,
    #[doc = "4: input ADC clock divided by 8"]
    B_0x4 = 4,
    #[doc = "5: input ADC clock divided by 10"]
    B_0x5 = 5,
    #[doc = "6: input ADC clock divided by 12"]
    B_0x6 = 6,
    #[doc = "7: input ADC clock divided by 16"]
    B_0x7 = 7,
    #[doc = "8: input ADC clock divided by 32"]
    B_0x8 = 8,
    #[doc = "9: input ADC clock divided by 64"]
    B_0x9 = 9,
    #[doc = "10: input ADC clock divided by 128"]
    B_0xA = 10,
    #[doc = "11: input ADC clock divided by 256"]
    B_0xB = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl crate::IsEnum for PRESC_A {}
#[doc = "Field `PRESC` reader - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\] = 0b00."]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::B_0x0),
            1 => Some(PRESC_A::B_0x1),
            2 => Some(PRESC_A::B_0x2),
            3 => Some(PRESC_A::B_0x3),
            4 => Some(PRESC_A::B_0x4),
            5 => Some(PRESC_A::B_0x5),
            6 => Some(PRESC_A::B_0x6),
            7 => Some(PRESC_A::B_0x7),
            8 => Some(PRESC_A::B_0x8),
            9 => Some(PRESC_A::B_0x9),
            10 => Some(PRESC_A::B_0xA),
            11 => Some(PRESC_A::B_0xB),
            _ => None,
        }
    }
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRESC_A::B_0x0
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRESC_A::B_0x1
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PRESC_A::B_0x2
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PRESC_A::B_0x3
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PRESC_A::B_0x4
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PRESC_A::B_0x5
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PRESC_A::B_0x6
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PRESC_A::B_0x7
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == PRESC_A::B_0x8
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == PRESC_A::B_0x9
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == PRESC_A::B_0xA
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == PRESC_A::B_0xB
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\] = 0b00."]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x0)
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x1)
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x2)
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x3)
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x4)
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x5)
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x6)
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x7)
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x8)
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x9)
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0xA)
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0xB)
    }
}
#[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: VREFINT channel disabled"]
    B_0x0 = 0,
    #[doc = "1: VREFINT channel enabled"]
    B_0x1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::B_0x0,
            true => VREFEN_A::B_0x1,
        }
    }
    #[doc = "VREFINT channel disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VREFEN_A::B_0x0
    }
    #[doc = "VREFINT channel enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VREFEN_A::B_0x1
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN_A>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT channel disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0x0)
    }
    #[doc = "VREFINT channel enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0x1)
    }
}
#[doc = "VSENSE enable This bit is set and cleared by software to control VSENSE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor channel disabled"]
    B_0x0 = 0,
    #[doc = "1: Temperature sensor channel enabled"]
    B_0x1 = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_R = crate::BitReader<TSEN_A>;
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::B_0x0,
            true => TSEN_A::B_0x1,
        }
    }
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSEN_A::B_0x0
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSEN_A::B_0x1
    }
}
#[doc = "Field `TSEN` writer - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN_A>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::B_0x0)
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::B_0x1)
    }
}
#[doc = "VBAT enable This bit is set and cleared by software to control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    #[doc = "0: VBAT channel disabled"]
    B_0x0 = 0,
    #[doc = "1: VBAT channel enabled"]
    B_0x1 = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::B_0x0,
            true => VBATEN_A::B_0x1,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VBATEN_A::B_0x0
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VBATEN_A::B_0x1
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN_A>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::B_0x0)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn CKMODE(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\] = 0b00."]
    #[inline(always)]
    pub fn PRESC(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    pub fn VREFEN(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    pub fn TSEN(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    pub fn VBATEN(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn CKMODE(&mut self) -> CKMODE_W<'_, CCR_SPEC> {
        CKMODE_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\] = 0b00."]
    #[inline(always)]
    pub fn PRESC(&mut self) -> PRESC_W<'_, CCR_SPEC> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    pub fn VREFEN(&mut self) -> VREFEN_W<'_, CCR_SPEC> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    pub fn TSEN(&mut self) -> TSEN_W<'_, CCR_SPEC> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    pub fn VBATEN(&mut self) -> VBATEN_W<'_, CCR_SPEC> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {}
