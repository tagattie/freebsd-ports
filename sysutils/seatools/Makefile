PORTNAME=		seatools
PORTVERSION=		2.54
CATEGORIES=		sysutils linux
MASTER_SITES=		${DOWNLOAD_URL}
PKGNAMEPREFIX=		linux-
DISTNAME=		seatools_cli

MAINTAINER=		cy@FreeBSD.org
COMMENT=		Seagate's SeaTools for Linux
WWW=		https://www.seagate.com

# Converted from RESTRICTED
LICENSE=	seatools
LICENSE_NAME=	seatools
LICENSE_TEXT=	Redistribution is not permitted
LICENSE_PERMS=	auto-accept

DOWNLOAD_URL=		http://www.seagate.com/files/www-content/support-content/downloads/seatools/_shared/downloads/seatools_cli.tar
NO_WRKSUBDIR=		yes
ONLY_FOR_ARCHS=		i386 amd64
USES=			linux tar

PLIST_FILES=		sbin/st
PORTDOCS=		sthelp.txt

OPTIONS_DEFINE=		DOCS

do-build:
	@${BRANDELF} -t Linux ${WRKSRC}/st

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/st ${STAGEDIR}${PREFIX}/sbin/st
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/sthelp.txt ${STAGEDIR}${DOCSDIR}/sthelp.txt

.include <bsd.port.pre.mk>

.if (!exists(${DISTDIR}/${DISTNAME}${EXTRACT_SUFX}))
IGNORE=	please fetch the distfile: ${DOWNLOAD_URL} by hand and place it in ${DISTDIR}
.endif

.include <bsd.port.post.mk>
