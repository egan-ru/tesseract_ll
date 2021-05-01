#![allow(non_camel_case_types)]
mod klle;
use klle::*;
use core::mem::*;

/* redefined type for testing */
pub type klle_str_t = klle_t<*const str>;


/* nurse is sentinel for test list */
static mut NURSE : MaybeUninit<klle_str_t> = MaybeUninit::uninit();
static mut THE_0 : MaybeUninit<klle_str_t> = MaybeUninit::uninit();
static mut THE_1 : MaybeUninit<klle_str_t> = MaybeUninit::uninit();
static mut THE_2 : MaybeUninit<klle_str_t> = MaybeUninit::uninit();
static mut THE_3 : MaybeUninit<klle_str_t> = MaybeUninit::uninit();
static mut THE_4 : MaybeUninit<klle_str_t> = MaybeUninit::uninit();


/**
 * print current node payload
 * @param node  - [in] node to print 
 */ 
pub unsafe fn dbg_print(node : &mut MaybeUninit<klle_str_t>)
{
        let nurse_ptr : *const klle_str_t = node.as_mut_ptr();
        let str_ref = &*(*nurse_ptr).data;
        println!("{}", str_ref);
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


/**
 * init static uninitialized struct
 * @param node  - [out] node to init
 * @param data  - [in] data to init
 */ 
pub unsafe fn klle_str_static_addh(node : &mut MaybeUninit<klle_str_t>, toadd : &mut MaybeUninit<klle_str_t>)
{
    let node_ptr : *mut klle_str_t = node.as_mut_ptr();
    let toadd_ptr : *mut klle_str_t = toadd.as_mut_ptr();
    (*node_ptr).addh(toadd_ptr);
}

fn main()
{
    println!("Start");

    unsafe {
        /* init variables */
        klle_str_static_init(&mut NURSE, "nurse!");
        klle_str_static_init(&mut THE_0, "the_0");
        klle_str_static_init(&mut THE_1, "the_1");
        klle_str_static_init(&mut THE_2, "the_2");
        klle_str_static_init(&mut THE_3, "the_3");
        klle_str_static_init(&mut THE_4, "the_4");

        /* debug print */
        dbg_print(&mut NURSE);

        /* add all to the tail */

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

