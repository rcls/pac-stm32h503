#[doc = "Register `LPMCSR` reader"]
pub type R = crate::R<LPMCSR_SPEC>;
#[doc = "Register `LPMCSR` writer"]
pub type W = crate::W<LPMCSR_SPEC>;
#[doc = "Field `LPMEN` reader - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK_A {
    #[doc = "0: the valid LPM token is NYET."]
    B_0x0 = 0,
    #[doc = "1: the valid LPM token is ACK."]
    B_0x1 = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
pub type LPMACK_R = crate::BitReader<LPMACK_A>;
impl LPMACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::B_0x0,
            true => LPMACK_A::B_0x1,
        }
    }
    #[doc = "the valid LPM token is NYET."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPMACK_A::B_0x0
    }
    #[doc = "the valid LPM token is ACK."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPMACK_A::B_0x1
    }
}
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG, LPMACK_A>;
impl<'a, REG> LPMACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the valid LPM token is NYET."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK_A::B_0x0)
    }
    #[doc = "the valid LPM token is ACK."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK_A::B_0x1)
    }
}
#[doc = "Field `REMWAKE` reader - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token"]
pub type REMWAKE_R = crate::BitReader;
#[doc = "Field `BESL` reader - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token"]
pub type BESL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
    #[inline(always)]
    pub fn LPMEN(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
    #[inline(always)]
    pub fn LPMACK(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token"]
    #[inline(always)]
    pub fn REMWAKE(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token"]
    #[inline(always)]
    pub fn BESL(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled."]
    #[inline(always)]
    pub fn LPMEN(&mut self) -> LPMEN_W<'_, LPMCSR_SPEC> {
        LPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)"]
    #[inline(always)]
    pub fn LPMACK(&mut self) -> LPMACK_W<'_, LPMCSR_SPEC> {
        LPMACK_W::new(self, 1)
    }
}
#[doc = "USB_LPMCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCSR_SPEC;
impl crate::RegisterSpec for LPMCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmcsr::R`](R) reader structure"]
impl crate::Readable for LPMCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpmcsr::W`](W) writer structure"]
impl crate::Writable for LPMCSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSR_SPEC {}
