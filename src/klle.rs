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
pub struct klle_<T> {
	next : *mut klle_<T>,  		/* next member pointer */
	prev : *mut klle_<T>,		/* next member pointer */
	data : T,	            /* data */
}
pub type klle_t<T> = klle_<T>;

/* functions */

/**
 * init linked list elemet
 * @param elem 		- [in] linked list element to init
 * @param dts		- [in] data to set
 */
#[inline(always)]
pub unsafe fn init<T>(elem : *mut klle_t<T>, dts : T)
{
    (*elem).next = elem;
	(*elem).prev = elem;
	(*elem).data = dts;
}

/**
 * check linked list nurse
 * false else
 * @param nurse 	- [in] linked list nurse
 * @return 	        - true if empty
 */
#[inline(always)]
pub unsafe fn is_empty<T>(nurse : *mut klle_t<T>)->b8
{
    let result : b8 = (*nurse).next == nurse;
    return result;
}

/**
 * add to linked list head
 * @param nurse 	- [inout] linked list nurse
 * @param data 		- [in] data to add
 */
#[inline(always)]
pub unsafe fn addh<T>(nurse : *mut klle_t<T>, new_head : *mut klle_t<T>)
{
	/*
	 * circullar linked list have rules:
	 * nurse next is head
	 * prev is move to head
	 *
	 * nurse prev is tail
	 * next is move to tail
	 */

	/************************************************/
	/*		        new_head  			*/
	/* nurse.next --/   ||	 \- old_head.prev 	*/
	/*	         /	    \/	   \			*/
	/*	nurse --->      ---> old_head		*/
	/************************************************/
	let old_head : *mut klle_t<T> = (*nurse).next;

	(*new_head).next = old_head;
	(*new_head).prev = nurse;
	(*old_head).prev = new_head;
	(*nurse).next = new_head;
}

/**
 * add to linked list tail
 * @param nurse 	- [inout] linked list nurse
 * @param data 		- [inout] data to add
 */
#[inline(always)]
pub unsafe fn addt<T>(nurse : *mut klle_t<T>, new_tail : *mut klle_t<T>)
{
	/*
	 * circullar linked list have rules:
	 * nurse next is head
	 * prev is move to head
	 *
	 * nurse prev is tail
	 * next is move to tail
	 */

	/************************************************/
	/*		 new_tail  			                    */
	/* old_tail.next/   ||	 \ nurse.prev		    */
	/*	     /	    \/	   \			            */
	/*    old_tail --->      ---> nurse		        */
	/************************************************/
	let old_tail : *mut klle_t<T> = (*nurse).prev;

	(*new_tail).next = nurse;
	(*new_tail).prev = old_tail;
	(*old_tail).next = new_tail;
	(*nurse).prev = new_tail;
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
pub unsafe fn geth<T>(nurse : *mut klle_t<T>)->*mut klle_t<T>
{
	/*
	 * circullar linked list have rules:
	 * nurse next is head
	 * prev is move to head
	 *
	 * nurse prev is tail
	 * next is move to tail
	 */

	/************************************************/
	/*	nurse --->  ||    ---> new_head		        */
	/*	 \  \next---||----------/   /		        */
	/*	  \---------||---------prev/		        */
	/*              ||				                */
	/*	     	    \/				                */
	/*      prev <--- old_head ---> next		    */
	/************************************************/
	let old_head : *mut klle_t<T> = (*nurse).next;

	(*nurse).next = (*old_head).next;
	(*(*old_head).next).prev = nurse;

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
pub unsafe fn gett<T>(nurse : *mut klle_t<T>)->*mut klle_t<T>
{
	/*
	 * circullar linked list have rules:
	 * nurse next is head
	 * prev is move to head
	 *
	 * nurse prev is tail
	 * next is move to tail
	 */

	/************************************************/
	/*	new_tail --->  ||    ---> nurse		        */
	/*	 \  \prev---||----------/   /		        */
	/*	  \---------||---------next/		        */
	/*              ||				                */
	/*	     	    \/				                */
	/*      prev <--- old_tail ---> next		    */
	/************************************************/
	let old_tail : *mut klle_t<T> = (*nurse).prev;

	(*nurse).prev = (*old_tail).prev;
	(*(*old_tail).prev).next = nurse;

	return old_tail;
}

/**
 * del entry from list, no cheking
 * do not apply to nurse, it's will break the list
 * @param del 		- [inout] data to del
 */
#[inline(always)]
pub unsafe fn del<T>(del: *mut klle_t<T>)
{
	/*
	 * circullar linked list have rules:
	 * nurse next is head
	 * prev is move to head
	 *
	 * nurse prev is tail
	 * next is move to tail
	 */

	/************************************************/
	/*	del.prev --->       ---> del.next	        */
	/*	 \  \next------||----------/   /	        */
	/*	  \------------||---------prev/		        */
	/*                 ||       			        */
	/*	     	       \/		        	        */
	/*  		       del 			                */
	/************************************************/
	(*(*del).prev).next = (*del).next;
	(*(*del).next).prev = (*del).prev;
}

