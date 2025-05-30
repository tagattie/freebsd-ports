# vim: ts=8 noexpandtab
#
# Provide support to use Tcl/Tk
#
# Variables set by this file:
#
# TCL_VER		- Major.Minor version of Tcl
#
# TCLSH			- Set to full path of the Tcl interpreter
#
# TCL_LIBDIR		- Path where the Tcl libraries can be found
#
# TCL_INCLUDEDIR	- Path where the Tcl C headers can be found
#
# TCL_PKG_LIB_PREFIX    - Library prefix, as per TIP595. This is tcl9 when
#			  when building against Tcl 9.0, and empty otherwise
#
# TCL_PKG_STUB_POSTFIX  - Stub library postfix. This is empty when building
#			  against Tcl 9.0, and DISTVERSION otherwise.
#			  See https://core.tcl-lang.org/tclconfig/info/381985d331b96ba9
#
#
# TK_VER		- Major.Minor version of Tk
#
# WISH			- Set to full path of the Tk interpreter
#
# TK_LIBDIR		- Path where the Tk libraries can be found
#
# TK_INCLUDEDIR		- Path where the Tk C headers can be found
#
#
#
# Usage:
#
# USES+=	PORT[:(VERSION|wrapper),build,run,tea,test]
#
# where PORT is one of:
#
# - tcl			- Depend on Tcl
# - tk			- Depend on both Tk and Tcl (Tk itself depends on Tcl)
#
# and VERSION is in one of the following formats:
#
# - (empty)		- Depend on any installed version of PORT. If none
#   			  is installed, bring in the default version. See
#   			  ${_TCLTK_DEFAULT_VERSION} below.
#
# - 86, 90		- Depend on a specific version series of PORT. Multiple
#   			  values are OK. The highest version available is
#   			  picked.
#
# - 86+ 		- Depend on any installed version greater or equal to
#   			  the specified version.
#
# If wrapper is specified, an additional dependency on tcl-wrapper or
# tk-wrapper is added. It is NOT possible to select a specific version of
# Tcl/Tk when using the wrapper.
#
# Build-time / Run-time / Test-time only dependencies can be specified with
# build, run or test.
#
# Tea can be used for Tcl/Tk extensions that use the Tcl Extension Architecture
# [http://www.tcl.tk/doc/tea] and allows to set common autoconf parameters.
#
# MAINTAINER: tcltk@FreeBSD.org

.if ${USES:Mtk} || ${USES:Mtk\:*}
.  if !defined(_TCLTK_PORT)
_TCLTK_IGNORE=	yes
.  endif
.endif

.if !defined(_INCLUDE_USES_TCL_MK) && !defined(_TCLTK_IGNORE)
_INCLUDE_USES_TCL_MK=	yes

#
# List the currently available versions.
#
# When adding a version, please keep the comment in
# Mk/bsd.default-versions.mk in sync.
_TCLTK_VALID_VERSIONS=	86 90

