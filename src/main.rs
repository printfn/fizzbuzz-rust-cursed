#![feature(iter_intersperse)]

fn main() {
    println!(
        "{}",
        fizzbuzz(
            1..=100,
            [
                ((|i| i % 3 == 0) as fn(&i32) -> bool, "Fizz"),
                (|i| i % 5 == 0, "Buzz"),
            ]
        )
    );
}

fn fizzbuzz<T: ToString>(
    arr: impl IntoIterator<Item = T>,
    predicates: impl IntoIterator<Item = (impl FnMut(&T) -> bool, impl ToString)> + Clone,
) -> String {
    arr.into_iter()
        .map(|x| {
            predicates
                .clone()
                .into_iter()
                .filter_map(|(mut p, s)| p(&x).then_some(s.to_string()))
                .reduce(|a, b| format!("{a}{b}"))
                .unwrap_or(x.to_string())
        })
        .intersperse("\n".to_string())
        .collect()
}
