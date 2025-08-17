#[doc = "Register `DBG_AUTH_HOST` reader"]
pub type R = crate::R<DBG_AUTH_HOST_SPEC>;
#[doc = "Register `DBG_AUTH_HOST` writer"]
pub type W = crate::W<DBG_AUTH_HOST_SPEC>;
#[doc = "Field `MESSAGE` reader - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MESSAGE_R = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE` writer - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MESSAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    pub fn MESSAGE(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    pub fn MESSAGE(&mut self) -> MESSAGE_W<'_, DBG_AUTH_HOST_SPEC> {
        MESSAGE_W::new(self, 0)
    }
}
#[doc = "DBGMCU debug authentication mailbox host register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_AUTH_HOST_SPEC;
impl crate::RegisterSpec for DBG_AUTH_HOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_auth_host::R`](R) reader structure"]
impl crate::Readable for DBG_AUTH_HOST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_auth_host::W`](W) writer structure"]
impl crate::Writable for DBG_AUTH_HOST_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBG_AUTH_HOST to value 0"]
impl crate::Resettable for DBG_AUTH_HOST_SPEC {}
