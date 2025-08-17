#[doc = "Register `ECCNMIR` reader"]
pub type R = crate::R<ECCNMIR_SPEC>;
#[doc = "Register `ECCNMIR` writer"]
pub type W = crate::W<ECCNMIR_SPEC>;
#[doc = "NMI behavior setup when a double ECC error occurs on flitf data part\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCNMI_MASK_EN_A {
    #[doc = "0: NMI generated if a double ECC error in the flitf data part"]
    B_0x0 = 0,
    #[doc = "1: NMI not generated if a double ECC error in the flitf data part"]
    B_0x1 = 1,
}
impl From<ECCNMI_MASK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCNMI_MASK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCNMI_MASK_EN` reader - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_R = crate::BitReader<ECCNMI_MASK_EN_A>;
impl ECCNMI_MASK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCNMI_MASK_EN_A {
        match self.bits {
            false => ECCNMI_MASK_EN_A::B_0x0,
            true => ECCNMI_MASK_EN_A::B_0x1,
        }
    }
    #[doc = "NMI generated if a double ECC error in the flitf data part"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECCNMI_MASK_EN_A::B_0x0
    }
    #[doc = "NMI not generated if a double ECC error in the flitf data part"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECCNMI_MASK_EN_A::B_0x1
    }
}
#[doc = "Field `ECCNMI_MASK_EN` writer - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_W<'a, REG> = crate::BitWriter<'a, REG, ECCNMI_MASK_EN_A>;
impl<'a, REG> ECCNMI_MASK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NMI generated if a double ECC error in the flitf data part"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_MASK_EN_A::B_0x0)
    }
    #[doc = "NMI not generated if a double ECC error in the flitf data part"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_MASK_EN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    pub fn ECCNMI_MASK_EN(&self) -> ECCNMI_MASK_EN_R {
        ECCNMI_MASK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    pub fn ECCNMI_MASK_EN(&mut self) -> ECCNMI_MASK_EN_W<'_, ECCNMIR_SPEC> {
        ECCNMI_MASK_EN_W::new(self, 0)
    }
}
#[doc = "SBS flift ECC NMI mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccnmir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccnmir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCNMIR_SPEC;
impl crate::RegisterSpec for ECCNMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccnmir::R`](R) reader structure"]
impl crate::Readable for ECCNMIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccnmir::W`](W) writer structure"]
impl crate::Writable for ECCNMIR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCNMIR to value 0"]
impl crate::Resettable for ECCNMIR_SPEC {}
