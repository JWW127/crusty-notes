use std::ops::{Deref, DerefMut};

// we can use Deref & DerefMut for cleaner code
pub struct MyStruct<T> {
    val: T,
}

// we are just defining how Deref should be impl for MyStruct
impl<T> Deref for MyStruct<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

// we are just defining how DerefMut should be impl for MyStruct
impl<T> DerefMut for MyStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

//Since we defined how our struct should handle Deref and DerefMut we can do this

fn test_deref() {
    let mut struct_instance = MyStruct { val: 'x' };
    *struct_instance = 'y';
    assert_eq!('y', struct_instance.val);
}
