#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as a trigger output (tim_trgo)."]
    B_0x0 = 0,
    #[doc = "1: Enable - the Counter enable signal, tim_cnt_en, is used as a trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated when the CEN control bit is written."]
    B_0x1 = 1,
    #[doc = "2: Update - The update event is selected as a trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    B_0x2 = 2,
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
#[doc = "Field `MMS` reader - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<MMS_A>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MMS_A> {
        match self.bits {
            0 => Some(MMS_A::B_0x0),
            1 => Some(MMS_A::B_0x1),
            2 => Some(MMS_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as a trigger output (tim_trgo)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MMS_A::B_0x0
    }
    #[doc = "Enable - the Counter enable signal, tim_cnt_en, is used as a trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated when the CEN control bit is written."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MMS_A::B_0x1
    }
    #[doc = "Update - The update event is selected as a trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MMS_A::B_0x2
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS_A>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as a trigger output (tim_trgo)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x0)
    }
    #[doc = "Enable - the Counter enable signal, tim_cnt_en, is used as a trigger output (tim_trgo). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated when the CEN control bit is written."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x1)
    }
    #[doc = "Update - The update event is selected as a trigger output (tim_trgo). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn MMS(&mut self) -> MMS_W<'_, CR2_SPEC> {
        MMS_W::new(self, 4)
    }
}
#[doc = "TIM7 control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}
