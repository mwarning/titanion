/*
 * $Id: rand.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
//module src.util.rand;


//private import core.time;

#[allow(non_snake_case)]

/**
 * Random number generator.
 */
 /*
public template StaticRandImpl() {
 protected:
  static Rand rand = null;

  public static void setRandSeed(seed : i64) {
    if (!rand) {
      rand = new Rand;
    }
    rand.setSeed(seed);
  }
}
*/

//public class Rand {
/*
  fn this() {
    let timer = TickDuration.currSystemTick().nsecs();
    init_genrand(timer as u32);
  }
*/
impl State {
  fn setSeed(&mut self, n : i64) {
    self.init_genrand(n as u32);
  }

  fn nextInt32(&mut self) -> u32 {
    self.genrand_int32()
  }

  fn nextInt(&mut self, n : i32) -> i32 {
    if n == 0 {
      0
    } else {
      (self.genrand_int32() % (n as u32)) as i32
    }
  }

  fn nextSignedInt(&mut self, n : i32) -> i32 {
    if n == 0 {
      0
    } else {
      (self.genrand_int32() % ((n as u32) * 2 + 1) - (n as u32)) as i32
    }
  }

  fn nextFloat(&mut self, n : f32 /*n = 1*/) -> f32 {
    (self.genrand_real1() as f32) * n
  }

  fn nextSignedFloat(&mut self, n : f32 /*= 1*/) -> f32 {
    (self.genrand_real1() as f32) * (n * 2.0) - n
  }
}

/* 
   MT.d
   Mersenne Twister random number generator -- D
   Based on code by Makoto Matsumoto, Takuji Nishimura, Shawn Cokus,
     Matthe Bellew, and Isaku Wada
   Andrew C. Edwards  v0.1  30 September 2003  edwardsac@ieee.org

   Before using, initialize the state by using init_genrand(seed) 
   or init_by_array(init_key, key_length).

   Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura,
   Copyright (C) 2003, Andrew C. Edwards
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

     1. Redistributions of source code must retain the above copyright
        notice, this list of conditions and the following disclaimer.

     2. Redistributions in binary form must reproduce the above copyright
        notice, this list of conditions and the following disclaimer in the
        documentation and/or other materials provided with the distribution.

     3. The names of its contributors may not be used to endorse or promote 
        products derived from this software without specific prior written 
        permission.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

   The original code included the following notice:
  
     Any feedback is very welcome.
     http://www.math.keio.ac.jp/matumoto/emt.html
     email: matumoto@math.keio.ac.jp
  
   Please CC: edwardsac@ieee.org on all correspondence
*/

/* 
   Modified by Kenta Cho.
   Remove 'static' to wrap with a Rand class.
*/

/* Period parameters */
const N : usize = 624;
const M : usize = 397;
const MATRIX_A : u32 = 0x9908b0df;   /* constant vector a */
const UMASK : u32 = 0x80000000; /* most significant w-r bits */
const LMASK : u32 = 0x7fffffff; /* least significant r bits */
fn MIXBITS(u : u32, v : u32) -> u32 { (u & UMASK) | (v & LMASK) }
fn TWIST(u : u32, v : u32) -> u32 { (MIXBITS(u,v) >> 1) ^ (if (v & 1) != 0 {MATRIX_A} else {0}) }

struct State {
  state : [u32; N], /* the array for the state vector  */
  //uint state[N]; /* the array for the state vector  */
  left : usize,
  initf : i32,
  next : usize,
}

impl Default for State {
    fn default() -> State {
        State { state : [0u32; N], left : 1, initf : 0, next : 0 }
   }
}

