//Examples
use std::fmt::{self, Debug};

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayPair")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}
#[test]
fn main1() {
    let pair = ArrayPair {
        left: [1, 2, 3],
        right: [4, 5, 6],
    };

    println!("{:?}", pair);
}
#[allow(dead_code)]
fn foo<const N: usize>() {}
#[allow(dead_code)]
fn bar<T, const M: usize>() {
    foo::<M>(); // ОК: `M` є константним параметром
    foo::<2021>(); // ОК: `2021` — це літерал
    foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // ОК: константний вираз, що не містить генеричних параметрів

    // foo::<{ M + 1 }>(); // ПОМИЛКА: константний вираз містить генеричний параметр `M`
    // foo::<{ std::mem::size_of::<T>() }>(); // ПОМИЛКА: константний вираз містить генеричний параметр `T`

    let _: [u8; M]; // ОК: `M` є константним параметром

    // let _: [u8; std::mem::size_of::<T>()]; // ПОМИЛКА: константний вираз містить генеричний параметр `T`
}
#[test]
fn main2() {}
#[allow(dead_code)]
pub struct MinSlice<T, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// The rest of the slice.
    pub tail: Vec<T>,
}

impl<T: Copy, const N: usize> MinSlice<T, N> {
    pub fn from_slice(slice: &[T]) -> Option<Self> {
        if slice.len() < N {
            return None;
        }

        let head = slice[..N].try_into().ok()?;
        let tail = slice[N..].to_vec();
        Some(MinSlice { head, tail })
    }
}
#[test]
fn main3() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ');
}
//Exercises
#[allow(dead_code)]
struct Array<T, const N: usize> {
    data : [T; N]
}
#[test]
fn main4() {
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 4]
        }
    ];
}
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
#[test]
fn main5() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}


