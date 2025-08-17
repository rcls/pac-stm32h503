#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Polynomial size These bits control the size of the polynomial.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    #[doc = "0: 32 bit polynomial"]
    B_0x0 = 0,
    #[doc = "1: 16 bit polynomial"]
    B_0x1 = 1,
    #[doc = "2: 8 bit polynomial"]
    B_0x2 = 2,
    #[doc = "3: 7 bit polynomial"]
    B_0x3 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLYSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for POLYSIZE_A {}
#[doc = "Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial."]
pub type POLYSIZE_R = crate::FieldReader<POLYSIZE_A>;
impl POLYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::B_0x0,
            1 => POLYSIZE_A::B_0x1,
            2 => POLYSIZE_A::B_0x2,
            3 => POLYSIZE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == POLYSIZE_A::B_0x0
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == POLYSIZE_A::B_0x1
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == POLYSIZE_A::B_0x2
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == POLYSIZE_A::B_0x3
    }
}
#[doc = "Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial."]
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, POLYSIZE_A, crate::Safe>;
impl<'a, REG> POLYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0x0)
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0x1)
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0x2)
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0x3)
    }
}
#[doc = "Reverse input data These bits control the reversal of the bit order of the input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN_A {
    #[doc = "0: Bit order not affected"]
    B_0x0 = 0,
    #[doc = "1: Bit reversal done by byte"]
    B_0x1 = 1,
    #[doc = "2: Bit reversal done by half-word"]
    B_0x2 = 2,
    #[doc = "3: Bit reversal done by word"]
    B_0x3 = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_IN_A {
    type Ux = u8;
}
impl crate::IsEnum for REV_IN_A {}
#[doc = "Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data"]
pub type REV_IN_R = crate::FieldReader<REV_IN_A>;
impl REV_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::B_0x0,
            1 => REV_IN_A::B_0x1,
            2 => REV_IN_A::B_0x2,
            3 => REV_IN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REV_IN_A::B_0x0
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == REV_IN_A::B_0x1
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == REV_IN_A::B_0x2
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == REV_IN_A::B_0x3
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data"]
pub type REV_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REV_IN_A, crate::Safe>;
impl<'a, REG> REV_IN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0x0)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0x1)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0x2)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0x3)
    }
}
#[doc = "Reverse output data This bit controls the reversal of the bit order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT_A {
    #[doc = "0: Bit order not affected"]
    B_0x0 = 0,
    #[doc = "1: Bit-reversed output format"]
    B_0x1 = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data."]
pub type REV_OUT_R = crate::BitReader<REV_OUT_A>;
impl REV_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::B_0x0,
            true => REV_OUT_A::B_0x1,
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REV_OUT_A::B_0x0
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == REV_OUT_A::B_0x1
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data."]
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG, REV_OUT_A>;
impl<'a, REG> REV_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT_A::B_0x0)
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    pub fn RESET(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn POLYSIZE(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn REV_IN(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn REV_OUT(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    pub fn RESET(&mut self) -> RESET_W<'_, CR_SPEC> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn POLYSIZE(&mut self) -> POLYSIZE_W<'_, CR_SPEC> {
        POLYSIZE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn REV_IN(&mut self) -> REV_IN_W<'_, CR_SPEC> {
        REV_IN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn REV_OUT(&mut self) -> REV_OUT_W<'_, CR_SPEC> {
        REV_OUT_W::new(self, 7)
    }
}
#[doc = "CRC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
