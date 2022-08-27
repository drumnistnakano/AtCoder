use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        ax: usize,
        ay: usize,
        bx: usize,
        by: usize,
        cx: usize,
        cy: usize,
        dx: usize,
        dy: usize
    }

    // 180度を超える角がある場合はそのままリターン
    // TODO: 後で関数化
    {
        // a,b,c
        let ba= vec![bx-ax, by-ay];
        let bc= vec![bx-cx, cy-ay];
        let _ba = ((ba[0]*ba[0] + ba[1]*ba[1]) as f64).sqrt();
        let _bc = ((bc[0]*bc[0] + bc[1]*bc[1]) as f64).sqrt();
        let abc = (ba[0]*bc[0] + ba[1]*bc[1]) as f64;
        let cosTheta_abc = abc / _ba * _bc;
        let angle_abc = cosTheta_abc.acos() * 180.0 / PI;

        if angle_abc >= 180.0 {
            println!("No");
            return;
        }

        // // b,c,d
        // let cb= vec![cx-bx, cy-by];
        // let cd= vec![cx-dx, cy-dy];
        // let _cb = ((cb[0]*cb[0] + cd[1]*cd[1]) as f64).sqrt();
        // let _cd = ((cd[0]*cd[0] + cd[1]*cd[1]) as f64).sqrt();
        // let bcd = (cb[0]*cd[0] + cb[1]*cd[1]) as f64;
        // let cosTheta_bcd = abc / _cb * _cd;
        // let angle_bcd = cosTheta_bcd.acos() * 180.0 / PI;

        // if angle_bcd >= 180.0 {
        //     println!("No");
        //     return;
        // }

        // // c,d,a
        // let dc = vec![dx-cx, dy-cy];
        // let da = vec![dx-ax, dy-ay];
        // let _dc = ((dc[0]*dc[0] + dc[1]*dc[1]) as f64).sqrt();
        // let _da = ((da[0]*da[0] + da[1]*da[1]) as f64).sqrt();
        // let cda = (dc[0]*da[0] + dc[1]*da[1]) as f64;
        // let cosTheta_cda = cda / _dc * _da;
        // let angle_cda = cosTheta_cda.acos() * 180.0 / PI;

        // if angle_cda >= 180.0 {
        //     println!("No");
        //     return;
        // }

        // // d,a,c
        // let ad = vec![ax-dx, ay-dy];
        // let ac = vec![ax-cx, ay-cy];
        // let _ad = ((ad[0]*ad[0] + ad[1]*ad[1]) as f64).sqrt();
        // let _ac = ((ac[0]*ac[0] + ac[1]*ac[1]) as f64).sqrt();
        // let dac = (ad[0]*ac[0] + ad[1]*ac[1]) as f64;
        // let cosTheta_dac = dac / _ad * _ac;
        // let angle_dac = cosTheta_dac.acos() * 180.0 / PI;

        // if angle_dac >= 180.0 {
        //     println!("No");
        //     return;
        // }
    }

    // 180度を超える角がないので凸
    println!("Yes");
}
