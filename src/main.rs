#![allow(non_camel_case_types)]
mod klle;
use klle::*;
use core::mem::*;

/* redefined type for testing */
pub type klle_str_t = klle_t<*const str>;


/* nurse is sentinel for test list */
static mut NURSE : klle_str_t = klle_str_t::uninit();

fn main()
{
    println!("Start");

    unsafe {
        /* init variables */
        NURSE.init("nurse!");

        /* debug print */

    }

    //unsafe {
    //    let nurse_ptr : *mut klle_str_t = NURSE.as_mut_ptr();
    //    (*nurse_ptr).init("nurse!"); 
    //}

    //unsafe {
    //    let nurse_ptr : *mut klle_str_t = NURSE.as_mut_ptr();
    //    println!("{}",((*nurse_ptr).data));
    //    println!("{}",(*(*nurse_ptr).next).data);
    //}

    println!("Done");
    
}

