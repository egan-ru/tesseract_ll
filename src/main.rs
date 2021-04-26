#![allow(non_camel_case_types)]

use core::mem::MaybeUninit;
mod klle;

/*
#[allow(dead_code)]
struct klle_test {
	next : *mut klle_test,      /* next member pointer */
	prev : *mut klle_test,      /* next member pointer */
	data : *mut u8,             /* data pointer */
}
*/

/* type klle_test_t = klle_test; */

/*
const nurse_txt : u8[] = "!nurse";
const m0_txt : u8[] = "0";
const m1_txt : u8[] = "1";
const m2_txt : u8[] = "2";
const m3_txt : u8[] = "3";
const m4_txt : u8[] = "4";
const m5_txt : u8[] = "5";
const m6_txt : u8[] = "6";
const m7_txt : u8[] = "7";
const m8_txt : u8[] = "8";
const m9_txt : u8[] = "9";
*/



/* test elemts, every element is in one pos for easy debugging */
/*
let mut test_nurse : klle_test_t;
let mut test_elem0 : klle_test_t;
let mut test_elem1 : klle_test_t;
let mut test_elem2 : klle_test_t;
let mut test_elem3 : klle_test_t;
let mut test_elem4 : klle_test_t;
let mut test_elem5 : klle_test_t;
let mut test_elem6 : klle_test_t;
let mut test_elem7 : klle_test_t;
let mut test_elem8 : klle_test_t;
let mut test_elem9 : klle_test_t;

*/

static mut nurse : Option<klle::klle_t<&str>> = None;

fn main()
{
    /*
     * let mut test_nurse : klle_test_t;
     * let tnp : *mut klle_test_t = &mut test_nurse;
     * klle::init(tnp, "nurse_txt");
     */

    nurse.init("nurse"); 


    println!("Hello, world!");
    
}

