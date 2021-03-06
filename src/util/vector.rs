/*
 * Copyright 2004 Kenta Cho. Some rights reserved.
 */

use std::ops::{Mul, MulAssign, Add, AddAssign};

#[allow(dead_code)]
static mut RSL : Vector = Vector{x : 0.0, y : 0.0};

/**
 * 2D vector.
 */
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
 pub x : f32,
 pub y : f32,

 //private:
 // static Vector rsl;
}

impl Add for Vector {
  type Output = Vector;

  fn add(self, v : Vector) -> Vector {
    Vector{x : self.x + v.x, y : self.y + v.y}
  }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
  type Output = Vector;

  fn add(self, v : &'b Vector) -> Vector {
    Vector{x : self.x + v.x, y : self.y + v.y}
  }
}

impl Mul for Vector {
  type Output = f32;

  fn mul(self, v : Vector) -> f32 {
   self.x * v.x + self.y * v.y
  }
}

impl<'a, 'b> Mul<&'b Vector> for &'a Vector {
  type Output = f32;

  fn mul(self, v : &'b Vector) -> f32 {
    self.x * v.x + self.y * v.y
  }
}

impl Mul<f32> for Vector {
  type Output = Vector;

  fn mul(self, f: f32) -> Vector {
    Vector {
      x: self.x * f,
      y: self.y * f,
    }
  }
}

impl MulAssign<f32> for Vector {
  //type Output = Vector;

  fn mul_assign(&mut self, f: f32) {
    self.x *= f;
    self.y *= f;
  }
}

impl AddAssign<Vector> for Vector {
  //type Output = Vector;

  fn add_assign(&mut self, v : Vector) {
    self.x += v.x;
    self.y += v.y;
  }
}


impl Vector {
/*
  public static this() {
    rsl = new Vector;
  }

  public this() {
    clear();
  }

  public this(float x, float y) {
    this.x = x;
    this.y = y;
  }
  */
  pub fn new(x : f32, y : f32) -> Vector {
    Vector {x: x, y: y,}
  }

  pub fn clear(&mut self) {
    self.x = 0.0;
    self.y = 0.0;
  }

  pub fn opMul(&mut self, v : Vector) -> f32 {
    self.x * v.x + self.y * v.y
  }

  pub fn getElement(&self, v : &Vector) -> Vector {
    let ll : f32 = v * v;
    if ll > 0.1 {
      let mag : f32 = self * v;
      RSL.x = mag * self.x / ll;
      RSL.y = mag * self.y / ll;
    } else {
      RSL.x = 0.0;
      RSL.y = 0.0;
    }
    RSL
  }

  // was getElement
  pub fn getElementMinMax(&self, v : &Vector, min : f32, max : f32) -> Vector {
    let ll = v * v;
    if ll > 0.1 {
      let mag : f32 = (self * v) / ll;
      RSL.x = mag * self.x;
      RSL.y = mag * self.y;
    } else {
      RSL.x = 0.0;
      RSL.y = 0.0;
    }
    let d = RSL.vctSize();
    if d > 0.1 && d < min {
      RSL *= min / d;
    } else if d > max {
      RSL *= max / d;
    }
    RSL
  }

  pub fn opAddAssign(&mut self, v : Vector) {
    self.x += v.x;
    self.y += v.y;
  }

  pub fn opSubAssign(&mut self, v : Vector) {
    self.x -= v.x;
    self.y -= v.y;
  }

  pub fn opMulAssign(&mut self, a : f32) {
    self.x *= a;
    self.y *= a;
  }

  pub fn opDivAssign(&mut self, a : f32) {
    self.x /= a;
    self.y /= a;
  }

  pub fn checkSide(&self, pos1 : Vector, pos2 : Vector) -> f32 {
   let xo = pos2.x - pos1.x;
   let yo = pos2.y - pos1.y;
    if xo == 0.0 {
      if yo == 0.0 {
        return 0.0;
      }
      if yo > 0.0 {
        return self.x - pos1.x;
      } else {
        return pos1.x - self.x;
      }
    } else if yo == 0.0 {
      if xo > 0.0 {
        return pos1.y - self.y;
      } else {
        return self.y - pos1.y;
      }
    } else {
      if (xo * yo) > 0.0 {
        return (self.x - pos1.x) / xo - (self.y - pos1.y) / yo;
      } else {
        return -(self.x - pos1.x) / xo + (self.y - pos1.y) / yo;
      }
    }
  }

