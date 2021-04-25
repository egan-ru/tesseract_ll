#![allow(non_camel_case_types)]

mod klle;

/* arch-part */
#[allow(dead_code)]
type b8 = bool;

#[allow(dead_code)]
struct klle_test {
	data : *mut u8,             /* data pointer */
	next : *mut klle_test,      /* next member pointer */
	prev : *mut klle_test,      /* next member pointer */
}

type klle_test_t = klle_test;
/*
let nurse_txt : [u8; 16] = "!nurse";
let m0_txt : u8[] = "0";
let m1_txt : u8[] = "1";
let m2_txt : u8[] = "2";
let m3_txt : u8[] = "3";
let m4_txt : u8[] = "4";
let m5_txt : u8[] = "5";
let m6_txt : u8[] = "6";
let m7_txt : u8[] = "7";
let m8_txt : u8[] = "8";
let m9_txt : u8[] = "9";

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
fn main()
{
    let mut test_nurse : klle_test_t;
    klle::init(&mut test_nurse, "nurse_txt");


    println!("Hello, world!");
    
}
