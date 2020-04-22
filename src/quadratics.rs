
    #[derive(Debug)]
    pub enum QuadraticRoots {
        None,
        Double(f64),
        Couple(f64, f64)
    }

    impl QuadraticRoots {
        pub fn solve (a: f64, b: f64, c: f64) -> QuadraticRoots { 
            let discr = b * b - 4.0 * a * c; 
            if discr < 0.0 {
                return QuadraticRoots::None;
            } 
            else if discr == 0.0 {
                return QuadraticRoots::Double(- 0.5 * b / a);
            }  
            else { 
                let q = if b > 0.0 { -0.5 * (b + discr.sqrt()) } else { -0.5 * (b - discr.sqrt()) };
                let x0 = q / a; 
                let x1 = c / q; 
                if x0 < x1 {
                    return QuadraticRoots::Couple(x0, x1)
                } else {
                    return QuadraticRoots::Couple(x1, x0)
                }
            } 
        }

    }