//Bubble sort
fn main() {
    let mut arr = [234,24,3,5,445,342355];
    let mut temp;
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    println!("排序后的结果是 {:?}", arr);
}
