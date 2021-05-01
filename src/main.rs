#![allow(non_camel_case_types)]
mod klle;
use klle::*;
use core::mem::*;

/* redefined type for testing */
pub type klle_str_t = klle_t<*const str>;


/* nurse is sentinel for test list */
static mut NURSE : MaybeUninit<klle_str_t> = MaybeUninit::uninit();

/**
 * print current node payload
 * @param node  - [in] node to print 
 */ 
pub unsafe fn dbg_print(node : &mut MaybeUninit<klle_str_t>)
{
        let nurse_ptr : *const klle_str_t = node.as_mut_ptr();
        if let Some(str_ref) = (*nurse_ptr).data.as_ref() {
            println!("{}", str_ref);
        }
}

/**
 * init static uninitialized struct
 * @param node  - [out] node to init
 * @param data  - [in] data to init
 */ 
pub unsafe fn klle_str_static_init(node : &mut MaybeUninit<klle_str_t>, data : *const str)
{
    let node_ptr : *mut klle_str_t = node.as_mut_ptr();
    (*node_ptr).init(data);
}

fn main()
{
    println!("Start");

    unsafe {
        klle_str_static_init(&mut NURSE, "nurse!");
        dbg_print(&mut NURSE);
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

