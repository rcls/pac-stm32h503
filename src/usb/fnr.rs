#[doc = "Register `FNR` reader"]
pub type R = crate::R<FNR_SPEC>;
#[doc = "Field `FN` reader - Frame number This bit field contains the 11-bits frame number contained in the last received SOF packet. The frame number is incremented for every frame sent by the host and it is useful for isochronous transfers. This bit field is updated on the generation of an SOF interrupt."]
pub type FN_R = crate::FieldReader<u16>;
#[doc = "Field `LSOF` reader - Lost SOF Device mode These bits are written by the hardware when an ESOF interrupt is generated, counting the number of consecutive SOF packets lost. At the reception of an SOF packet, these bits are cleared."]
pub type LSOF_R = crate::FieldReader;
#[doc = "Field `LCK` reader - Locked Device mode This bit is set by the hardware when at least two consecutive SOF packets have been received after the end of an USB reset condition or after the end of an USB resume sequence. Once locked, the frame timer remains in this state until an USB reset or USB suspend event occurs."]
pub type LCK_R = crate::BitReader;
#[doc = "Field `RXDM` reader - Receive data - line status This bit can be used to observe the status of received data minus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
pub type RXDM_R = crate::BitReader;
#[doc = "Field `RXDP` reader - Receive data + line status This bit can be used to observe the status of received data plus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
pub type RXDP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Frame number This bit field contains the 11-bits frame number contained in the last received SOF packet. The frame number is incremented for every frame sent by the host and it is useful for isochronous transfers. This bit field is updated on the generation of an SOF interrupt."]
    #[inline(always)]
    pub fn FN(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost SOF Device mode These bits are written by the hardware when an ESOF interrupt is generated, counting the number of consecutive SOF packets lost. At the reception of an SOF packet, these bits are cleared."]
    #[inline(always)]
    pub fn LSOF(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Locked Device mode This bit is set by the hardware when at least two consecutive SOF packets have been received after the end of an USB reset condition or after the end of an USB resume sequence. Once locked, the frame timer remains in this state until an USB reset or USB suspend event occurs."]
    #[inline(always)]
    pub fn LCK(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status This bit can be used to observe the status of received data minus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
    #[inline(always)]
    pub fn RXDM(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status This bit can be used to observe the status of received data plus upstream port data line. It can be used during end-of-suspend routines to help determining the wakeup event."]
    #[inline(always)]
    pub fn RXDP(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "USB frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FNR_SPEC;
impl crate::RegisterSpec for FNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnr::R`](R) reader structure"]
impl crate::Readable for FNR_SPEC {}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FNR_SPEC {}
