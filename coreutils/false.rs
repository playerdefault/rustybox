use libc;
/* vi: set sw=4 ts=4: */
/*
 * Mini false implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config FALSE
//config:	bool "false (tiny)"
//config:	default y
//config:	help
//config:	false returns an exit code of FALSE (1).
//applet:IF_FALSE(APPLET_NOFORK(false, false, BB_DIR_BIN, BB_SUID_DROP, false))
//kbuild:lib-$(CONFIG_FALSE) += false.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/000095399/utilities/false.html */
/* "false --help" is special-cased to ignore --help */
//usage:#define false_trivial_usage NOUSAGE_STR
//usage:#define false_full_usage ""
//usage:#define false_example_usage
//usage:       "$ false\n"
//usage:       "$ echo $?\n"
//usage:       "1\n"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn false_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return 1i32;
}