impl State {

/* initializes state[N] with a seed */
fn init_genrand(&mut self, s : u32)
{
    self.state[0] = s & 0xffffffff;
    //for (int j=1; j<N; j++) {
    for j in 1..N {
        self.state[j] = (1812433253 * (self.state[j-1] ^ (self.state[j-1] >> 30)) + (j as u32)) as u32; 
        /* See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier. */
        /* In the previous versions, MSBs of the seed affect   */
        /* only MSBs of the array state[].                        */
        /* 2002/01/09 modified by Makoto Matsumoto             */
        self.state[j] &= 0xffffffff;  /* for >32 bit machines */
    }
    self.left = 1;
    self.initf = 1;
}

/* initialize by an array with array-length */
/* init_key is the array for initializing keys */
/* key_length is its length */
//uint init_key[];
//uint key_length;
fn init_by_array(&mut self, init_key : &Vec<u32>, key_length : usize)
{
    let mut i : usize = 1;
    let mut j : usize = 0;
    self.init_genrand(19650218);

    let kk : usize = if N > key_length { N } else { key_length };
    for k in 0 .. kk + 1 {
        self.state[i] = (self.state[i] ^ ((self.state[i-1] ^ (self.state[i-1] >> 30)) * 1664525)) as u32 + init_key[j] + (j as u32); /* non linear */
        self.state[i] &= 0xffffffff; /* for WORDSIZE > 32 machines */
        i += 1; j += 1;
        if i>=N { self.state[0] = self.state[N-1]; i=1; }
        if j>=key_length { j=0; }
    }

    for k in 0 .. N {
        self.state[i] = (self.state[i] ^ ((self.state[i-1] ^ (self.state[i-1] >> 30)) * 1566083941)) - (i as u32) as u32; /* non linear */
        self.state[i] &= 0xffffffff; /* for WORDSIZE > 32 machines */
        i += 1;
        if i >= N {
          self.state[0] = self.state[N-1];
          i = 1;
        }
    }

    self.state[0] = 0x80000000; /* MSB is 1; assuring non-zero initial array */ 
    self.left = 1;
    self.initf = 1;
}

fn next_state(&mut self)
{
    //uint *p = state.ptr;

    /* if init_genrand() has not been called, */
    /* a default initial seed is used         */
    if self.initf == 0 {
      self.init_genrand(5489);
    }

    self.left = N;
    self.next = 0;
   //next = state.ptr;

    let mut p : usize = 0;
    //TODO: use while loops
    for i in 0..N-M-1 {
      p += 1;
      self.state[p] = self.state[p + M] ^ TWIST(self.state[p + 0], self.state[p + 1]);
    }
  
    for i in 0..M {
      p += 1;
      self.state[p] = self.state[p + M - N] ^ TWIST(self.state[p + 0], self.state[p + 1]);
    }

    self.state[p] = self.state[p + M - N] ^ TWIST(self.state[p + 0], self.state[0]);
}

/* generates a random number on [0,0xffffffff]-interval */
fn genrand_int32(&mut self) -> u32
{
    self.left -= 1;
    if self.left == 0 {
      self.next_state();
    }

    let mut y = self.state[self.next]; //*next++;
    self.next += 1;

    /* Tempering */
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    y
}

/* generates a random number on [0,0x7fffffff]-interval */
fn genrand_int31(&mut self) -> i64
{
    self.left -= 1;
    if self.left == 0 {
      self.next_state();
    }
    //y = *next++;
    let mut y = self.state[self.next];
    self.next += 1;

    /* Tempering */
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    (y>>1) as i64
}

/* generates a random number on [0,1]-real-interval */
fn genrand_real1(&mut self) -> f64
{
    self.left -= 1;
    if self.left == 0 {
      self.next_state();
    }

    //y = *next++;
    let mut y = self.state[self.next];
    self.next += 1;

    /* Tempering */
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    (y as f64) * (1.0 / 4294967295.0)
    /* divided by 2^32-1 */ 
}

/* generates a random number on [0,1)-real-interval */
fn genrand_real2(&mut self) -> f64
{
    self.left -= 1;
    if self.left == 0 {
      self.next_state();
    }
    //y = *next++;
    let mut y = self.state[self.next];
    self.next += 1;

    /* Tempering */
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    (y as f64) * (1.0 / 4294967296.0)
    /* divided by 2^32 */
}

/* generates a random number on (0,1)-real-interval */
fn genrand_real3(&mut self) -> f64
{
    self.left -= 1;
    if self.left == 0 {
      self.next_state();
    }
    //y = *next++;
    let mut y = self.state[self.next];
    self.next += 1;

    /* Tempering */
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    ((y as f64) + 0.5) * (1.0/4294967296.0)
    /* divided by 2^32 */
}

/* generates a random number on [0,1) with 53-bit resolution*/
fn genrand_res53(&mut self) -> f64() 
{
  let a : u32 = self.genrand_int32() >> 5;
  let b : u32 = self.genrand_int32() >> 6;
  ((a as f64) * 67108864.0 + (b as f64)) * (1.0 / 9007199254740992.0)
}

}

fn main() {}

//}
