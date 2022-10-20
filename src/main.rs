// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//             let sub_arr = [6,8,10];

// Bài tập 2 : Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
// Ví dụ: let input = “adbcdaDd”. 

// Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”

// Nhập s = ‘d’ => in ra kết quả : 4, “abca”


fn main() {
    println!("Hello, world!");
}

mod excerse1 {
    fn is_sub_array<T>(a: Vec<T>, b: Vec<T>) -> bool 
    where T: PartialEq
    {
        let a_len = a.len();
        let b_len = b.len();
        if a_len >= b_len {
            return a.windows(b_len).any(|x| x == b);
        }
        false
    }

    #[test]
    fn test_is_super() {
        let super_arr = vec![1,2,3];
        let sub_arr = vec![1,2];
        assert_eq!(is_sub_array(super_arr, sub_arr), true)
    }

    #[test]
    fn test_is_sub() {
        let super_arr = vec![1,2,3];
        let sub_arr = vec![1,2];
        assert_eq!(is_sub_array(sub_arr, super_arr), false)
    }

    #[test]
    fn test_is_equal() {
        let super_arr = vec![1,2];
        let sub_arr = vec![1,2];
        assert_eq!(is_sub_array(super_arr, sub_arr), true)
    }
}

mod excerse2 {
    fn check_character(mother_str: &str, ch : char) -> (usize, String) {
        let str : String = mother_str.chars().filter(|e| *e != ch).collect();

        (mother_str.len() - str.len(), str)
    }

    #[test]
    fn check_1() {
        let str = "abca";
        let ch = 'a';
        assert_eq!(check_character(str, ch).0, 2)
    }

    #[test]
    fn check_2() {
        let str = "abca";
        let ch = 'a';
        assert_eq!(check_character(str, ch).1, "bc".to_string())
    }
}
