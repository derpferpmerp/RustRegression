extern crate libm;

fn calc_avg(l: &[f64]) -> f64 {
	let mut sum = 0f64;
	for i in l {
		sum += i;
	}
	let size = l.len() as f64;
	&sum / (&size)
}

fn list_range(b: i64, e: i64) -> Vec<f64> {
	((b)..(e)).map(|v| v as f64).collect()
}

fn calc_sxx_or_sxy(l: &[f64]) -> f64 {
	let avg = calc_avg(l);
	let mut sxxval = 0f64;
	for i in l {
		let v = (i - avg) as f64;
		sxxval += f64::powf(v, 2.0);
	}
	sxxval
}

fn calc_sxy(l_x: &[f64], l_y: &[f64]) -> f64 {
	let avg_x = calc_avg(l_x);
	let avg_y = calc_avg(l_y);
	let mut sxyval = 0f64;
	for i in 0..(l_x.len()) {
		sxyval += (l_x[i] - avg_x) * (l_y[i] - avg_y);
	}
	sxyval
}

fn smart_form_num(num: f64) -> String {
	if num == 0f64 {
		return String::from("");
	}
	return match num > 0f64 {
		true => format!("+ {}", num),
		false => format!("- {}", num * -1f64),
	};
}

fn type_of<T>(_: &T) -> String {
	return std::any::type_name::<T>().to_string();
}


fn nat_log(x: f32) -> f32 {
	let euler: f32 = libm::expf(1f32);
	return libm::logf(x) / libm::logf(euler);
}


struct Regression {
	xl: Vec<f64>,
	yl: Vec<f64>,
}

impl Regression {
	fn linear(&self) -> String {
		let slope = calc_sxy(&self.xl, &self.yl) / calc_sxx_or_sxy(&self.xl);
		let intercept = calc_avg(&self.yl) - (slope * calc_avg(&self.xl));
		return format!(
			"[ LINEAR ] y = {m}x {opt}",
			m = slope,
			opt = smart_form_num(intercept)
		);
	}
	fn logarithmic(&self) {
		let mut ln_x: Vec<f32> = (self.xl).iter().map(|x| nat_log(*x as f32)).collect();
		println!("{:#?}", ln_x);
	}
}

fn main() {
	let l = list_range(0, 1000);
	let ly = list_range(1000, 2000);
	println!("{:?}", calc_sxx_or_sxy(&l));
	println!("{:?}", calc_sxy(&l, &ly));
	let reg_obj = Regression { xl: l, yl: ly };
	println!("{:?}", reg_obj.linear());
	reg_obj.logarithmic();
}
