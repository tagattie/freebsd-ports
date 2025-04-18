# This Makefile was copied from lang/python27 and then gradually
# adapted/cleaned-up for Tauthon.
# As for lang/python27, some core modules with extraneous dependencies are
# disabled, and should be made available in other ports if the need arises (see
# 'pkg-message').
# -- Olivier Certner <olce.freebsd.ports@certner.fr>
PORTNAME=	tauthon
DISTVERSIONPREFIX=	v
DISTVERSION=	2.8.5-20240706
CATEGORIES=	lang python

MAINTAINER=	olce.freebsd.ports@certner.fr
COMMENT=	Backwards-compatible fork of CPython 2.7 with Python 3.x features
WWW=		https://github.com/naftaliharris/tauthon

LICENSE=	PSFL

DEPRECATED=	Uses Python 2.7 codebase

USES=		pathfix shebangfix autoreconf ncurses pkgconfig readline ssl \
		tar:xz compiler
PATHFIX_MAKEFILEIN=	Makefile.pre.in

USE_GITHUB=	yes
GH_ACCOUNT=	naftaliharris
GH_TAGNAME=	b787044f7c9a3728604471119e6d6220d09d6d94

USE_LDCONFIG=	yes
GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=${PREFIX}/share
# Piggyback on Python support in shebangfix
python_CMD=	${PREFIX}/bin/${NAME_VERSION}
# Better be safe than sorry
.for _DIRS in Demo/cgi Demo/comparisons Demo/curses Demo/parser Demo/pdist \
	Demo/pysvr Demo/scripts Demo/sockets Demo/tkinter/guido Demo/turtle \
	Demo/zlib Doc/includes Doc/tools Lib Lib/encodings Lib/ensurepip \
	Lib/idlelib Lib/lib2to3/pgen2 Lib/lib2to3/tests \
	Lib/lib2to3/tests/data Lib/plat-mac Lib/test Lib/test/crashers \
	Mac/BuildScript Mac/scripts Mac/Tools Modules/_ctypes/libffi \
	Parser PCbuild Python Tools/compiler Tools/faqwiz Tools/freeze \
	Tools/gdb Tools/i18n Tools/nuget Tools/pybench Tools/scripts \
	Tools/ssl Tools/unicode Tools/webchecker
