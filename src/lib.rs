extern crate smallvec;






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


///Returns a combined slice given two slices that are next to each other in memory.
///Panics if they are not next to each other.
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
///Panics if they are not next to each other.
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
    


    pub struct Logger{
        file:File,
        counter:usize
    }
    impl Logger{

        pub fn new(str:&'static str)->Logger{
            
            let file = File::create(str).unwrap();
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

        pub fn write_data<I:Iterator<Item=f64>>(&mut self,stuff:I){
            
            write!(self.file,"{},",self.counter).unwrap();
            for k in stuff{
                write!(self.file,"{},",k).unwrap();    
            }
            writeln!(self.file,"").unwrap();
            self.counter+=1;
            
        }
    }
}





use std::time::Instant;

pub struct Timer2{
    a:std::time::Instant
}

impl Timer2{
    pub fn new()->Timer2{
        Timer2{a:Instant::now()}
    }

    ///Returns the time since this object was created in seconds.
    pub fn elapsed(&self)->f64{
        let elapsed = self.a.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        sec
    }
}