  // was checkSide
  pub fn checkSideOffset(&self, pos1 : Vector, pos2 : Vector, ofs : Vector) -> f32 {
    let xo = pos2.x - pos1.x;
    let yo = pos2.y - pos1.y;
    let mx = self.x + ofs.x;
    let my = self.y + ofs.y;
    if xo == 0.0 {
      if yo == 0.0 {
        return 0.0;
      }
      if yo > 0.0 {
        return mx - pos1.x;
      } else {
        return pos1.x - mx;
      }
    } else if yo == 0.0 {
      if xo > 0.0 {
        return pos1.y - my;
      } else {
        return my - pos1.y;
      }
    } else {
      if (xo * yo) > 0.0 {
        return (mx - pos1.x) / xo - (my - pos1.y) / yo;
      } else {
        return -(mx - pos1.x) / xo + (my - pos1.y) / yo;
      }
    }
  }

  pub fn checkCross(&self, p : Vector, p1 : Vector, p2 : Vector, width : f32) -> bool {
    let a1x : f32;
    let a1y : f32;
    let a2x : f32;
    let a2y : f32;

    if self.x < p.x {
      a1x = self.x - width;
      a2x = p.x + width;
    } else {
      a1x = p.x - width;
      a2x = self.x + width;
    }

    if self.y < p.y {
      a1y = self.y - width;
      a2y = p.y + width;
    } else {
      a1y = p.y - width;
      a2y = self.y + width;
    }

    let b1x : f32;
    let b1y : f32;
    let b2x : f32;
    let b2y : f32;

    if p2.y < p1.y {
      b1y = p2.y - width;
      b2y = p1.y + width;
    } else {
      b1y = p1.y - width;
      b2y = p2.y + width;
    }

    if (a2y >= b1y) && (b2y >= a1y) {
      if p2.x < p1.x {
        b1x = p2.x - width;
        b2x = p1.x + width;
      } else {
        b1x = p1.x - width;
        b2x = p2.x + width;
      }

      if (a2x >= b1x) && (b2x >= a1x) {
        let a = self.y - p.y;
        let b = p.x - self.x;
        let c = p.x * self.y - p.y * self.x;
        let d = p2.y - p1.y;
        let e = p1.x - p2.x;
        let f = p1.x * p2.y - p1.y * p2.x;
        let dnm = b * d - a * e;
        if dnm != 0.0 {
          let x : f32 = (b * f - c * e) / dnm;
          let y : f32 = (c * d - a * f) / dnm;
          if a1x <= x && x <= a2x && a1y <= y && y <= a2y &&
              b1x <= x && x <= b2x && b1y <= y && y <= b2y {
            return true;
          }
        }
      }
    }
    false
  }

  pub fn checkHitDist(&self, p : Vector, pp : Vector, dist : f32) -> bool {
    let bmvx = pp.x - p.x;
    let bmvy = pp.y - p.y;
    let inaa = bmvx * bmvx + bmvy * bmvy;
    if inaa > 0.00001 {
      let sofsx = self.x - p.x;
      let sofsy = self.y - p.y;
      let inab = bmvx * sofsx + bmvy * sofsy;
      if inab >= 0.0 && inab <= inaa {
        let hd = sofsx * sofsx + sofsy * sofsy - inab * inab / inaa;
        if hd >= 0.0 && hd <= dist {
          return true;
        }
      }
    }
    false
  }

  pub fn vctSize(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  // was dist()
  pub fn dist2(&self, v : Vector) -> f32 {
    self.dist3(v.x, v.y)
  }

  ///
  pub fn dist3(&self, px : f32 /*= 0*/, py : f32 /*= 0*/) -> f32 {
    let ax = (self.x - px).abs();
    let ay = (self.y - py).abs();
    if ax > ay {
      ax + ay / 2.0
    } else {
      ay + ax / 2.0
    }
  }

  pub fn distAcc(&self, v : Vector) -> f32 {
    ((v.x - self.x) * (v.x - self.x) + (v.y - self.y) * (v.y - self.y)).sqrt()
  }

  // was contains
  pub fn containsVectorRadius(&self, p : Vector, r : f32) -> bool {
    self.containsXYRadius(p.x, p.y, r)
  }

  // was contains
  pub fn containsVector(&self, p : Vector) -> bool {
    self.containsXYRadius(p.x, p.y, 1.0)
  }

  // was contains
  pub fn containsXY(&self, px : f32, py : f32) -> bool {
    self.containsXYRadius(px, py, 1.0)
  }

  // was contains
  pub fn containsXYRadius(&self, px : f32, py : f32, r : f32) -> bool {
    (px >= (-self.x * r) && (px <= self.x * r) && (py >= -self.y * r) && (py <= self.y * r))
  }

  pub fn roll(&mut self, d : f32) {
    let tx = self.x * d.cos() - self.y * d.sin();
    self.y = self.x * d.sin() + self.y * d.cos();
    self.x = tx;
  }

  pub fn toString(&self) -> String {
    format!("({}, {})", self.x, self.y)
  }
}

static mut RSL3 : Vector3 = Vector3{x : 0.0, y : 0.0, z : 0.0};

/**
 * 3D vector.
 */
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3 {
  pub x : f32,
  pub y : f32,
  pub z : f32,
}

impl Mul for Vector3 {
  type Output = f32;

