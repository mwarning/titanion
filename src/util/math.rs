/*
 * $Id: math.d,v 1.1.1.1 2006/11/19 07:54:55 kenta Exp $
 *
 * Copyright 2006 Kenta Cho. Some rights reserved.
 */
//module src.util.math;

//private import std.math;
//static PI : f32 = std::f32::consts::PI;
use std::f32::consts::PI;

/**
 * Math utility methods.
 */

pub fn normalize_deg(d : f32) -> f32 {
	let mut rd : f32 = d;
	if rd < -PI {
	  rd = PI * 2.0 - (-rd % (PI * 2.0));
	}

	(rd + PI) % (PI * 2.0) - PI
}

pub fn normalize_deg_360(d : f32) -> f32 {
	let rd : f32 = d;
	if rd < -180.0 {
	 	360.0 - (((-rd as i32) % 360) as f32)
	} else {
		(((rd + 180.0) as i32) % 360) as f32 - 180.0
	}
}

