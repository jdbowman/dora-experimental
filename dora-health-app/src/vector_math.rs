

fn vec_add<'c, T>(a: &'c Vec<T>, b: &'c Vec<T>) -> Vec<T> 
where &'c T: std::ops::Add<&'c T, Output=T>
{
	let mut result: Vec<T> = Vec::with_capacity(a.len());
	for (i, j) in a.iter().zip(b) {
		result.push(i + j);
	}
	result
}


fn vec_sub<'c, T>(a: &'c Vec<T>, b: &'c Vec<T>) -> Vec<T>
where &'c T: std::ops::Sub<&'c T, Output=T>
{
	let mut result: Vec<T> = Vec::with_capacity(a.len());
	for (i, j) in a.iter().zip(b) {
		result.push(i - j);
	}
	result
}


fn vec_sum<'c, T>(a: &'c Vec<T>) -> T
where T: std::ops::AddAssign<T> + num_traits::identities::Zero + std::marker::Copy
{
	let mut result = T::zero();
	for i in a.iter() {
		result += *i;
	}
	result
}
