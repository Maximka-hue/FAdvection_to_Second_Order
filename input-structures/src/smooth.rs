pub(crate) fn smoothZF_rs(fs: &mut Vec<f64>, Nmax: usize, smooth_int: f64, fi:  &mut Vec<f64>, ftd:  &mut Vec<f64>){
    let mut sign: i64 = -1;
    let mut f1: f64;
    let mut f2: f64;

    for n in 0.. Nmax-1{
        fi[n]= smooth_int * (fs[n+1] - fs[n]);
    }
//So i need to determine somehow Fi[Nmax-1].
    fi[Nmax-1] = 0.0; //Notice, please.
    for n in 1.. Nmax-1{
        ftd[n] = fs[n] + (fi[n] - fi[n-1]);
    }
    ftd[0] = fs[0];
    ftd[Nmax -1] = fs[Nmax -1];
    for n in 0 .. (Nmax - 1){
        fs[n] = ftd[n+1] - ftd[n];
    }
//Main calculations ++++++++++++++++++++++++++++++++++++++++
    for n in 0..Nmax-1{
        fi[n] = smooth_int * (ftd[n+1] - ftd[n]);
        let sgn = fi[n].signum() as i64; // copy sign from calculated value
        if sgn >= 0 {
            fi[n] = fi[n].abs();
        sign = sgn
        }
        if n==0{
            f2= sign as f64 * fs[1];
            if f2< fi[0]{
                fi[0] = f2;
            }
        }
    else if n== Nmax-2{//Should be noticed!
        f1 = sign as f64 * fs[Nmax - 2];
        if  f1 < fi[Nmax - 1]//What value??? - 0;
        {
            fi[Nmax - 1] = f1;
        }
    }
    else
    {
        f1 = sign as f64 * fs[n-1];
        f2 = sign as f64 * fs[n+1];
        if f1 < fi[n]{
            fi[n] = f1;
        }
        if f2 < fi[n]{
            fi[n] = f2;
        }
    }
    if fi[n] < 0.0
        {fi[n] = 0.0;}
    else
      {fi[n] = sign as f64 * fi[n];}
    }
//-------------------------------------------
    for n in 1..Nmax{
        fs[n] = ftd[n] - (fi[n] - fi[n-1]);//fi[last] = 0;
        fs[0] = ftd[0];
    }
}
