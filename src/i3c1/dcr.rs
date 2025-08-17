#[doc = "Register `DCR` reader"]
pub type R = crate::R<DCR_SPEC>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DCR_SPEC>;
#[doc = "device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCR_A {
    #[doc = "0: generic device (for v1.0 devices)"]
    B_0x0 = 0,
}
impl From<DCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DCR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCR_A {
    type Ux = u8;
}
impl crate::IsEnum for DCR_A {}
#[doc = "Field `DCR` reader - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_R = crate::FieldReader<DCR_A>;
impl DCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCR_A> {
        match self.bits {
            0 => Some(DCR_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "generic device (for v1.0 devices)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCR_A::B_0x0
    }
}
#[doc = "Field `DCR` writer - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DCR_A>;
impl<'a, REG> DCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generic device (for v1.0 devices)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DCR_A::B_0x0)
    }
}
impl R {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    pub fn DCR(&self) -> DCR_R {
        DCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    pub fn DCR(&mut self) -> DCR_W<'_, DCR_SPEC> {
        DCR_W::new(self, 0)
    }
}
#[doc = "I3C device characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {}
