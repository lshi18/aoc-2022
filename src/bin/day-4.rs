use std::fs;

fn main() {
    let input = String::from_utf8(fs::read("input-day4.txt").unwrap()).unwrap();
    let parsed = input.trim_end().split("\n").map(|s| {
        s.split(&[',', '-'])
            .map(|cs| cs.parse().unwrap())
            .collect::<Vec<u32>>()
    });

    let res1 = parsed.clone().filter(|v| is_contained(v)).count();

    let res2 = parsed.filter(|v| is_overlapped(v)).count();
    //.collect::<Vec<_>>();

    assert!(res1 == 595, "res1 should be 595, got: #{}", res1);
    assert!(res2 == 952, "res1 should be 952, got: #{}", res2);
}

fn is_contained<T>(v: &[T]) -> bool
where
    T: PartialOrd,
{
    assert!(v.len() == 4);

    let a = &v[0];
    let b = &v[1];
    assert!(a <= b);

    let c = &v[2];
    let d = &v[3];
    assert!(c <= d);

    (a <= c && b >= d) || (a >= c && b <= d)
}

fn is_overlapped<T>(v: &[T]) -> bool
where
    T: PartialOrd,
{
    assert!(v.len() == 4);
    let a = &v[0];
    let b = &v[1];
    assert!(a <= b);

    let c = &v[2];
    let d = &v[3];
    assert!(c <= d);

    (c <= b && c >= a) || (a >= c && a <= d) || (b >= c && b <= d) || (d <= b && d >= a)
}
