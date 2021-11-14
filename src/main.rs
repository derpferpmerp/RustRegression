extern crate libm;

fn calc_avg(l: &Vec<f64>) -> f64 {
    let mut sum = 0f64;
    for i in l {
        sum += i;
    }
    let size = l.len() as f64;
    return &sum / (&size);
}

fn list_range(b: i64, e: i64) -> Vec<f64> {
    return ((b)..(e)).map(|v| v as f64).collect();
}

fn calc_sxx_or_sxy(l: &Vec<f64>) -> f64 {
    let avg = calc_avg(&l);
    let mut sxxval = 0f64;
    for i in l {
        let v = (i - avg) as f64;
        sxxval += f64::powf(v, 2.0);
    }
    return sxxval;
}

fn calc_sxy(l_x: &Vec<f64>, l_y: &Vec<f64>) -> f64 {
    let avg_x = calc_avg(&l_x);
    let avg_y = calc_avg(&l_y);
    let mut sxyval = 0f64;
    for i in 0..(l_x.len()) {
        sxyval += (l_x[i] - avg_x) * (l_y[i] - avg_y);
    }
    return sxyval;
}

fn smart_form_num(num: f64) -> String {
    if num == 0f64 {
        return String::from("");
    }
    return match num > 0f64 {
        true => format!("+ {}", num).to_string(),
        false => format!("- {}", num * -1f64).to_string(),
    };
}

struct Regression {
    xl: Vec<f64>,
    yl: Vec<f64>,
}

impl Regression {
    fn linear(&self) -> String {
        let mut slope = calc_sxy(&self.xl, &self.yl) / calc_sxx_or_sxy(&self.xl);
        let mut intercept = calc_avg(&self.yl) - (&slope * calc_avg(&self.xl));
        return format!(
            "[ LINEAR ] y = {m}x {opt}",
            m = slope,
            opt = smart_form_num(intercept)
        )
        .to_string();
    }
    fn logarithmic(&self) {
        let euler: f32 = libm::expf(1f32);
        let mut ln_x: Vec<f32> = Vec::new();
        for i in &self.xl {
            let mut form = *i as f32;
            ln_x.push(libm::logf(form) / libm::logf(euler));
        }
        println!("{:?}", ln_x);
    }
}

fn main() {
    let l = list_range(0, 1000);
    let ly = list_range(1000, 2000);
    println!("{:?}", calc_sxx_or_sxy(&l));
    println!("{:?}", calc_sxy(&l, &ly));
    let mut reg_obj = Regression { xl: l, yl: ly };
    println!("{:?}", reg_obj.linear());
    reg_obj.logarithmic();
}