  fn mul(self, v : Vector3) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }
}

impl<'a, 'b> Mul<&'b Vector3> for &'a Vector3 {
  type Output = f32;

  fn mul(self, v : &'b Vector3) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }
}

impl Vector3 {
  /*
  public static this() {
    rsl3 = new Vector3;
  }

  public this() {
    clear();
  }

  public this(float x, float y, float z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }*/

  pub fn new(x : f32, y : f32, z:f32) -> Vector3 {
    Vector3 {x: x, y: y, z: z}
  }

  pub fn clear(&mut self) {
    self.x = 0.0;
    self.y = 0.0;
    self.z = 0.0;
  }

  pub fn rollX(&mut self, d : f32) {
    let ty = self.y * d.cos() - self.z * d.sin();
    self.z = self.y * d.sin() + self.z * d.cos();
    self.y = ty;
  }

  pub fn rollY(&mut self, d : f32) {
    let tx = self.x * d.cos() - self.z * d.sin();
    self.z = self.x * d.sin() + self.z * d.cos();
    self.x = tx;
  }

  pub fn rollZ(&mut self, d : f32) {
    let tx = self.x * d.cos() - self.y * d.sin();
    self.y = self.x * d.sin() + self.y * d.cos();
    self.x = tx;
  }

  pub fn blend(&mut self, v1 : Vector3, v2 : Vector3, ratio : f32) {
    self.x = v1.x * ratio + v2.x * (1.0 - ratio);
    self.y = v1.y * ratio + v2.y * (1.0 - ratio);
    self.z = v1.z * ratio + v2.z * (1.0 - ratio);
  }

  pub fn vctSize(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  // was dist()
  pub fn distVector3(&self, v : Vector3) -> f32 {
    self.distXYZ(v.x, v.y, v.z)
  }

  // was dist()
  pub fn distXYZ(&self, px : f32, py : f32, pz : f32) -> f32 {
    let ax = (self.x - px).abs();
    let ay = (self.y - py).abs();
    let az = (self.z - pz).abs();

    let axy = if ax > ay {
      ax + ay / 2.0
    } else {
      ay + ax / 2.0
    };

    if axy > az {
      axy + az / 2.0
    } else {
      az + axy / 2.0
    }
  }

  // was getElement
  pub unsafe fn getElementVector3(&self, v : &Vector3) -> Vector3 {
    let ll = v * v;
    if ll != 0.0 {
      let mag : f32 = self * v;
      RSL3.x = mag * v.x / ll;
      RSL3.y = mag * v.y / ll;
      RSL3.z = mag * v.z / ll;
    } else {
      RSL3.x = 0.0;
      RSL3.y = 0.0;
      RSL3.z = 0.0;
    }

    RSL3
  }

  pub fn op_mul(&self, v : Vector3) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }

  pub fn op_add_assign(&mut self, v : Vector3) {
    self.x += v.x;
    self.y += v.y;
    self.z += v.z;
  }

  pub fn op_sub_assign(&mut self, v : Vector3) {
    self.x -= v.x;
    self.y -= v.y;
    self.z -= v.z;
  }

  pub fn op_mul_assign(&mut self, a : f32) {
    self.x *= a;
    self.y *= a;
    self.z *= a;
  }

  pub fn op_div_assign(&mut self, a : f32) {
    self.x /= a;
    self.y /= a;
    self.z /= a;
  }


  pub fn to_string(&self) -> String {
    format!("({}, {}, {})", self.x, self.y, self.z)
  }
}