SHEBANG_FILES+=	${_DIRS}/*.py
.endfor

CONFIGURE_ARGS+=	--enable-shared
CONFIGURE_ENV+=		ac_cv_opt_olimit_ok=no OPT=""				# Null out OPT to respect user CFLAGS and remove optimizations

INSTALL_TARGET=		altinstall						# Don't want cloberring of unprefixed files

TEST_TARGET=		buildbottest
TEST_ARGS=		TESTOPTS=-j${MAKE_JOBS_NUMBER}

MAKE_ARGS+=		INSTALL_SHARED="${INSTALL_LIB}"				# Strip shared library

PLIST_SUB=		ABI=${ABIFLAGS} \
			DISTVERSION=${DISTVERSION} \
			TAUTHON_VERSION=${TAUTHON_VERSION} \
			VERSION_LONG=${VERSION_LONG} \
			VERSION_NOMICRO=${VERSION_NOMICRO} \
			OSMAJOR=${OSVERSION:C/([0-9]*)[0-9]{5}/\1/}		# For plat-freebsd* in pkg-plist. https://bugs.python.org/issue19554

OPTIONS_DEFINE=		DEBUG IPV6 LIBFFI LTO NLS OPTIMIZATIONS PYMALLOC THREADS
OPTIONS_DEFAULT=	LIBFFI PYMALLOC THREADS UCS4
OPTIONS_SINGLE=		UNICODE
OPTIONS_SINGLE_UNICODE=	UCS2 UCS4
OPTIONS_SUB=		yes

LIBFFI_DESC=	Use libffi from ports instead of bundled version
LTO_DESC=	Apply Link-Time Optimizations (implies OPTIMIZATIONS)
NLS_DESC=	Enable gettext support for the locale module
OPTIMIZATIONS_DESC=	Enable code optimizations (notably PGO)
UCS2_DESC=	Enable UCS2 Unicode Strings
UCS4_DESC=	Enable UCS4 Unicode Strings
PYMALLOC_DESC=	Enable specialized mallocs

DEBUG_CONFIGURE_WITH=	pydebug
IPV6_CONFIGURE_ENABLE=	ipv6
LIBFFI_CONFIGURE_WITH=	system-ffi
LIBFFI_LIB_DEPENDS=	libffi.so:devel/libffi

LTO_IMPLIES=		OPTIMIZATIONS
LTO_CONFIGURE_WITH=	lto

# Use CPPFLAGS over CFLAGS due to -I ordering, causing elementtree and pyexpat
# to break in Python 2.7, or preprocessor complaints in Python >= 3.3
# Upstream Issue: https://bugs.python.org/issue6299
NLS_USES=		gettext
NLS_CPPFLAGS=		-I${LOCALBASE}/include
NLS_LIBS=		-L${LOCALBASE}/lib -lintl
NLS_CONFIGURE_ENV_OFF=	ac_cv_lib_intl_textdomain=no ac_cv_header_libintl_h=no

OPTIMIZATIONS_CONFIGURE_ENABLE=	optimizations

PYMALLOC_CONFIGURE_WITH=pymalloc

THREADS_CONFIGURE_WITH=	threads
THREADS_LDFLAGS=	-lpthread

UCS2_CONFIGURE_ENABLE=	unicode=ucs2
UCS4_CONFIGURE_ENABLE=	unicode=ucs4

TAUTHON_VERSION=	${DISTVERSION:C/-.*\$//}
VERSION_LONG=		${TAUTHON_VERSION}.final.0
VERSION_NOMICRO=	${TAUTHON_VERSION:R}
NAME_VERSION=		tauthon${VERSION_NOMICRO}

DISABLED_MODULES=	_bsddb _sqlite3 _tkinter gdbm

.include <bsd.port.pre.mk>

.if !empty(PORT_OPTIONS:MLTO) && ${CC:T} == "cc"
# Tauthon's configure and Makefile recognize which compiler we are using based
# on the executable name.
.  if ${CC:T} == ${CC}
CC:=${COMPILER_TYPE}
.  else
CC:=${CC:H}/${COMPILER_TYPE}
.  endif
.endif

# https://bugs.python.org/issue22521
# https://bugs.python.org/issue23042
.if !${PORT_OPTIONS:MLIBFFI} && ${ARCH} == i386
BROKEN=	You must use libffi from ports on i386. Enable the LIBFFI option
.endif

.if ${ARCH} == i386
PLIST_SUB+=	X86_ONLY=""
.else
PLIST_SUB+=	X86_ONLY="@comment "
.endif

.if ${ARCH:M*64*}
PLIST_SUB+=	32BIT_ONLY="@comment "
.else
PLIST_SUB+=	32BIT_ONLY=""
.endif

# See https://bugs.freebsd.org/115940 and https://bugs.freebsd.org/193650
.if !exists(/usr/bin/ypcat) || defined(WITHOUT_NIS)
PLIST_SUB+=	NO_NIS="@comment "
DISABLED_MODULES+=	nis
.else
PLIST_SUB+=	NO_NIS=""
.endif

post-patch:
	@${REINPLACE_CMD} -e 's|/usr/local/bin/python|${python_CMD}|' \
		${WRKSRC}/Lib/cgi.py

	@${REINPLACE_CMD} -e \
		's/%%DISABLED_MODULES%%/${DISABLED_MODULES:C/^(.*)$/"\1", /g}/g' \
		${WRKSRC}/setup.py

post-install:
# Create symlink for tauthon executable
	${RLN} ${STAGEDIR}${PREFIX}/bin/${NAME_VERSION} \
		${STAGEDIR}${PREFIX}/bin/tauthon
# Strip shared extensions
	for i in ${STAGEDIR}${PREFIX}/lib/${NAME_VERSION}/lib-dynload/*.so; do \
		${STRIP_CMD} $$i; done
# This code block exists for the qemu-user enabled cross build environment.
# When using this environment in poudriere, CC is not set to the default
# of /usr/bin/cc and a cross-compile toolchain is used.  We need to hand
# edit this so that the run time configuration for python matches what the
# FreeBSD base system provides.  sbruno 02Aug2017
.if ${CC} == /nxb-bin/usr/bin/cc
	@${REINPLACE_CMD} -e 's=/nxb-bin==' \
		${STAGEDIR}${PREFIX}/lib/${NAME_VERSION}/_sysconfigdata.py
	@cd ${WRKSRC} && ${SETENV} LD_LIBRARY_PATH=${WRKSRC} \
		./tauthon -E -m compileall -d ${PREFIX}/lib/${NAME_VERSION} \
		${STAGEDIR}${PREFIX}/lib/${NAME_VERSION}/_sysconfigdata.py
	@cd ${WRKSRC} && ${SETENV} LD_LIBRARY_PATH=${WRKSRC} \
		./tauthon -E -O -m compileall -d ${PREFIX}/lib/${NAME_VERSION} \
		${STAGEDIR}${PREFIX}/lib/${NAME_VERSION}/_sysconfigdata.py
	@${REINPLACE_CMD} -e 's=/nxb-bin==' \
		${STAGEDIR}${PREFIX}/lib/${NAME_VERSION}/config/Makefile
.endif
	${INSTALL_DATA} ${WRKSRC}/Tools/gdb/libpython.py \
		${STAGEDIR}${PREFIX}/lib/lib${NAME_VERSION}.so.1-gdb.py

.include <bsd.port.post.mk>

.if !empty(PORT_OPTIONS:MLTO) && ${COMPILER_TYPE} == gcc
# These have to be appended after bsd.port.mk appends to them, because we are
# basically overriding what it did on seeing USE_BINUTILS (pulled by USE_GCC).
CONFIGURE_ENV+=	AR=${CC:S/^gcc/gcc-ar/} RANLIB=${CC:S/^gcc/gcc-ranlib/}
MAKE_ENV+=	AR=${CC:S/^gcc/gcc-ar/} RANLIB=${CC:S/^gcc/gcc-ranlib/}
.endif
