#![allow(non_camel_case_types)]
mod klle;
use klle::*;
use core::mem::*;

/* redefined type for testing */
pub type klle_str_t = klle_t<*const str>;


/* nurse is sentinel for test list */
static mut NURSE : klle_str_t = klle_str_t::uninit();
static mut NUM_0 : klle_str_t = klle_str_t::uninit();
static mut NUM_1 : klle_str_t = klle_str_t::uninit();
static mut NUM_2 : klle_str_t = klle_str_t::uninit();
static mut NUM_3 : klle_str_t = klle_str_t::uninit();

/**
 * print node string
 * (for debugging purposes)
 * @param node      - [in] node to print info
 */
unsafe fn print_node(node : &mut klle_str_t)
{
    let data : *const str = *(*node).data();
    let pdata : &str = &*data;
    println!("{}", pdata);
}

fn main()
{
    println!("Start");

    unsafe {
        /* init variables */
        NURSE.init("nurse!");
        NUM_0.init("num_0");
        NUM_1.init("num_1");
        NUM_2.init("num_2");
        NUM_3.init("num_3");

        /* add to head */
        NURSE.addh(&mut NUM_0);
        NURSE.addh(&mut NUM_1);
        NURSE.addh(&mut NUM_2);
        NURSE.addh(&mut NUM_3);

        /* debug print */
        print_node(&mut*NURSE.next());
        print_node(&mut*NURSE.prev());
        //let pdata : &str = &*(*NURSE.data());
        //println!("{}", pdata);
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

