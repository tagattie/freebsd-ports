PORTNAME=	bcompare
DISTVERSION=	4.4.7
DISTVERSIONSUFFIX=	.28397
CATEGORIES=	editors devel linux
MASTER_SITES=	https://scootersoftware.com/files/
PKGNAMEPREFIX=	linux-
DISTNAME_amd64=	${DISTNAME}.x86_64
DISTNAME_i386=	${DISTNAME}.i386
EXTRACT_SUFX=	.rpm

MAINTAINER=	matias@pizarro.net
COMMENT=	Compare, sync, and merge files and folders (X11)
WWW=		https://www.scootersoftware.com/

LICENSE=	SCOOTERSOFTWARE
LICENSE_NAME=	Scooter Software License
LICENSE_FILE=	${FILESDIR}/LICENSE.txt
LICENSE_PERMS=	dist-mirror no-dist-sell pkg-mirror no-pkg-sell auto-accept

ONLY_FOR_ARCHS=	amd64 i386
ONLY_FOR_ARCHS_REASON=	upstream only supports amd64 and i386

USES=		desktop-file-utils gnome linux:c7 shared-mime-info shebangfix
USE_LINUX=	base:run devtools:build qt-x11:run xorglibs:run

SHEBANG_FILES=	${BCLIB_SRC}/kde_context_menu

NO_BUILD=	yes

NO_WRKSUBDIR=	true

DATA_FILTER=	-type d -o -name *\.html -o -name *\.js -o -name *\.css -o \
		-name *\.gif -o -name *\.png -o -name *\.jpg -o -name \
		*\.desktop -o -name mime\.types -o -name README -o -name \
		BCompare.mad -o -name RPM-GPG-KEY-scootersoftware -o -name \
		scootersoftware.repo

LIB_FILES=	libQt4Pas.so.5 lib7z.so libunrar.so nosched.so
PROGRAM_FILES=	BCompare bcmount32 bcmount64
SCRIPT_FILES=	bcmount.sh kde_context_menu

.include <bsd.port.options.mk>

.if ${ARCH} == i386
LIBDIR=		lib
EXTRA_PATCHES=	${PATCHDIR}/extra-patch-usr_bin_bcompare_i386
PLIST_SUB+=	QT4AMD64="@comment "
.elif ${ARCH} == amd64
LIBDIR=		lib64
EXTRA_PATCHES=	${PATCHDIR}/extra-patch-usr_bin_bcompare_amd64
PLIST_SUB+=	QT4AMD64=""
.endif

BCLIB_SRC=	${WRKSRC}/usr/${LIBDIR}/beyondcompare
BCLIB_STG=	${STAGEDIR}${PREFIX}/lib/beyondcompare

post-patch:
	${REINPLACE_CMD} -e 's|^#!/usr/bin/sh$$|#!/bin/sh|' ${BCLIB_SRC}/bcmount.sh

pre-install:
	${LINUXBASE}/usr/bin/gcc --sysroot=${LINUXBASE} -Wall -fPIC -shared \
		${FILESDIR}/nosched.c -ldl -o ${BCLIB_SRC}/nosched.so

do-install:
# bin
	${INSTALL_SCRIPT} ${WRKSRC}/usr/bin/bcompare ${STAGEDIR}${PREFIX}/bin
# lib
	(cd ${BCLIB_SRC} && ${COPYTREE_SHARE}  .                ${BCLIB_STG} "${DATA_FILTER}")
	(cd ${BCLIB_SRC} && ${INSTALL_PROGRAM} ${PROGRAM_FILES} ${BCLIB_STG})
	(cd ${BCLIB_SRC} && ${INSTALL_SCRIPT}  ${SCRIPT_FILES}  ${BCLIB_STG})
	(cd ${BCLIB_SRC} && ${INSTALL_LIB}     ${LIB_FILES}     ${BCLIB_STG})
.if ${ARCH} == amd64
	${INSTALL_LIB} ${BCLIB_SRC}/qt4/*.so.4 ${BCLIB_STG}/qt4
.endif
	${INSTALL_LIB} ${BCLIB_SRC}/ext/*.so   ${BCLIB_STG}/ext
	${LN} -sf ${LINUXBASE}/usr/${LIBDIR}/libbz2.so.1.0.6 ${BCLIB_STG}/libbz2.so.1.0
# data
	(cd ${WRKSRC}/usr/share && ${COPYTREE_SHARE} . ${STAGEDIR}${PREFIX}/share)
	${MKDIR} ${STAGEDIR}${PREFIX}/share/icons/hicolor/16x16/apps
	${MKDIR} ${STAGEDIR}${PREFIX}/share/icons/hicolor/32x32/apps
	${MKDIR} ${STAGEDIR}${PREFIX}/share/icons/hicolor/48x48/apps
	${LN} -sf ../../../../pixmaps/bcompare.png \
		${STAGEDIR}${PREFIX}/share/icons/hicolor/16x16/apps/bcompare.png
	${LN} -sf ../../../../pixmaps/bcomparefull32.png \
		${STAGEDIR}${PREFIX}/share/icons/hicolor/32x32/apps/bcompare.png
	${LN} -sf ../../../../pixmaps/bcompare.png \
		${STAGEDIR}${PREFIX}/share/icons/hicolor/48x48/apps/bcompare.png

.include <bsd.port.mk>
