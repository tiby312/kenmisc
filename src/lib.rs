
extern crate rand;

//use std;

use std::time::Instant;


unsafe impl<T:Send> std::marker::Send for PreVec<T>{}

///A construct to avoid excessive dynamic allocation by reusing a Vec
pub struct PreVec<T>{
    vec:Vec<* mut T>
}
impl<T> PreVec<T>{
	#[inline(always)]
    pub fn new()->PreVec<T>{
        PreVec{vec:Vec::new()}
    }

    ///Clears the vec and returns a mutable reference to a vec.
    #[inline(always)]
    pub fn get_empty<'a>(&'a mut self)->&mut Vec<&'a mut T>{
        self.vec.clear();
        unsafe{std::mem::transmute(&mut self.vec)}
    }
    ///Clears the vec and returns a mutable reference to a vec.
    #[inline(always)]
    pub fn get_empty_const<'a>(&'a mut self)->&mut Vec<&'a T>{
        self.vec.clear();
        unsafe{std::mem::transmute(&mut self.vec)}
    }
}






///An experimental construct that integrates some signal.
#[derive(Debug,Copy, Clone)]
pub struct Integrator<T:std::ops::AddAssign+Clone+Copy>{
    val:T
}

impl<T:std::ops::AddAssign+Clone+Copy> Integrator<T>{
    //Assume delta time is one.
    pub fn new(val:T)->Integrator<T>{
        Integrator{val:val}
    }
    pub fn time_step(&mut self,val:T)->&T{
        self.val+=val;
        &self.val
    }
    pub fn get(&self)->&T{
        &self.val
    }
    pub fn get_mut(&mut self)->&mut T{
        &mut self.val
    }
}

///Modify a slice to point to one less element, and return that element.
pub fn pop_first_element<'a,T>(source:&mut &'a mut [T])->&'a mut T{
    let k=&mut [];
    let nodes2=std::mem::replace(source,k);
    let (a,b)=nodes2.split_at_mut(1);

    let aa=&mut a[0];
    std::mem::replace(source,b);
    return aa
}


///Return an empty slice at the other of another slice.
///To avoid using unsafe{}, passing a refernece to the slice, and modify it.
pub fn get_empty_slice_at_end<'a,T>(arr:&mut &'a mut [T])->&'a mut [T]{
    let k=std::mem::replace(arr,&mut []);
    
    let len=k.len();
    let (a,b)=k.split_at_mut(len);
    
    std::mem::replace(arr,a);
    return b                    
}

///Returns true if two slices are contiguous in memory.
pub fn slices_contiguous<T>(first:&[T],second:&[T])->bool{
    let f1=first.len();
    first[f1..].as_ptr() == second.as_ptr()
}

///Returns a combined slice given two slices that are next to each other in memory.
pub fn join_mut<'a,T>(first: &'a mut [T],second:&'a mut [T])->&'a mut[T]{
    let f1=first.len();
    if first[f1..].as_mut_ptr() == second.as_mut_ptr(){
        unsafe{
            return std::slice::from_raw_parts_mut(first.as_mut_ptr(),f1+second.len());
        }
    }else{
        panic!("Slices are not next to each other in memory.");
    }

}

///Returns a combined slice given two slices that are next to each other in memory.
pub fn join<'a,T>(first: &'a [T],second:&'a [T])->&'a [T]{
    let f1=first.len();
    if first[f1..].as_ptr() == second.as_ptr(){
        unsafe{
            return std::slice::from_raw_parts(first.as_ptr(),f1+second.len());
        }
    }else{
        panic!("Slices are not next to each other in memory.");
    }

}



pub mod log{

    use std::fs::File;
    use std::io::Write;
    use super::*;


    pub struct Logger{
        file:File,
        counter:usize
    }
    impl Logger{

        pub fn new(str:&'static str)->Logger{
            
            let file = File::create(str).unwrap();
           
            /*
            write!(file,"Iteration,").unwrap();
            for k in names{
                write!(file,"{},",k).unwrap();    
            }
            
            writeln!(file,"").unwrap();
            */
            Logger{file:file,counter:0}
        }

        pub fn with_names(str:&'static str,names:&[&'static str])->Logger{
            
            let mut file = File::create(str).unwrap();
           
            write!(file,"Iteration,").unwrap();
            for k in names{
                write!(file,"{},",k).unwrap();    
            }
            writeln!(file,"").unwrap();
            Logger{file:file,counter:0}
        }

        pub fn write_str(&mut self,strf:&'static str,slice:&[String]){

            write!(self.file,"{},",strf).unwrap();
            for k in slice{
                write!(self.file,"{},",k).unwrap();    
            }
            writeln!(self.file,"").unwrap();
            
        }
        pub fn write_data(&mut self,slice:&[f64]){
            
            write!(self.file,"{},",self.counter).unwrap();
            for k in slice{
                write!(self.file,"{},",k).unwrap();    
            }
            writeln!(self.file,"").unwrap();
            self.counter+=1;
            
        }
    }
}






pub struct Timer2{
    a:std::time::Instant
}

impl Timer2{
    pub fn new()->Timer2{
        Timer2{a:Instant::now()}
    }
    pub fn elapsed(&self)->f64{
        let elapsed = self.a.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        sec
    }
}



///Modified quick selct found here:
///To use unsafe{}
///https://github.com/willcrichton/algo-rs
pub mod quickselect{
    use std;
    use rand;
    /// Given a list of N things, returns the K-th largest element.
    ///
    /// Assumes 0 <= K < N.
    pub trait KthLargest<T> {
        fn kth_largest<'a,F>(&self, list: &'a mut [T], k: usize,func:&mut F) -> usize where F: FnMut(&T, &T) -> std::cmp::Ordering;
    }

    /// [Randomized selection](http://en.wikipedia.org/wiki/Quickselect) -- runs in O(n)
    pub struct QuickSelect;

    fn kth_largest_helper<'a, T,F>(list: &'a mut [T], mut left: usize, mut right: usize, k: usize,func:&mut F) -> usize where F: FnMut(&T, &T) -> std::cmp::Ordering{
        if left == right { left }
        else {
            loop {
                let pivot = if right == left { left }
                else { left + rand::random::<usize>() % (right - left) };
                let pivot = partition(list, left, right, pivot,func);

                if k == pivot { return k }
                else if k < pivot {
                    right = pivot - 1;
                } else {
                    left = pivot + 1;
                }
            }
        }
    }

    
    fn partition<T,F>(list: &mut [T], left: usize, right: usize, pivot: usize,func:&mut F) -> usize where F: FnMut(&T, &T) -> std::cmp::Ordering {
        list.swap(pivot, right);
        //swap_unchecked(list,pivot,right);
        let mut store_index = left;
        for i in left..right {
            let bb={
                let (a,b)=unsafe{(list.get_unchecked(i),list.get_unchecked(right))};
                func(a,b)
            };
            if bb==std::cmp::Ordering::Less {
                //list,store_index,i);
                list.swap(store_index, i);
                store_index += 1;
            }
        }
        //swap_unchecked(list,right,store_index);
        list.swap(right, store_index);
        store_index
    }

    impl<T> KthLargest<T> for QuickSelect {
        fn kth_largest<'a,F>(&self, list: &'a mut [T], k: usize,func:&mut F) -> usize where F: FnMut(&T, &T) -> std::cmp::Ordering{
            if k >= list.len() {
                panic!("tried to find {}-largest on {} elements, k is too big", k, list.len())
            } else {
                let right = list.len() - 1;
                kth_largest_helper(list, 0, right, k,func)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
