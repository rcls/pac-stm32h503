#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_EN_A {
    #[doc = "0: Temperature sensor 1 disabled"]
    B_0x0 = 0,
    #[doc = "1: Temperature sensor 1 enabled"]
    B_0x1 = 1,
}
impl From<TS1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_EN` reader - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
pub type TS1_EN_R = crate::BitReader<TS1_EN_A>;
impl TS1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_EN_A {
        match self.bits {
            false => TS1_EN_A::B_0x0,
            true => TS1_EN_A::B_0x1,
        }
    }
    #[doc = "Temperature sensor 1 disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_EN_A::B_0x0
    }
    #[doc = "Temperature sensor 1 enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_EN_A::B_0x1
    }
}
#[doc = "Field `TS1_EN` writer - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
pub type TS1_EN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_EN_A>;
impl<'a, REG> TS1_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor 1 disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_EN_A::B_0x0)
    }
    #[doc = "Temperature sensor 1 enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_EN_A::B_0x1)
    }
}
#[doc = "Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_START_A {
    #[doc = "0: No software trigger."]
    B_0x0 = 0,
    #[doc = "1: Software trigger for a frequency measurement. (only if TS1 is ready)."]
    B_0x1 = 1,
}
impl From<TS1_START_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_START` reader - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
pub type TS1_START_R = crate::BitReader<TS1_START_A>;
impl TS1_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_START_A {
        match self.bits {
            false => TS1_START_A::B_0x0,
            true => TS1_START_A::B_0x1,
        }
    }
    #[doc = "No software trigger."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_START_A::B_0x0
    }
    #[doc = "Software trigger for a frequency measurement. (only if TS1 is ready)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_START_A::B_0x1
    }
}
#[doc = "Field `TS1_START` writer - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
pub type TS1_START_W<'a, REG> = crate::BitWriter<'a, REG, TS1_START_A>;
impl<'a, REG> TS1_START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No software trigger."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_START_A::B_0x0)
    }
    #[doc = "Software trigger for a frequency measurement. (only if TS1 is ready)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_START_A::B_0x1)
    }
}
#[doc = "Field `TS1_INTRIG_SEL` reader - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
pub type TS1_INTRIG_SEL_R = crate::FieldReader;
#[doc = "Field `TS1_INTRIG_SEL` writer - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
pub type TS1_INTRIG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS1_SMP_TIME` reader - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
pub type TS1_SMP_TIME_R = crate::FieldReader;
#[doc = "Field `TS1_SMP_TIME` writer - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
pub type TS1_SMP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCLK_SEL_A {
    #[doc = "0: High speed reference clock (PCLK)"]
    B_0x0 = 0,
    #[doc = "1: Low speed reference clock (LSE)"]
    B_0x1 = 1,
}
impl From<REFCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFCLK_SEL` reader - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
pub type REFCLK_SEL_R = crate::BitReader<REFCLK_SEL_A>;
impl REFCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFCLK_SEL_A {
        match self.bits {
            false => REFCLK_SEL_A::B_0x0,
            true => REFCLK_SEL_A::B_0x1,
        }
    }
    #[doc = "High speed reference clock (PCLK)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REFCLK_SEL_A::B_0x0
    }
    #[doc = "Low speed reference clock (LSE)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == REFCLK_SEL_A::B_0x1
    }
}
#[doc = "Field `REFCLK_SEL` writer - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
pub type REFCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG, REFCLK_SEL_A>;
impl<'a, REG> REFCLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed reference clock (PCLK)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLK_SEL_A::B_0x0)
    }
    #[doc = "Low speed reference clock (LSE)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REFCLK_SEL_A::B_0x1)
    }
}
#[doc = "Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Q_MEAS_OPT_A {
    #[doc = "0: Measurement with calibration"]
    B_0x0 = 0,
    #[doc = "1: Measurement without calibration"]
    B_0x1 = 1,
}
impl From<Q_MEAS_OPT_A> for bool {
    #[inline(always)]
    fn from(variant: Q_MEAS_OPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Q_MEAS_OPT` reader - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
pub type Q_MEAS_OPT_R = crate::BitReader<Q_MEAS_OPT_A>;
impl Q_MEAS_OPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Q_MEAS_OPT_A {
        match self.bits {
            false => Q_MEAS_OPT_A::B_0x0,
            true => Q_MEAS_OPT_A::B_0x1,
        }
    }
    #[doc = "Measurement with calibration"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == Q_MEAS_OPT_A::B_0x0
    }
    #[doc = "Measurement without calibration"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == Q_MEAS_OPT_A::B_0x1
    }
}
#[doc = "Field `Q_MEAS_OPT` writer - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
pub type Q_MEAS_OPT_W<'a, REG> = crate::BitWriter<'a, REG, Q_MEAS_OPT_A>;
impl<'a, REG> Q_MEAS_OPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measurement with calibration"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Q_MEAS_OPT_A::B_0x0)
    }
    #[doc = "Measurement without calibration"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Q_MEAS_OPT_A::B_0x1)
    }
}
#[doc = "High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSREF_CLK_DIV_A {
    #[doc = "0: No divider"]
    B_0x0 = 0,
    #[doc = "1: No divider"]
    B_0x1 = 1,
    #[doc = "2: 1/2 division ratio"]
    B_0x2 = 2,
    #[doc = "127: 1/127 division ratio"]
    B_0x7F = 127,
}
impl From<HSREF_CLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSREF_CLK_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSREF_CLK_DIV_A {
    type Ux = u8;
}
impl crate::IsEnum for HSREF_CLK_DIV_A {}
#[doc = "Field `HSREF_CLK_DIV` reader - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
pub type HSREF_CLK_DIV_R = crate::FieldReader<HSREF_CLK_DIV_A>;
impl HSREF_CLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HSREF_CLK_DIV_A> {
        match self.bits {
            0 => Some(HSREF_CLK_DIV_A::B_0x0),
            1 => Some(HSREF_CLK_DIV_A::B_0x1),
            2 => Some(HSREF_CLK_DIV_A::B_0x2),
            127 => Some(HSREF_CLK_DIV_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSREF_CLK_DIV_A::B_0x0
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSREF_CLK_DIV_A::B_0x1
    }
    #[doc = "1/2 division ratio"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == HSREF_CLK_DIV_A::B_0x2
    }
    #[doc = "1/127 division ratio"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == HSREF_CLK_DIV_A::B_0x7F
    }
}
#[doc = "Field `HSREF_CLK_DIV` writer - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
pub type HSREF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7, HSREF_CLK_DIV_A>;
impl<'a, REG> HSREF_CLK_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No divider"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSREF_CLK_DIV_A::B_0x0)
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSREF_CLK_DIV_A::B_0x1)
    }
    #[doc = "1/2 division ratio"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HSREF_CLK_DIV_A::B_0x2)
    }
    #[doc = "1/127 division ratio"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(HSREF_CLK_DIV_A::B_0x7F)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
    #[inline(always)]
    pub fn TS1_EN(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
    #[inline(always)]
    pub fn TS1_START(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
    #[inline(always)]
    pub fn TS1_INTRIG_SEL(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
    #[inline(always)]
    pub fn TS1_SMP_TIME(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
    #[inline(always)]
    pub fn REFCLK_SEL(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
    #[inline(always)]
    pub fn Q_MEAS_OPT(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
    #[inline(always)]
    pub fn HSREF_CLK_DIV(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready."]
    #[inline(always)]
    pub fn TS1_EN(&mut self) -> TS1_EN_W<'_, CFGR1_SPEC> {
        TS1_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software."]
    #[inline(always)]
    pub fn TS1_START(&mut self) -> TS1_START_W<'_, CFGR1_SPEC> {
        TS1_START_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input."]
    #[inline(always)]
    pub fn TS1_INTRIG_SEL(&mut self) -> TS1_INTRIG_SEL_W<'_, CFGR1_SPEC> {
        TS1_INTRIG_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE."]
    #[inline(always)]
    pub fn TS1_SMP_TIME(&mut self) -> TS1_SMP_TIME_W<'_, CFGR1_SPEC> {
        TS1_SMP_TIME_W::new(self, 16)
    }
    #[doc = "Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE)."]
    #[inline(always)]
    pub fn REFCLK_SEL(&mut self) -> REFCLK_SEL_W<'_, CFGR1_SPEC> {
        REFCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1)."]
    #[inline(always)]
    pub fn Q_MEAS_OPT(&mut self) -> Q_MEAS_OPT_W<'_, CFGR1_SPEC> {
        Q_MEAS_OPT_W::new(self, 21)
    }
    #[doc = "Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ..."]
    #[inline(always)]
    pub fn HSREF_CLK_DIV(&mut self) -> HSREF_CLK_DIV_W<'_, CFGR1_SPEC> {
        HSREF_CLK_DIV_W::new(self, 24)
    }
}
#[doc = "Temperature sensor configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {}
