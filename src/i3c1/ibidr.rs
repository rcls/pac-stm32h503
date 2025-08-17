#[doc = "Register `IBIDR` reader"]
pub type R = crate::R<IBIDR_SPEC>;
#[doc = "Register `IBIDR` writer"]
pub type W = crate::W<IBIDR_SPEC>;
#[doc = "Field `IBIDB0` reader - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\] mandatory data byte)."]
pub type IBIDB0_R = crate::FieldReader;
#[doc = "Field `IBIDB0` writer - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\] mandatory data byte)."]
pub type IBIDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IBIDB1` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
pub type IBIDB1_R = crate::FieldReader;
#[doc = "Field `IBIDB1` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
pub type IBIDB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IBIDB2` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
pub type IBIDB2_R = crate::FieldReader;
#[doc = "Field `IBIDB2` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
pub type IBIDB2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IBIDB3` reader - 8-bit IBI payload data (latest byte on I3C bus)."]
pub type IBIDB3_R = crate::FieldReader;
#[doc = "Field `IBIDB3` writer - 8-bit IBI payload data (latest byte on I3C bus)."]
pub type IBIDB3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\] mandatory data byte)."]
    #[inline(always)]
    pub fn IBIDB0(&self) -> IBIDB0_R {
        IBIDB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
    #[inline(always)]
    pub fn IBIDB1(&self) -> IBIDB1_R {
        IBIDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
    #[inline(always)]
    pub fn IBIDB2(&self) -> IBIDB2_R {
        IBIDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn IBIDB3(&self) -> IBIDB3_R {
        IBIDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\] mandatory data byte)."]
    #[inline(always)]
    pub fn IBIDB0(&mut self) -> IBIDB0_W<'_, IBIDR_SPEC> {
        IBIDB0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
    #[inline(always)]
    pub fn IBIDB1(&mut self) -> IBIDB1_W<'_, IBIDR_SPEC> {
        IBIDB1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
    #[inline(always)]
    pub fn IBIDB2(&mut self) -> IBIDB2_W<'_, IBIDR_SPEC> {
        IBIDB2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn IBIDB3(&mut self) -> IBIDB3_W<'_, IBIDR_SPEC> {
        IBIDB3_W::new(self, 24)
    }
}
#[doc = "I3C IBI payload data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBIDR_SPEC;
impl crate::RegisterSpec for IBIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibidr::R`](R) reader structure"]
impl crate::Readable for IBIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibidr::W`](W) writer structure"]
impl crate::Writable for IBIDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IBIDR to value 0"]
impl crate::Resettable for IBIDR_SPEC {}
