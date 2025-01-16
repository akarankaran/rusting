fn main() {
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4), None, Some(6)];

    let mut iter = values.iter();

    while let Some(&Some(value)) = iter.next() {
        println!("Processing value: {}", value);
    }

    let mut iter2 = values.iter();

    while let Some(maybe_value) = iter2.next() {
        if let Some(value) = maybe_value {
            println!("Value found: {}", value);
        } else {
            println!("No value present.");
        }
    }

    let mut count = 0;

    while let Some(value) = values.get(count) {
        match value {
            Some(num) => println!("Value at index {}: {}", count, num),
            None => println!("No value at index {}", count),
        }
        count += 1;
    }

    let mut non_none_count = 0;
    let mut non_none_values = vec![];

    while let Some(&current) = values.get(non_none_count) {
        if let Some(val) = current {
            non_none_values.push(val);
        }
        non_none_count += 1;
    }

    println!("All non-none values: {:?}", non_none_values);
}