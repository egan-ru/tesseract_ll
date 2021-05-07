/* 
 * This file is part of the XXX distribution (https://github.com/mentalsupernova/rust_test 
 * Copyright (c) 2021 egan.fryazino, mentalsupernova.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/* linked list with nurse,
 * used in various kernel structs
 */

#![allow(non_camel_case_types)]
//#![no_std]
use core::mem::*;

/* arch-part */
type b8 = bool;

/* kernel linked list element */
pub struct klle<T> {
	pub next : *mut klle<T>,  		/* next member pointer */
	pub prev : *mut klle<T>,		/* next member pointer */
	pub data : T,	                /* data */
}

/* kernel linked list element,
 * with possibilily usage with
 * statically allocated mem
 */
pub struct klleu<T> {
    pub ll: MaybeUninit<klle<T>>,   /* unused wrapper for klle */
}

pub type klle_t<T> = klleu<T>;

/* functions */


/* linked with resticted functionality implementation */
impl <T> klle_t<T> {

    /**
     * init linked list elemet
     * @param elem 		- [in] linked list element to init
     * @param dts		- [in] data to set
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn init(&mut self, dts : T)
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();

        (*ll).next = ll;
	    (*ll).prev = ll;
	    (*ll).data = dts;
    }

    /**
     * check linked list nurse
     * false else
     * @param nurse 	- [in] linked list nurse
     * @return 	        - true if empty
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn is_empty(&mut self)->b8
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();

        let result : b8 = (*ll).next == ll;
        return result;
    }

    /**
     * add to linked list head
     * @param nurse 	- [inout] linked list nurse
     * @param data 		- [in] data to add
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn addh(&mut self, new_head : *mut klle_t<T>)
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
        let nhead : *mut klle<T> = (*new_head).ll.as_mut_ptr();
	    let ohead : *mut klle<T> = (*ll).next;

	    (*nhead).next = ohead;
	    (*nhead).prev = ll;
	    (*ohead).prev = nhead;
	    (*ll).next = nhead;
    }

    /**
     * add to linked list tail
     * @param nurse 	- [inout] linked list nurse
     * @param data 		- [inout] data to add
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn addt(&mut self, new_tail : *mut klle_t<T>)
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
        let ntail : *mut klle<T> = (*new_tail).ll.as_mut_ptr();
	    let otail : *mut klle<T> = (*ll).prev;

	    (*ntail).next = ll;
	    (*ntail).prev = otail;
	    (*otail).next = ntail;
	    (*ll).prev = ntail;
    }

    /**
     * get data from linked list head,
     * head is flushed after
     *
     * @param llist		- [inout] list to use
     * @return		    - linked list head,
     * 			          or nurse if linked list is empty
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn geth(&mut self)->*mut klle_t<T>
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
	    let ohead : *mut klle<T> = (*ll).next;

	    (*ll).next = (*ohead).next;
	    (*(*ohead).next).prev = ll;

	    return ohead as *mut klle_t<T>;
    }

    /**
     * get data from linked list tail,
     * tail is flushed after
     *
     * @param llist		- [inout] list to use
     * @return		    - linked list tail,
     * 			          or nurse if linked list is empty
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn gett(&mut self)->*mut klle_t<T>
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
	    let otail : *mut klle<T> = (*ll).prev;

	    (*ll).prev = (*otail).prev;
	    (*(*otail).prev).next = ll;

	    return otail as *mut klle_t<T>;
    }

    /**
     * del entry from list, no cheking
     * do not apply to nurse, it's will break the list
     * @param del 		- [inout] data to del
     */
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn del(&mut self)
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();

	    (*(*ll).prev).next = (*ll).next;
	    (*(*ll).next).prev = (*ll).prev;
    }

    /**
     * create list entry in uninitialized state
     * @return      - uninitialized list entry
     */ 
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn uninit()->klle_t<T>
    {
        return klle_t{ll: MaybeUninit::uninit()};
    }

    /**
     * return pointer to next linked list node
     * @return      - next linked list node
     */ 
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn next(&mut self)->*mut klle_t<T>
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
        let next : *mut klle<T> = (*ll).next;
        return next as *mut klle_t<T>;
    }

    /**
     * return pointer to prev linked list node
     * @return      - prev linked list node
     */ 
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn prev(&mut self)->*mut klle_t<T>
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
        let prev : *mut klle<T> = (*ll).prev;
        return prev as *mut klle_t<T>;
    }

    /**
     * return linked list data
     * @return      - linked list data
     */ 
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn data(&mut self)->&mut T
    {
        let ll : *mut klle<T> = (*self).ll.as_mut_ptr();
        let datap : &mut T = &mut(*ll).data;
        return datap;
    }
}

