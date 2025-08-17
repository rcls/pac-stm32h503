#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECR_SPEC>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECR_SPEC>;
#[doc = "Index enable This bit indicates if the Index event resets the counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE_A {
    #[doc = "0: Index disabled"]
    B_0x0 = 0,
    #[doc = "1: Index enabled"]
    B_0x1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_R = crate::BitReader<IE_A>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::B_0x0,
            true => IE_A::B_0x1,
        }
    }
    #[doc = "Index disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IE_A::B_0x0
    }
    #[doc = "Index enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IE_A::B_0x1
    }
}
#[doc = "Field `IE` writer - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG, IE_A>;
impl<'a, REG> IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IE_A::B_0x0)
    }
    #[doc = "Index enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IE_A::B_0x1)
    }
}
#[doc = "Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\] bitfield must be written when IE bit is reset (index disabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDIR_A {
    #[doc = "0: Index resets the counter whatever the direction"]
    B_0x0 = 0,
    #[doc = "1: Index resets the counter when up-counting only"]
    B_0x1 = 1,
    #[doc = "2: Index resets the counter when down-counting only"]
    B_0x2 = 2,
}
impl From<IDIR_A> for u8 {
    #[inline(always)]
    fn from(variant: IDIR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDIR_A {
    type Ux = u8;
}
impl crate::IsEnum for IDIR_A {}
#[doc = "Field `IDIR` reader - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\] bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_R = crate::FieldReader<IDIR_A>;
impl IDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDIR_A> {
        match self.bits {
            0 => Some(IDIR_A::B_0x0),
            1 => Some(IDIR_A::B_0x1),
            2 => Some(IDIR_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "Index resets the counter whatever the direction"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IDIR_A::B_0x0
    }
    #[doc = "Index resets the counter when up-counting only"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IDIR_A::B_0x1
    }
    #[doc = "Index resets the counter when down-counting only"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IDIR_A::B_0x2
    }
}
#[doc = "Field `IDIR` writer - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\] bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IDIR_A>;
impl<'a, REG> IDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Index resets the counter whatever the direction"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDIR_A::B_0x0)
    }
    #[doc = "Index resets the counter when up-counting only"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDIR_A::B_0x1)
    }
    #[doc = "Index resets the counter when down-counting only"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IDIR_A::B_0x2)
    }
}
#[doc = "Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBLK_A {
    #[doc = "0: Index always active"]
    B_0x0 = 0,
    #[doc = "1: Index disabled hen tim_ti3 input is active, as per CC3P bitfield"]
    B_0x1 = 1,
    #[doc = "2: Index disabled when tim_ti4 input is active, as per CC4P bitfield"]
    B_0x2 = 2,
}
impl From<IBLK_A> for u8 {
    #[inline(always)]
    fn from(variant: IBLK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IBLK_A {
    type Ux = u8;
}
impl crate::IsEnum for IBLK_A {}
#[doc = "Field `IBLK` reader - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_R = crate::FieldReader<IBLK_A>;
impl IBLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IBLK_A> {
        match self.bits {
            0 => Some(IBLK_A::B_0x0),
            1 => Some(IBLK_A::B_0x1),
            2 => Some(IBLK_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "Index always active"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBLK_A::B_0x0
    }
    #[doc = "Index disabled hen tim_ti3 input is active, as per CC3P bitfield"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBLK_A::B_0x1
    }
    #[doc = "Index disabled when tim_ti4 input is active, as per CC4P bitfield"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IBLK_A::B_0x2
    }
}
#[doc = "Field `IBLK` writer - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IBLK_A>;
impl<'a, REG> IBLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Index always active"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IBLK_A::B_0x0)
    }
    #[doc = "Index disabled hen tim_ti3 input is active, as per CC3P bitfield"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IBLK_A::B_0x1)
    }
    #[doc = "Index disabled when tim_ti4 input is active, as per CC4P bitfield"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IBLK_A::B_0x2)
    }
}
#[doc = "First index This bit indicates if the first index only is taken into account\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIDX_A {
    #[doc = "0: Index is always active"]
    B_0x0 = 0,
    #[doc = "1: the first Index only resets the counter"]
    B_0x1 = 1,
}
impl From<FIDX_A> for bool {
    #[inline(always)]
    fn from(variant: FIDX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIDX` reader - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_R = crate::BitReader<FIDX_A>;
impl FIDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIDX_A {
        match self.bits {
            false => FIDX_A::B_0x0,
            true => FIDX_A::B_0x1,
        }
    }
    #[doc = "Index is always active"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FIDX_A::B_0x0
    }
    #[doc = "the first Index only resets the counter"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FIDX_A::B_0x1
    }
}
#[doc = "Field `FIDX` writer - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_W<'a, REG> = crate::BitWriter<'a, REG, FIDX_A>;
impl<'a, REG> FIDX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index is always active"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FIDX_A::B_0x0)
    }
    #[doc = "the first Index only resets the counter"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FIDX_A::B_0x1)
    }
}
#[doc = "Index positioning In quadrature encoder mode (SMS\\[3:0\\] = 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\] = 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\] bit is not significant\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPOS_A {
    #[doc = "0: Index resets the counter when AB = 00"]
    B_0x0 = 0,
    #[doc = "1: Index resets the counter when AB = 01"]
    B_0x1 = 1,
    #[doc = "2: Index resets the counter when AB = 10"]
    B_0x2 = 2,
    #[doc = "3: Index resets the counter when AB = 11"]
    B_0x3 = 3,
}
impl From<IPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: IPOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IPOS_A {
    type Ux = u8;
}
impl crate::IsEnum for IPOS_A {}
#[doc = "Field `IPOS` reader - Index positioning In quadrature encoder mode (SMS\\[3:0\\] = 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\] = 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\] bit is not significant"]
pub type IPOS_R = crate::FieldReader<IPOS_A>;
impl IPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPOS_A {
        match self.bits {
            0 => IPOS_A::B_0x0,
            1 => IPOS_A::B_0x1,
            2 => IPOS_A::B_0x2,
            3 => IPOS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Index resets the counter when AB = 00"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IPOS_A::B_0x0
    }
    #[doc = "Index resets the counter when AB = 01"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IPOS_A::B_0x1
    }
    #[doc = "Index resets the counter when AB = 10"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IPOS_A::B_0x2
    }
    #[doc = "Index resets the counter when AB = 11"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IPOS_A::B_0x3
    }
}
#[doc = "Field `IPOS` writer - Index positioning In quadrature encoder mode (SMS\\[3:0\\] = 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\] = 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\] bit is not significant"]
pub type IPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IPOS_A, crate::Safe>;
impl<'a, REG> IPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Index resets the counter when AB = 00"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IPOS_A::B_0x0)
    }
    #[doc = "Index resets the counter when AB = 01"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IPOS_A::B_0x1)
    }
    #[doc = "Index resets the counter when AB = 10"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IPOS_A::B_0x2)
    }
    #[doc = "Index resets the counter when AB = 11"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IPOS_A::B_0x3)
    }
}
#[doc = "Field `PW` reader - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\] x tPWG"]
pub type PW_R = crate::FieldReader;
#[doc = "Field `PW` writer - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\] x tPWG"]
pub type PW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWPRSC` reader - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_R = crate::FieldReader;
#[doc = "Field `PWPRSC` writer - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    pub fn IE(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\] bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    pub fn IDIR(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    pub fn IBLK(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    pub fn FIDX(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\] = 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\] = 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\] bit is not significant"]
    #[inline(always)]
    pub fn IPOS(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\] x tPWG"]
    #[inline(always)]
    pub fn PW(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    pub fn PWPRSC(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    pub fn IE(&mut self) -> IE_W<'_, ECR_SPEC> {
        IE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\] bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    pub fn IDIR(&mut self) -> IDIR_W<'_, ECR_SPEC> {
        IDIR_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    pub fn IBLK(&mut self) -> IBLK_W<'_, ECR_SPEC> {
        IBLK_W::new(self, 3)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    pub fn FIDX(&mut self) -> FIDX_W<'_, ECR_SPEC> {
        FIDX_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\] = 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\] = 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\] bit is not significant"]
    #[inline(always)]
    pub fn IPOS(&mut self) -> IPOS_W<'_, ECR_SPEC> {
        IPOS_W::new(self, 6)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\] x tPWG"]
    #[inline(always)]
    pub fn PW(&mut self) -> PW_W<'_, ECR_SPEC> {
        PW_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    pub fn PWPRSC(&mut self) -> PWPRSC_W<'_, ECR_SPEC> {
        PWPRSC_W::new(self, 24)
    }
}
#[doc = "TIM3 timer encoder control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {}
