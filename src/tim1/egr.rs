#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGR_SPEC>;
#[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting)."]
    B_0x1 = 1,
}
impl From<UG_A> for bool {
    #[inline(always)]
    fn from(variant: UG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG_A>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UG_A::B_0x0)
    }
    #[doc = "Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UG_A::B_0x1)
    }
}
#[doc = "Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1G_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: A capture/compare event is generated on channel 1:"]
    B_0x1 = 1,
}
impl From<CC1G_A> for bool {
    #[inline(always)]
    fn from(variant: CC1G_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG, CC1G_A>;
impl<'a, REG> CC1G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G_A::B_0x0)
    }
    #[doc = "A capture/compare event is generated on channel 1:"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G_A::B_0x1)
    }
}
#[doc = "Field `CC2G` writer - Capture/compare 2 generation Refer to CC1G description"]
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation Refer to CC1G description"]
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation Refer to CC1G description"]
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMG_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits"]
    B_0x1 = 1,
}
impl From<COMG_A> for bool {
    #[inline(always)]
    fn from(variant: COMG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMG` writer - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMG_A>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMG_A::B_0x0)
    }
    #[doc = "When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMG_A::B_0x1)
    }
}
#[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TG_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    B_0x1 = 1,
}
impl From<TG_A> for bool {
    #[inline(always)]
    fn from(variant: TG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG, TG_A>;
impl<'a, REG> TG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TG_A::B_0x0)
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TG_A::B_0x1)
    }
}
#[doc = "Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    B_0x1 = 1,
}
impl From<BG_A> for bool {
    #[inline(always)]
    fn from(variant: BG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BG_A>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BG_A::B_0x0)
    }
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BG_A::B_0x1)
    }
}
#[doc = "Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2G_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    B_0x1 = 1,
}
impl From<B2G_A> for bool {
    #[inline(always)]
    fn from(variant: B2G_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG, B2G_A>;
impl<'a, REG> B2G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(B2G_A::B_0x0)
    }
    #[doc = "A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(B2G_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn UG(&mut self) -> UG_W<'_, EGR_SPEC> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub fn CC1G(&mut self) -> CC1G_W<'_, EGR_SPEC> {
        CC1G_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation Refer to CC1G description"]
    #[inline(always)]
    pub fn CC2G(&mut self) -> CC2G_W<'_, EGR_SPEC> {
        CC2G_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation Refer to CC1G description"]
    #[inline(always)]
    pub fn CC3G(&mut self) -> CC3G_W<'_, EGR_SPEC> {
        CC3G_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation Refer to CC1G description"]
    #[inline(always)]
    pub fn CC4G(&mut self) -> CC4G_W<'_, EGR_SPEC> {
        CC4G_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output."]
    #[inline(always)]
    pub fn COMG(&mut self) -> COMG_W<'_, EGR_SPEC> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn TG(&mut self) -> TG_W<'_, EGR_SPEC> {
        TG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn BG(&mut self) -> BG_W<'_, EGR_SPEC> {
        BG_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware."]
    #[inline(always)]
    pub fn B2G(&mut self) -> B2G_W<'_, EGR_SPEC> {
        B2G_W::new(self, 8)
    }
}
#[doc = "TIM1 event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {}
