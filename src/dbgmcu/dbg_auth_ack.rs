#[doc = "Register `DBG_AUTH_ACK` reader"]
pub type R = crate::R<DBG_AUTH_ACK_SPEC>;
#[doc = "Register `DBG_AUTH_ACK` writer"]
pub type W = crate::W<DBG_AUTH_ACK_SPEC>;
#[doc = "Field `HOST_ACK` reader - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_R = crate::BitReader;
#[doc = "Field `HOST_ACK` writer - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_ACK` reader - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_R = crate::BitReader;
#[doc = "Field `DEV_ACK` writer - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    pub fn HOST_ACK(&self) -> HOST_ACK_R {
        HOST_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    pub fn DEV_ACK(&self) -> DEV_ACK_R {
        DEV_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    pub fn HOST_ACK(&mut self) -> HOST_ACK_W<'_, DBG_AUTH_ACK_SPEC> {
        HOST_ACK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    pub fn DEV_ACK(&mut self) -> DEV_ACK_W<'_, DBG_AUTH_ACK_SPEC> {
        DEV_ACK_W::new(self, 1)
    }
}
#[doc = "DBGMCU debug authentication mailbox acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_AUTH_ACK_SPEC;
impl crate::RegisterSpec for DBG_AUTH_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_auth_ack::R`](R) reader structure"]
impl crate::Readable for DBG_AUTH_ACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_auth_ack::W`](W) writer structure"]
impl crate::Writable for DBG_AUTH_ACK_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBG_AUTH_ACK to value 0"]
impl crate::Resettable for DBG_AUTH_ACK_SPEC {}
