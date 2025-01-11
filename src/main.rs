fn largest_ref(v: &[i32]) -> &i32 {
    let mut largest = &v[0];
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref_generic<T: PartialOrd>(v: &[T]) -> &T {
    let mut largest = &v[0];
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref_with_none(v: &[Option<i32>]) -> Option<&i32> {
    let mut largest: Option<&i32> = None;
    for item in v {
        if let Some(value) = item {
            match largest {
                Some(l) if value > l => largest = Some(value),
                None => largest = Some(value),
                _ => {}
            }
        }
    }
    largest
}

fn largest_ref_with_default(v: &[i32], default: i32) -> &i32 {
    let mut largest = &default;
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref_with_index(v: &[i32]) -> (usize, &i32) {
    let mut max_index = 0;
    for (index, item) in v.iter().enumerate() {
        if item > &v[max_index] {
            max_index = index;
        }
    }
    (max_index, &v[max_index])
}