#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "DAC channel1 ready status bit This bit is set and cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1RDY_A {
    #[doc = "0: DAC channel1 is not yet ready to accept the trigger nor output data"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 is ready to accept the trigger or output data"]
    B_0x1 = 1,
}
impl From<DAC1RDY_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1RDY` reader - DAC channel1 ready status bit This bit is set and cleared by hardware."]
pub type DAC1RDY_R = crate::BitReader<DAC1RDY_A>;
impl DAC1RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1RDY_A {
        match self.bits {
            false => DAC1RDY_A::B_0x0,
            true => DAC1RDY_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 is not yet ready to accept the trigger nor output data"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC1RDY_A::B_0x0
    }
    #[doc = "DAC channel1 is ready to accept the trigger or output data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC1RDY_A::B_0x1
    }
}
#[doc = "DAC channel1 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORSTAT1_A {
    #[doc = "0: DOR\\[11:0\\] is used actual DAC output"]
    B_0x0 = 0,
    #[doc = "1: DORB\\[11:0\\] is used actual DAC output"]
    B_0x1 = 1,
}
impl From<DORSTAT1_A> for bool {
    #[inline(always)]
    fn from(variant: DORSTAT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORSTAT1` reader - DAC channel1 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode."]
pub type DORSTAT1_R = crate::BitReader<DORSTAT1_A>;
impl DORSTAT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DORSTAT1_A {
        match self.bits {
            false => DORSTAT1_A::B_0x0,
            true => DORSTAT1_A::B_0x1,
        }
    }
    #[doc = "DOR\\[11:0\\] is used actual DAC output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DORSTAT1_A::B_0x0
    }
    #[doc = "DORB\\[11:0\\] is used actual DAC output"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DORSTAT1_A::B_0x1
    }
}
#[doc = "DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel1"]
    B_0x0 = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    B_0x1 = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_R = crate::BitReader<DMAUDR1_A>;
impl DMAUDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::B_0x0,
            true => DMAUDR1_A::B_0x1,
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAUDR1_A::B_0x0
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAUDR1_A::B_0x1
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR1_A>;
impl<'a, REG> DMAUDR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1_A::B_0x0)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1_A::B_0x1)
    }
}
#[doc = "DAC channel1 calibration offset status This bit is set and cleared by hardware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_FLAG1_A {
    #[doc = "0: calibration trimming value is lower than the offset correction value"]
    B_0x0 = 0,
    #[doc = "1: calibration trimming value is equal or greater than the offset correction value"]
    B_0x1 = 1,
}
impl From<CAL_FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_FLAG1` reader - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
pub type CAL_FLAG1_R = crate::BitReader<CAL_FLAG1_A>;
impl CAL_FLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_FLAG1_A {
        match self.bits {
            false => CAL_FLAG1_A::B_0x0,
            true => CAL_FLAG1_A::B_0x1,
        }
    }
    #[doc = "calibration trimming value is lower than the offset correction value"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CAL_FLAG1_A::B_0x0
    }
    #[doc = "calibration trimming value is equal or greater than the offset correction value"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CAL_FLAG1_A::B_0x1
    }
}
#[doc = "DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWST1_A {
    #[doc = "0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    B_0x0 = 0,
    #[doc = "1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    B_0x1 = 1,
}
impl From<BWST1_A> for bool {
    #[inline(always)]
    fn from(variant: BWST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWST1` reader - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization)."]
pub type BWST1_R = crate::BitReader<BWST1_A>;
impl BWST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWST1_A {
        match self.bits {
            false => BWST1_A::B_0x0,
            true => BWST1_A::B_0x1,
        }
    }
    #[doc = "There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BWST1_A::B_0x0
    }
    #[doc = "There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BWST1_A::B_0x1
    }
}
#[doc = "DAC channel2 ready status bit This bit is set and cleared by hardware. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC2RDY_A {
    #[doc = "0: DAC channel2 is not yet ready to accept the trigger nor output data"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 is ready to accept the trigger or output data"]
    B_0x1 = 1,
}
impl From<DAC2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2RDY` reader - DAC channel2 ready status bit This bit is set and cleared by hardware. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DAC2RDY_R = crate::BitReader<DAC2RDY_A>;
impl DAC2RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC2RDY_A {
        match self.bits {
            false => DAC2RDY_A::B_0x0,
            true => DAC2RDY_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 is not yet ready to accept the trigger nor output data"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC2RDY_A::B_0x0
    }
    #[doc = "DAC channel2 is ready to accept the trigger or output data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC2RDY_A::B_0x1
    }
}
#[doc = "DAC channel2 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORSTAT2_A {
    #[doc = "0: DOR\\[11:0\\] is used actual DAC output"]
    B_0x0 = 0,
    #[doc = "1: DORB\\[11:0\\] is used actual DAC output"]
    B_0x1 = 1,
}
impl From<DORSTAT2_A> for bool {
    #[inline(always)]
    fn from(variant: DORSTAT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORSTAT2` reader - DAC channel2 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DORSTAT2_R = crate::BitReader<DORSTAT2_A>;
