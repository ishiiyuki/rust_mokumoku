use super::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> 
{
    match *order{
        SortOrder::Ascending => sort_by(x, &|a,b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a,b| b.cmp(a)),
    }

}
pub fn sort_by <T,F>(x: &mut [T], comparator: &F) -> Result<(),String>
    where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true,comparator);
        Ok(())
    }
    else{
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}


fn do_sort<T,F>(x: &mut [T],forward: bool,comparator: &F)
    where F: Fn(&T,&T) -> Ordering
{
    if x.len() >1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true,comparator);
        do_sort(&mut x[mid_point..], false,comparator);
        sub_sort(x, forward,comparator);
    }
}

fn sub_sort<T,F>(x: &mut [T],forward: bool,comparator: &F)
where F: Fn(&T,&T) -> Ordering
{
    if x.len() >1 {
        compare_and_swap(x, forward,comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward,comparator);
        sub_sort(&mut x[mid_point..], forward,comparator);
    }
}


fn compare_and_swap<T,F>(x: &mut [T],forward: bool,comparator: &F)
where F: Fn(&T,&T) -> Ordering
{
    // 比較に先立ちforward（bool値）をOrdering値に変換しておく
    let swap_condition = if forward{
        Ordering::Greater
    }
    else
    {
        Ordering::Less
    };

    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition{
            x.swap(i, mid_point + i);
        }
    }
}

//単体テストコード 
// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests{
    use super::{sort,sort_by};
    use crate::SortOrder::*;

    // 構造体Studentを定義する
    // 構造体は関連する値を1つにまとめたデータ構造。複数のデータフィールドを持つ

    // deriveアトリビュートを使い、DebugトレイトとPartialEqトレイトの実装を自動導出する
    #[derive(Debug,PartialEq)]
    struct Student{
        first_name: String, 
        last_name: String,
        age: u8,
    }

    impl Student{

        //new 
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }

    }

    #[test]
    fn sort_to_fail(){
        let mut x = vec![10,30,11]; //２のべき乗ではない
        assert!(sort(&mut x, &Ascending).is_err());//戻り値はエラー
    }

    #[test]
    fn sort_u32_ascending(){

        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[test]
    fn sort_students_aby_age_ascending()
    {
        let taro = Student::new ("Taro","Yamada",16);
        let hanako = Student::new("Hanako","Yamada",14);
        let kyoko = Student::new("Kyoko","Ito",15);
        let ryosuke = Student::new("Ryosuke","Hayashi",17);

        let mut x = vec![&taro,&hanako,&kyoko,&ryosuke];
        let expected = vec![&ryosuke,&kyoko,&hanako,&taro];

        assert_eq!(sort_by(&mut x,
            // まずlast_nameを比較する
            &|a, b| a.last_name.cmp(&b.last_name)
                // もしlast_nameが等しくない（LessまたはGreater）ならそれを返す
                // last_nameが等しい（Equal）ならfirst_nameを比較する
                .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())
        );

        assert_eq!(x,expected);
    }

}