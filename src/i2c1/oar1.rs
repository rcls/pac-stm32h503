#[doc = "Register `OAR1` reader"]
pub type R = crate::R<OAR1_SPEC>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<OAR1_SPEC>;
#[doc = "Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\] contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\] and OA1\\[0\\] are don't care. 10-bit addressing mode: OA1\\[9:0\\] contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\] contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\] and OA1\\[0\\] are don't care. 10-bit addressing mode: OA1\\[9:0\\] contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA1MODE_A {
    #[doc = "0: Own address 1 is a 7-bit address."]
    B_0x0 = 0,
    #[doc = "1: Own address 1 is a 10-bit address."]
    B_0x1 = 1,
}
impl From<OA1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_R = crate::BitReader<OA1MODE_A>;
impl OA1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA1MODE_A {
        match self.bits {
            false => OA1MODE_A::B_0x0,
            true => OA1MODE_A::B_0x1,
        }
    }
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OA1MODE_A::B_0x0
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OA1MODE_A::B_0x1
    }
}
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG, OA1MODE_A>;
impl<'a, REG> OA1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA1MODE_A::B_0x0)
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA1MODE_A::B_0x1)
    }
}
#[doc = "Own Address 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA1EN_A {
    #[doc = "0: Own address 1 disabled. The received slave address OA1 is NACKed."]
    B_0x0 = 0,
    #[doc = "1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    B_0x1 = 1,
}
impl From<OA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type OA1EN_R = crate::BitReader<OA1EN_A>;
impl OA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA1EN_A {
        match self.bits {
            false => OA1EN_A::B_0x0,
            true => OA1EN_A::B_0x1,
        }
    }
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OA1EN_A::B_0x0
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OA1EN_A::B_0x1
    }
}
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG, OA1EN_A>;
impl<'a, REG> OA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA1EN_A::B_0x0)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA1EN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\] contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\] and OA1\\[0\\] are don't care. 10-bit addressing mode: OA1\\[9:0\\] contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn OA1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn OA1MODE(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn OA1EN(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\] contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\] and OA1\\[0\\] are don't care. 10-bit addressing mode: OA1\\[9:0\\] contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn OA1(&mut self) -> OA1_W<'_, OAR1_SPEC> {
        OA1_W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn OA1MODE(&mut self) -> OA1MODE_W<'_, OAR1_SPEC> {
        OA1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn OA1EN(&mut self) -> OA1EN_W<'_, OAR1_SPEC> {
        OA1EN_W::new(self, 15)
    }
}
#[doc = "I2C own address 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for OAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {}