impl DORSTAT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DORSTAT2_A {
        match self.bits {
            false => DORSTAT2_A::B_0x0,
            true => DORSTAT2_A::B_0x1,
        }
    }
    #[doc = "DOR\\[11:0\\] is used actual DAC output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DORSTAT2_A::B_0x0
    }
    #[doc = "DORB\\[11:0\\] is used actual DAC output"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DORSTAT2_A::B_0x1
    }
}
#[doc = "DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR2_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel2"]
    B_0x0 = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)."]
    B_0x1 = 1,
}
impl From<DMAUDR2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDR2_R = crate::BitReader<DMAUDR2_A>;
impl DMAUDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR2_A {
        match self.bits {
            false => DMAUDR2_A::B_0x0,
            true => DMAUDR2_A::B_0x1,
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAUDR2_A::B_0x0
    }
    #[doc = "DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAUDR2_A::B_0x1
    }
}
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR2_A>;
impl<'a, REG> DMAUDR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR2_A::B_0x0)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR2_A::B_0x1)
    }
}
#[doc = "DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_FLAG2_A {
    #[doc = "0: calibration trimming value is lower than the offset correction value"]
    B_0x0 = 0,
    #[doc = "1: calibration trimming value is equal or greater than the offset correction value"]
    B_0x1 = 1,
}
impl From<CAL_FLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_FLAG2` reader - DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type CAL_FLAG2_R = crate::BitReader<CAL_FLAG2_A>;
impl CAL_FLAG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_FLAG2_A {
        match self.bits {
            false => CAL_FLAG2_A::B_0x0,
            true => CAL_FLAG2_A::B_0x1,
        }
    }
    #[doc = "calibration trimming value is lower than the offset correction value"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CAL_FLAG2_A::B_0x0
    }
    #[doc = "calibration trimming value is equal or greater than the offset correction value"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CAL_FLAG2_A::B_0x1
    }
}
#[doc = "DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWST2_A {
    #[doc = "0: There is no write operation of DAC_SHSR2 ongoing: DAC_SHSR2 can be written"]
    B_0x0 = 0,
    #[doc = "1: There is a write operation of DAC_SHSR2 ongoing: DAC_SHSR2 cannot be written"]
    B_0x1 = 1,
}
impl From<BWST2_A> for bool {
    #[inline(always)]
    fn from(variant: BWST2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWST2` reader - DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type BWST2_R = crate::BitReader<BWST2_A>;
impl BWST2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWST2_A {
        match self.bits {
            false => BWST2_A::B_0x0,
            true => BWST2_A::B_0x1,
        }
    }
    #[doc = "There is no write operation of DAC_SHSR2 ongoing: DAC_SHSR2 can be written"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BWST2_A::B_0x0
    }
    #[doc = "There is a write operation of DAC_SHSR2 ongoing: DAC_SHSR2 cannot be written"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BWST2_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 11 - DAC channel1 ready status bit This bit is set and cleared by hardware."]
    #[inline(always)]
    pub fn DAC1RDY(&self) -> DAC1RDY_R {
        DAC1RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn DORSTAT1(&self) -> DORSTAT1_R {
        DORSTAT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn DMAUDR1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn CAL_FLAG1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization)."]
    #[inline(always)]
    pub fn BWST1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - DAC channel2 ready status bit This bit is set and cleared by hardware. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DAC2RDY(&self) -> DAC2RDY_R {
        DAC2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DAC channel2 output register status bit This bit is set and cleared by hardware. It is applicable only when the DAC operates in Double data mode. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DORSTAT2(&self) -> DORSTAT2_R {
        DORSTAT2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAUDR2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn CAL_FLAG2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn BWST2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn DMAUDR1(&mut self) -> DMAUDR1_W<'_, SR_SPEC> {
        DMAUDR1_W::new(self, 13)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAUDR2(&mut self) -> DMAUDR2_W<'_, SR_SPEC> {
        DMAUDR2_W::new(self, 29)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
