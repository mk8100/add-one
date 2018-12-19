
/// sqrt function
/// 
pub fn sqrt(v:f64) -> Option<f64>{
    match v.sqrt() {
        x if x.is_normal() => Some(x),
        _                  => None,
    }
}

/// log function
/// 
pub fn log(v:f64) -> Option<f64>{
    match v.log(10.0) {
        x if x.is_normal() => Some(x),
        _                  => None,
    }
}

///negate function
///
pub fn neg(v:f64) -> f64{
    -1.0 * v
}

///pow(2) function
/// 
pub fn square(v:f32) -> f64{
    (v as f64).powi(2)
}

/// double function
/// 
pub fn double(v:f32) -> f64{
    (v as f64) * 2.0
}

pub fn double64(v:f64) -> f64{
    v.powi(10)
}

#[cfg(test)]
mod utests {

    use super::*;

    #[test]
    fn square_maxf32_test() {
        assert!(square(std::f32::MAX).is_finite());
    }

    #[test]
    fn double64_check(){
        let m = std::f64::MAX;
        assert!(double64(m).is_infinite());
    }

    #[test]
    fn double64_opt_chain(){
        let m = std::f64::MAX;
        let res = Some(3.0).map(double64).and_then(|x|{ Some(x) } );
        assert_eq!(None, res);
    }
}
