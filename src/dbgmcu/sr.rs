#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `AP_PRESENT` writer - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
pub type AP_PRESENT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AP_ENABLED` writer - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
pub type AP_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
    #[inline(always)]
    pub fn AP_PRESENT(&mut self) -> AP_PRESENT_W<'_, SR_SPEC> {
        AP_PRESENT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
    #[inline(always)]
    pub fn AP_ENABLED(&mut self) -> AP_ENABLED_W<'_, SR_SPEC> {
        AP_ENABLED_W::new(self, 16)
    }
}
#[doc = "DBGMCU status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SR to value 0x0001_0003"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x0001_0003;
}
