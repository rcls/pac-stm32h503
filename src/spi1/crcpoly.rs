#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CRCPOLY_SPEC>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CRCPOLY_SPEC>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial register This register contains the polynomial for the CRC calculation. The default 9-bit polynomial setting 0x107 corresponds to default 8-bit setting of DSIZE. It is compatible with setting 0x07 used at some other ST products with fixed length of the polynomial string where the most significant bit of the string is always kept hidden. Length of the polynomial is given by the most significant bit of the value stored at this register. It has to be set greater than DSIZE. CRC33_17 bit has to be set additionally with CRCPOLY register when DSIZE is configured to maximum 32-bit or 16-bit size and CRC is enabled (to keep polynomial length grater than data size). Note: CRCPOLY\\[31:16\\] bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type CRCPOLY_R = crate::FieldReader<u32>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial register This register contains the polynomial for the CRC calculation. The default 9-bit polynomial setting 0x107 corresponds to default 8-bit setting of DSIZE. It is compatible with setting 0x07 used at some other ST products with fixed length of the polynomial string where the most significant bit of the string is always kept hidden. Length of the polynomial is given by the most significant bit of the value stored at this register. It has to be set greater than DSIZE. CRC33_17 bit has to be set additionally with CRCPOLY register when DSIZE is configured to maximum 32-bit or 16-bit size and CRC is enabled (to keep polynomial length grater than data size). Note: CRCPOLY\\[31:16\\] bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC polynomial register This register contains the polynomial for the CRC calculation. The default 9-bit polynomial setting 0x107 corresponds to default 8-bit setting of DSIZE. It is compatible with setting 0x07 used at some other ST products with fixed length of the polynomial string where the most significant bit of the string is always kept hidden. Length of the polynomial is given by the most significant bit of the value stored at this register. It has to be set greater than DSIZE. CRC33_17 bit has to be set additionally with CRCPOLY register when DSIZE is configured to maximum 32-bit or 16-bit size and CRC is enabled (to keep polynomial length grater than data size). Note: CRCPOLY\\[31:16\\] bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn CRCPOLY(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial register This register contains the polynomial for the CRC calculation. The default 9-bit polynomial setting 0x107 corresponds to default 8-bit setting of DSIZE. It is compatible with setting 0x07 used at some other ST products with fixed length of the polynomial string where the most significant bit of the string is always kept hidden. Length of the polynomial is given by the most significant bit of the value stored at this register. It has to be set greater than DSIZE. CRC33_17 bit has to be set additionally with CRCPOLY register when DSIZE is configured to maximum 32-bit or 16-bit size and CRC is enabled (to keep polynomial length grater than data size). Note: CRCPOLY\\[31:16\\] bits are reserved at instances with data size limited to 16-bit. There is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn CRCPOLY(&mut self) -> CRCPOLY_W<'_, CRCPOLY_SPEC> {
        CRCPOLY_W::new(self, 0)
    }
}
#[doc = "SPI/I2S polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCPOLY_SPEC;
impl crate::RegisterSpec for CRCPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CRCPOLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CRCPOLY_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCPOLY to value 0x0107"]
impl crate::Resettable for CRCPOLY_SPEC {
    const RESET_VALUE: u32 = 0x0107;
}
