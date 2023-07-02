**Note:** this project is dead. It served as illustration for this [blog post](https://info.varnish-software.com/blog/writing-vmods-in-rust) but we now have [varnish-rs](https://info.varnish-software.com/blog/writing-vmods-in-rust) which is better, easier and more up-to-date, so have a look at that instead.

============
vmod_pupa
============

----------------------
Varnish Pupa Module
----------------------

:Date: 2016-03-31
:Version: 1.0
:Manual section: 3

SYNOPSIS
========

import pupa;

DESCRIPTION
===========

Example Varnish vmod demonstrating how to write an out-of-tree Varnish vmod,
IN RUST!

Stores the number of occurences of strings you push/pull into it.

FUNCTIONS
=========

Have a look at src/vmod_pupa.vcc. 

INSTALLATION
============

The source tree is based on autotools to configure the building, and
does also have the necessary bits in place to do functional unit tests
using the ``varnishtest`` tool.

Building requires the Varnish header files and uses pkg-config to find
the necessary paths.

Pre-requisites::

 sudo apt-get install -y autotools-dev make automake libtool pkg-config libvarnishapi1 libvarnishapi-dev

Usage::

 ./autogen.sh
 ./configure

If you have installed Varnish to a non-standard directory, call
``autogen.sh`` and ``configure`` with ``PKG_CONFIG_PATH`` pointing to
the appropriate path. For example, when varnishd configure was called
with ``--prefix=$PREFIX``, use

 PKG_CONFIG_PATH=${PREFIX}/lib/pkgconfig
 export PKG_CONFIG_PATH

Make targets:

* make - builds the vmod.
* make install - installs your vmod.
* make check - runs the unit tests in ``src/tests/*.vtc``
* make distcheck - run check and prepare a tarball of the vmod.

Installation directories
------------------------

By default, the vmod ``configure`` script installs the built vmod in
the same directory as Varnish, determined via ``pkg-config(1)``. The
vmod installation directory can be overridden by passing the
``VMOD_DIR`` variable to ``configure``.

Other files like man-pages and documentation are installed in the
locations determined by ``configure``, which inherits its default
``--prefix`` setting from Varnish.

COMMON PROBLEMS
===============

* configure: error: Need varnish.m4 -- see README.rst

  Check if ``PKG_CONFIG_PATH`` has been set correctly before calling
  ``autogen.sh`` and ``configure``

* Incompatibilities with different Varnish Cache versions

  Make sure you build this vmod against its correspondent Varnish Cache version.
  For example, to build against Varnish Cache 4.0, this vmod must be built from branch 4.0.
