
pub fn sort(x: &mut [u32], up: bool)
{
    if x.len() >1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool)
{
    if x.len() >1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}


fn compare_and_swap(x: &mut [u32], up: bool)
{
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if(x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

//単体テストコード 
// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests{
    use super::sort;

    #[test]
    fn sort_u32_ascending(){

       // テストデータとしてu32型のベクタを作成しxに束縛する
        // sort関数によって内容が更新されるので、可変を表すmutキーワードが必要
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // `&mut x`は`&mut x[..]`と書いてもいい
        sort(&mut x, true);

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
}