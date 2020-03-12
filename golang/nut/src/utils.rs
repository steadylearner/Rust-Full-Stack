use std::mem::{align_of, size_of, size_of_val};
use std::slice::{from_raw_parts, from_raw_parts_mut};

/// finds continuous block in sorted vector
pub fn find_contiguous(arr: &[u64], len: usize) -> Option<usize> {
	let mut max_found = 1usize;
	let arrlen = arr.len();
	if arrlen == 0 {
		return None;
	};
	if arrlen < len as usize {
		return None;
	};
	if len == 0 {
		return None;
	};
	if len == 1 {
		return Some(0);
	};
	for (index, item) in arr.iter().enumerate().take(arrlen - 1) {
		let next = arr[index + 1];
		if next - item == 1 {
			max_found += 1;
		} else {
			max_found = 1;
		}

		if max_found >= len {
			return Some(index + 2 - max_found);
		}
	}
	None
}

/// converts any to bytes slice
pub unsafe fn to_u8_slice<T: Sized>(p: &T) -> &[u8] {
	from_raw_parts((p as *const T) as *const u8, size_of::<T>())
}

/// converts any to bytes slice
pub unsafe fn slice_to_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
	from_raw_parts(p.as_ptr() as *const u8, size_of::<T>() * p.len())
}

/// converts any to bytes slice
#[allow(dead_code)]
pub unsafe fn to_u8_slice_mut<T: Sized>(p: &mut T) -> &mut [u8] {
	from_raw_parts_mut((p as *mut T) as *mut u8, size_of::<T>())
}

/// converts any to bytes slice
#[allow(dead_code)]
pub unsafe fn slice_to_u8_slice_mut<T: Sized>(p: &mut [T]) -> &mut [u8] {
	from_raw_parts_mut(p.as_mut_ptr() as *mut u8, size_of::<T>() * p.len())
}

#[allow(dead_code)]
pub fn is_aligned<T>(ptr: *const T) -> bool {
	(ptr as usize) % align_of::<T>() == 0
}

/// dump writes n bytes of the page to STDERR as hex output.
///
/// passing 0 as max will print entire struct
#[allow(dead_code)]
pub fn hexdump<T: Sized>(input: &T, max: usize) {
	let vsize = size_of_val(input);
	let size = if max == 0 {
		vsize
	} else {
		usize::min(vsize, max)
	};
	let buf = unsafe { to_u8_slice(input) };
	eprintln!("{:x?}", &buf[..size]);
}

/// dump writes n bytes of the page to STDERR as hex output.
///
/// passing 0 as max will print entire struct
#[allow(dead_code)]
pub fn hexdump_slice<T: Sized>(input: &[T], max: usize) {
	let buf = unsafe { from_raw_parts(input.as_ptr() as *const u8, max) };
	eprintln!("{:x?}", &buf);
}

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
	if n < min {
		return min;
	}
	if n > max {
		return max;
	}
	n
}

#[cfg(test)]
pub mod tests {
	use super::{find_contiguous, slice_to_u8_slice, to_u8_slice};

	#[test]
	fn test_find_contiguous() {
		assert_eq!(find_contiguous(&[1, 2, 3], 1), Some(0));
		assert_eq!(find_contiguous(&[7, 9, 18], 1), Some(0));
		assert_eq!(find_contiguous(&[1, 2, 3, 4, 5, 6, 7, 8], 3), Some(0));
		assert_eq!(find_contiguous(&[1, 3, 5, 6, 7, 8, 9, 10], 5), Some(2));
		assert_eq!(find_contiguous(&[4, 5, 7, 9, 10, 11, 12, 18], 3), Some(3));
		assert_eq!(find_contiguous(&[1, 3, 5, 7, 9, 11], 5), None);
		assert_eq!(find_contiguous(&[7, 9, 12, 13, 18], 2), Some(2));
		assert_eq!(find_contiguous(&[7, 9, 12, 13], 2), Some(2));
	}

	#[test]
	fn test_to_u8_slice() {
		let sl = &[1u8, 23, 24, 5, 65];
		assert_eq!(sl, unsafe { to_u8_slice(sl) });

		let sl = &[1u16, 23, 24, 5, 65];
		assert_eq!([1, 0, 23, 0, 24, 0, 5, 0, 65, 0], unsafe {
			to_u8_slice(sl)
		});
	}

	#[test]
	fn test_slice_to_u8_slice() {
		let sl: &[u8] = &[1u8, 23, 24, 5, 65];
		assert_eq!(sl, unsafe { slice_to_u8_slice(sl) });

		let sl: &[u16] = &[1u16, 23, 24, 5, 65];
		assert_eq!([1, 0, 23, 0, 24, 0, 5, 0, 65, 0], unsafe {
			slice_to_u8_slice(sl)
		});
	}
}
