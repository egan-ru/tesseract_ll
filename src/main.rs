#![allow(non_camel_case_types)]
mod klle;
use klle::*;
use core::mem::*;

/* redefined type for testing */
pub type klle_str_t = klle_t<*const str>;


/* nurse is sentinel for test list */
static mut NURSE : MaybeUninit<klle_str_t> = MaybeUninit::uninit();



//static mut nurse : klle_str_t = MaybeUninit::<klle_str_t>::uninit();

fn main()
{
    /*
     * let mut test_nurse : klle_test_t;
     * let tnp : *mut klle_test_t = &mut test_nurse;
     * klle::init(tnp, "nurse_txt");
     */
    unsafe {
        let nurse_ptr : *mut klle_str_t = NURSE.as_mut_ptr();
        (*nurse_ptr).init("nurse!"); 
    }

    unsafe {
        let nurse_ptr : *mut klle_str_t = NURSE.as_mut_ptr();
        println!("{:?}",(*nurse_ptr).data);
    }

    println!("Hello, world!");
    
}

