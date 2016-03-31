AM_CPPFLAGS = @VMOD_INCLUDES@ -Wall -Werror

vmoddir = @VMOD_DIR@
vmod_LTLIBRARIES = libvmod_pupa.la

libvmod_pupa_la_LDFLAGS = -Wc,../target/debug/libpupa.a -module -export-dynamic -avoid-version -shared

../target/debug/libpupa.a: pupa.rs
	cargo build

libvmod_pupa_la_LIBADD = \
	../target/debug/libpupa.a

nodist_libvmod_pupa_la_SOURCES = \
	vcc_if.c \
	vcc_if.h

vmod_pupa.lo: vcc_if.c vcc_if.h

vcc_if.c: vcc_if.h

vcc_if.h: @VMODTOOL@ $(top_srcdir)/src/vmod_pupa.vcc
	@VMODTOOL@ $(top_srcdir)/src/vmod_pupa.vcc

VMOD_TESTS = $(top_srcdir)/src/tests/*.vtc
.PHONY: $(VMOD_TESTS)

$(top_srcdir)/src/tests/*.vtc: libvmod_pupa.la
	@VARNISHTEST@ -Dvarnishd=@VARNISHD@ -Dvmod_topbuild=$(abs_top_builddir) $@

check: $(VMOD_TESTS)

EXTRA_DIST = \
	vmod_pupa.vcc \
	$(VMOD_TESTS)

CLEANFILES = \
	$(builddir)/vcc_if.c \
	$(builddir)/vcc_if.h \
	$(builddir)/vmod_pupa.rst \
	$(builddir)/vmod_pupa.man.rst
