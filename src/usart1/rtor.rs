#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RTOR_SPEC>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RTOR_SPEC>;
#[doc = "Field `RTO` reader - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
pub type RTO_R = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 - 0 information characters + LEC BLEN = 1 - 0 information characters + CRC BLEN = 255 - 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
pub type BLEN_R = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 - 0 information characters + LEC BLEN = 1 - 0 information characters + CRC BLEN = 255 - 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
    #[inline(always)]
    pub fn RTO(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 - 0 information characters + LEC BLEN = 1 - 0 information characters + CRC BLEN = 255 - 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
    #[inline(always)]
    pub fn BLEN(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
    #[inline(always)]
    pub fn RTO(&mut self) -> RTO_W<'_, RTOR_SPEC> {
        RTO_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 - 0 information characters + LEC BLEN = 1 - 0 information characters + CRC BLEN = 255 - 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
    #[inline(always)]
    pub fn BLEN(&mut self) -> BLEN_W<'_, RTOR_SPEC> {
        BLEN_W::new(self, 24)
    }
}
#[doc = "USART receiver timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {}
