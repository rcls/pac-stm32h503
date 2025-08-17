#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[2:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[2:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGA_A {
    #[doc = "0: Watchdog disabled"]
    B_0x0 = 0,
    #[doc = "1: Watchdog enabled"]
    B_0x1 = 1,
}
impl From<WDGA_A> for bool {
    #[inline(always)]
    fn from(variant: WDGA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
pub type WDGA_R = crate::BitReader<WDGA_A>;
impl WDGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDGA_A {
        match self.bits {
            false => WDGA_A::B_0x0,
            true => WDGA_A::B_0x1,
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDGA_A::B_0x0
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDGA_A::B_0x1
    }
}
#[doc = "Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG, WDGA_A>;
impl<'a, REG> WDGA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA_A::B_0x0)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[2:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn T(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn WDGA(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[2:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn T(&mut self) -> T_W<'_, CR_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn WDGA(&mut self) -> WDGA_W<'_, CR_SPEC> {
        WDGA_W::new(self, 7)
    }
}
#[doc = "WWDG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
