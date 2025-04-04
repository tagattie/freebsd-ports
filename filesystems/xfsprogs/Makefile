PORTNAME=	xfsprogs
DISTVERSION=	3.2.4
PORTREVISION=	4
CATEGORIES=	filesystems sysutils
MASTER_SITES=	ftp://oss.sgi.com/projects/xfs/cmd_tars/ \
		ftp://oss.sgi.com/projects/xfs/previous/cmd_tars/

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Utilities for managing XFS filesystems
WWW=		http://oss.sgi.com/projects/xfs/

LICENSE=	LGPL21 GPLv2
LICENSE_COMB=	multi
LICENSE_FILE=	${WRKSRC}/doc/COPYING

USES=		cpe gettext gmake libtool:build readline
CPE_VENDOR=	sgi
CPE_PRODUCT=	${PORTNAME}
GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=	${PREFIX}/share
CONFIGURE_ENV=	OPTIMIZER=-fcommon
CONFIGURE_ARGS=	--disable-shared --enable-readline
ALL_TARGET=	# empty
MAKE_ARGS=	V=1

OPTIONS_DEFINE=	DOCS

post-patch:
	@${REINPLACE_CMD} -e '/utmp/d' \
		${WRKSRC}/quota/report.c ${WRKSRC}/quota/util.c
	@${REINPLACE_CMD} \
		-e 's|/usr/local|${LOCALBASE}|g' \
		-e 's/lio_listio/timer_create/' \
		${WRKSRC}/configure
	@${REINPLACE_CMD} -e '/#!/s|bash|sh|' \
		${WRKSRC}/include/install-sh ${WRKSRC}/install-sh

post-install:
.for f in mkfs.xfs xfs_copy xfs_db xfs_estimate xfs_fsr xfs_growfs xfs_io \
     xfs_logprint xfs_mdrestore xfs_quota xfs_repair xfs_rtcp
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/sbin/${f}
.endfor

.include <bsd.port.mk>
