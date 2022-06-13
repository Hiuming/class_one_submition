pub fn SolutionEx1() {
    let vec1 = vec![1, 2,3,5,6,8, 10, 11];
    let vec2 = vec![6,8,10];
    print!("vec 1 : {:?}",vec1);
    print!("vec 2 : {:?}",&vec2);
    println!("is sub array: {}", is_sub_array(&vec1, &vec2));
}

//Solution for exercise #1
fn is_sub_array<T:PartialEq>(_parent_list: &[T],_child_list: &[T]) ->bool{
    if _child_list.len() == 0 {return true;}
   return _parent_list.windows(_child_list.len()).any(|c| c == _child_list);
}