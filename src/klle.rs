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

/* arch-part */
type b8 = bool;

/**
 * kernel linked list element
 */
pub struct klle<T> {
	next : *mut klle<T>,  		/* next member pointer */
	prev : *mut klle<T>,		/* next member pointer */
	data : T,	            /* data */
}
pub type klle_t<T> = klle<T>;

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
        let rself = self as *mut klle_t<T>;

        (*rself).next = rself;
	    (*rself).prev = rself;
	    (*rself).data = dts;
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
        let rself = self as *mut klle_t<T>;
        let result : b8 = (*rself).next == rself;
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
        let rself = self as *mut klle_t<T>;
	    let old_head : *mut klle_t<T> = (*rself).next;

	    (*new_head).next = old_head;
	    (*new_head).prev = rself;
	    (*old_head).prev = new_head;
	    (*rself).next = new_head;
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
        let rself = self as *mut klle_t<T>;
	    let old_tail : *mut klle_t<T> = (*rself).prev;

	    (*new_tail).next = rself;
	    (*new_tail).prev = old_tail;
	    (*old_tail).next = new_tail;
	    (*rself).prev = new_tail;
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
        let rself = self as *mut klle_t<T>;
	    let old_head : *mut klle_t<T> = (*rself).next;

	    (*rself).next = (*old_head).next;
	    (*(*old_head).next).prev = rself;

	    return old_head;
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
        let rself = self as *mut klle_t<T>;
	    let old_tail : *mut klle_t<T> = (*rself).prev;

	    (*rself).prev = (*old_tail).prev;
	    (*(*old_tail).prev).next = rself;

	    return old_tail;
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
        let rself = self as *mut klle_t<T>;
	    (*(*rself).prev).next = (*rself).next;
	    (*(*rself).next).prev = (*rself).prev;
    }

}

