#[doc = "Register `TIMEOUTR` reader"]
pub type R = crate::R<TIMEOUTR_SPEC>;
#[doc = "Register `TIMEOUTR` writer"]
pub type W = crate::W<TIMEOUTR_SPEC>;
#[doc = "Field `TIMEOUTA` reader - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TIMEOUTA_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIDLE_A {
    #[doc = "0: TIMEOUTA is used to detect SCL low timeout"]
    B_0x0 = 0,
    #[doc = "1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    B_0x1 = 1,
}
impl From<TIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIDLE` reader - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
pub type TIDLE_R = crate::BitReader<TIDLE_A>;
impl TIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIDLE_A {
        match self.bits {
            false => TIDLE_A::B_0x0,
            true => TIDLE_A::B_0x1,
        }
    }
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIDLE_A::B_0x0
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIDLE_A::B_0x1
    }
}
#[doc = "Field `TIDLE` writer - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG, TIDLE_A>;
impl<'a, REG> TIDLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE_A::B_0x0)
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIDLE_A::B_0x1)
    }
}
#[doc = "Clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTEN_A {
    #[doc = "0: SCL timeout detection is disabled"]
    B_0x0 = 0,
    #[doc = "1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
    B_0x1 = 1,
}
impl From<TIMOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTEN` reader - Clock timeout enable"]
pub type TIMOUTEN_R = crate::BitReader<TIMOUTEN_A>;
impl TIMOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUTEN_A {
        match self.bits {
            false => TIMOUTEN_A::B_0x0,
            true => TIMOUTEN_A::B_0x1,
        }
    }
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIMOUTEN_A::B_0x0
    }
    #[doc = "SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIMOUTEN_A::B_0x1
    }
}
#[doc = "Field `TIMOUTEN` writer - Clock timeout enable"]
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUTEN_A>;
impl<'a, REG> TIMOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN_A::B_0x0)
    }
    #[doc = "SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTEN_A::B_0x1)
    }
}
#[doc = "Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TIMEOUTB_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Extended clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTEN_A {
    #[doc = "0: Extended clock timeout detection is disabled"]
    B_0x0 = 0,
    #[doc = "1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
    B_0x1 = 1,
}
impl From<TEXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTEN` reader - Extended clock timeout enable"]
pub type TEXTEN_R = crate::BitReader<TEXTEN_A>;
impl TEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEXTEN_A {
        match self.bits {
            false => TEXTEN_A::B_0x0,
            true => TEXTEN_A::B_0x1,
        }
    }
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEXTEN_A::B_0x0
    }
    #[doc = "Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEXTEN_A::B_0x1
    }
}
#[doc = "Field `TEXTEN` writer - Extended clock timeout enable"]
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG, TEXTEN_A>;
impl<'a, REG> TEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN_A::B_0x0)
    }
    #[doc = "Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn TIMEOUTA(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn TIDLE(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    pub fn TIMOUTEN(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    pub fn TIMEOUTB(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    pub fn TEXTEN(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn TIMEOUTA(&mut self) -> TIMEOUTA_W<'_, TIMEOUTR_SPEC> {
        TIMEOUTA_W::new(self, 0)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn TIDLE(&mut self) -> TIDLE_W<'_, TIMEOUTR_SPEC> {
        TIDLE_W::new(self, 12)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    pub fn TIMOUTEN(&mut self) -> TIMOUTEN_W<'_, TIMEOUTR_SPEC> {
        TIMOUTEN_W::new(self, 15)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    pub fn TIMEOUTB(&mut self) -> TIMEOUTB_W<'_, TIMEOUTR_SPEC> {
        TIMEOUTB_W::new(self, 16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    pub fn TEXTEN(&mut self) -> TEXTEN_W<'_, TIMEOUTR_SPEC> {
        TEXTEN_W::new(self, 31)
    }
}
#[doc = "I2C timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUTR_SPEC;
impl crate::RegisterSpec for TIMEOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeoutr::R`](R) reader structure"]
impl crate::Readable for TIMEOUTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeoutr::W`](W) writer structure"]
impl crate::Writable for TIMEOUTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMEOUTR to value 0"]
impl crate::Resettable for TIMEOUTR_SPEC {}