#
# Bring in the default and check that the specified version is in the list of
# valid versions.
#
_TCLTK_DEFAULT_VERSION=	${TCLTK_DEFAULT:S/.//}
.  if ! ${_TCLTK_VALID_VERSIONS:M${_TCLTK_DEFAULT_VERSION}}
IGNORE=	Invalid tcltk version ${TCLTK_DEFAULT}
.  endif

#
# _TCLTK_PORT tells us whether we're depending on Tcl or Tk. When using
# USES+=tk, the included file tk.mk sets this before including this file.
#
_TCLTK_PORT?=	tcl

#
# Parse a ver+ argument.
#
.  if ${tcl_ARGS:M*+}
_TCLTK_MIN_VERSION:=	${tcl_ARGS:M*+:S/+//}
_TCLTK_WANTED_VERSIONS:=${_TCLTK_DEFAULT_VERSION}
.  endif

#
# Parse one or more ver arguments.
#
.  if ${tcl_ARGS:M86}
_TCLTK_WANTED_VERSIONS:=${tcl_ARGS:M86}
.  endif
.  if ${tcl_ARGS:M90}
_TCLTK_WANTED_VERSIONS:=${tcl_ARGS:M90}
.  endif

#
# It makes little sense to specify both the wrapper and a specific version.
#
.  if ${tcl_ARGS:Mwrapper} && defined(_TCLTK_WANTED_VERSIONS)
IGNORE=	USES=${_TCLTK_PORT}: it is not possible to specify both a version and the wrapper: ${tcl_ARGS}
.  endif

#
# If no version was specified with any of the ver or ver+ arguments, set the
# default version.
#
.  if !defined(_TCLTK_WANTED_VERSIONS)
_TCLTK_WANTED_VERSIONS=	${_TCLTK_DEFAULT_VERSION}
.  endif

# 
# Resolve minimum versions (ver+). Append anything greater or equal than the
# specified minimum version to the list of wanted versions.
#
.  if defined(_TCLTK_MIN_VERSION)
.    for _v in ${_TCLTK_VALID_VERSIONS}
.      if ${_TCLTK_MIN_VERSION} <= ${_v}
_TCLTK_WANTED_VERSIONS+=${_v}
.      endif
.    endfor
.  endif

#
# Right now we have built a list of potential versions that we may depend on.
# Let's sort them and remove any duplicates. We then locate the highest one
# already installed, if any.
#
.  for _v in ${_TCLTK_WANTED_VERSIONS:O:u}
_TCLTK_HIGHEST_VERSION:=${_v}
.    if exists(${LOCALBASE}/lib/lib${_TCLTK_PORT}${_v}.so)
_TCLTK_WANTED_VERSION:=	${_v}
.    endif
.  endfor

#
# If we couldn't find any wanted version installed, depend on the default or the highest one.
.  if !defined(_TCLTK_WANTED_VERSION)
.    if ${_TCLTK_WANTED_VERSIONS:M${_TCLTK_DEFAULT_VERSION}}
_TCLTK_WANTED_VERSION:=	${_TCLTK_DEFAULT_VERSION}
.    else
_TCLTK_WANTED_VERSION:= ${_TCLTK_HIGHEST_VERSION}
.    endif
.  endif

#
# Exported variables
#
TCL_VER:=	${_TCLTK_WANTED_VERSION:S/8/8./:S/9/9./}
TCL_SHLIB_VER:=	${_TCLTK_WANTED_VERSION}
TCLSH:=		${LOCALBASE}/bin/tclsh${TCL_VER}
TCL_LIBDIR:=	${LOCALBASE}/lib/tcl${TCL_VER}
TCL_INCLUDEDIR:=${LOCALBASE}/include/tcl${TCL_VER}

.  if ${_TCLTK_PORT} == "tk"
TK_VER:=	${_TCLTK_WANTED_VERSION:S/8/8./:S/9/9./}
TK_SHLIB_VER:=	${_TCLTK_WANTED_VERSION}
WISH:=		${LOCALBASE}/bin/wish${TCL_VER}
TK_LIBDIR:=	${LOCALBASE}/lib/tk${TK_VER}
TK_INCLUDEDIR:=	${LOCALBASE}/include/tk${TK_VER}
.  endif

#
# Dependencies
#
_TCLTK_BUILD_DEPENDS=
_TCLTK_RUN_DEPENDS=
_TCLTK_LIB_DEPENDS=

# Construct the correct dependency lines (wrapper)
.  if ${tcl_ARGS:Mwrapper}
.    if ${_TCLTK_PORT} == "tcl"
_TCLTK_WRAPPER_PORT=	tclsh:lang/tcl-wrapper
.    elif ${_TCLTK_PORT} == "tk"
_TCLTK_WRAPPER_PORT=	wish:x11-toolkits/tk-wrapper
.    endif
.  endif

# Construct the correct dependency lines (Tcl/Tk)
.  if ${_TCLTK_PORT} == "tcl"
_TCLTK_EXE_LINE=	tclsh${TCL_VER}:lang/tcl${_TCLTK_WANTED_VERSION}
_TCLTK_LIB_LINE=	libtcl${TCL_SHLIB_VER}.so:lang/tcl${_TCLTK_WANTED_VERSION}
.  elif ${_TCLTK_PORT} == "tk"
_TCLTK_EXE_LINE=	wish${TK_VER}:x11-toolkits/tk${_TCLTK_WANTED_VERSION}
_TCLTK_LIB_LINE=	libtk${TK_SHLIB_VER}.so:x11-toolkits/tk${_TCLTK_WANTED_VERSION} \
			libtcl${TCL_SHLIB_VER}.so:lang/tcl${_TCLTK_WANTED_VERSION}
.  endif

.  if ${tcl_ARGS:Mbuild}
BUILD_DEPENDS+=	${_TCLTK_WRAPPER_PORT} \
		${_TCLTK_EXE_LINE}
.  elif ${tcl_ARGS:Mrun}
RUN_DEPENDS+=	${_TCLTK_WRAPPER_PORT} \
		${_TCLTK_EXE_LINE}
.  elif ${tcl_ARGS:Mtest}
TEST_DEPENDS+=	${_TCLTK_WRAPPER_PORT} \
		${_TCLTK_EXE_LINE}
.  else
RUN_DEPENDS+=	${_TCLTK_WRAPPER_PORT}
LIB_DEPENDS+=	${_TCLTK_LIB_LINE}
.  endif

# Setup TEA stuff
.  if ${tcl_ARGS:Mtea}
GNU_CONFIGURE=	yes
TCL_PKG?=	${PORTNAME:C/^tcl(-?)//:C/(-?)tcl\$//}${PORTVERSION}
.    if ${TCL_VER} == "9.0"
TCL_PKG_LIB_PREFIX=	tcl9
TCL_PKG_STUB_POSTFIX=	
.    else
TCL_PKG_LIB_PREFIX=	
TCL_PKG_STUB_POSTFIX=	${DISTVERSION}
.    endif
PLIST_SUB+=	TCL_PKG=${TCL_PKG} TCL_PKG_LIB_PREFIX=${TCL_PKG_LIB_PREFIX} \
		TCL_PKG_STUB_POSTFIX=${TCL_PKG_STUB_POSTFIX}
CONFIGURE_ARGS+=--exec-prefix=${PREFIX} \
		--with-tcl=${TCL_LIBDIR} \
		--with-tclinclude=${TCL_INCLUDEDIR}
.    if ${_TCLTK_PORT} == "tk"
CONFIGURE_ARGS+=--with-tk=${TK_LIBDIR} --with-tkinclude=${TK_INCLUDEDIR}
.    endif
.  endif

.endif # defined(_INCLUDE_USES_TCL_MK)

.undef _TCLTK_IGNORE
