use proconio::input;
use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModInt<const MOD: usize> {
    val: usize,
}

impl<const MOD: usize> fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const MOD: usize> ModInt<MOD> {
    pub fn new(n: usize) -> Self {
        Self { val: n % MOD }
    }

    pub fn val(&self) -> usize {
        // 念のためMOD演算
        self.val % MOD
    }

    pub fn _set_val(&mut self, val: usize) {
        self.val = val % MOD;
    }

    pub fn pow_u(&self, mut n: usize) -> Self {
        let mut val = self.val;
        let mut res: usize = 1;
        while n > 0 {
            if n % 2 == 1 {
                res = (res * val) % MOD;
            }
            val = (val * val) % MOD;
            n /= 2;
        }

        Self { val: res }
    }

    pub fn pow(&self, other: Self) -> Self {
        self.pow_u(other.val)
    }

    pub fn inv(&self) -> Self {
        self.pow_u(MOD - 2)
    }
}

impl<const MOD: usize> ops::Add for ModInt<MOD> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: (self.val + other.val) % MOD,
        }
    }
}

impl<const MOD: usize> ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            val: (self.val + other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Mul for ModInt<MOD> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            val: (self.val * other.val) % MOD,
        }
    }
}

impl<const MOD: usize> ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            val: (self.val * other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Sub for ModInt<MOD> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if self.val < other.val {
            self.val += MOD;
        }
        Self {
            val: self.val - other.val % MOD,
        }
    }
}

impl<const MOD: usize> ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, other: Self) {
        if self.val < other.val {
            self.val += MOD;
        }
        *self = Self {
            val: (self.val - other.val) % MOD,
        };
    }
}

impl<const MOD: usize> ops::Div for ModInt<MOD> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.val == 0 {
            panic!("0 division occured.");
        }

        self * other.inv()
    }
}

impl<const MOD: usize> ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, other: Self) {
        if other.val == 0 {
            panic!("0 division occured.");
        }

        *self *= other.inv();
    }
}

pub struct ModCom<const MOD: usize> {
    fac: Vec<usize>,
    finv: Vec<usize>,
}

impl<const MOD: usize> ModCom<MOD> {
    pub fn new(cap: usize) -> Self {
        let mut fac = vec![0; cap];
        let mut finv = vec![0; cap];
        let mut inv = vec![0; cap];
        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..cap {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Self { fac, finv }
    }

    pub fn com(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % MOD) % MOD
    }
}

use std::collections::HashMap;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let mut acc = vec![0_i64; n+1];
    for i in 0..n { acc[i+1] = acc[i] + a[i]; }
    let mut map = HashMap::new();
    let mut dp = vec![ModInt::<MOD>::new(0); n+1];
    dp[0] = ModInt::<MOD>::new(1);
    let mut sum = ModInt::<MOD>::new(0);
    for i in 0..n {
        sum += dp[i];
        dp[i+1] = sum;
        *map.entry(acc[i]).or_insert(ModInt::<MOD>::new(0)) += dp[i];
        if let Some(&val) = map.get(&(acc[i+1] - k)) {
            dp[i+1] -= val;
        }
    }
    println!("{}", dp[n]);
}
