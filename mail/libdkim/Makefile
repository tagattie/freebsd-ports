PORTNAME=	libdkim
PORTVERSION=	1.0.21
CATEGORIES=	mail
MASTER_SITES=	SF

MAINTAINER=	ports@FreeBSD.org
COMMENT=	DKIM (DomainKeys Identified Mail) library
WWW=		https://dkim.org/

LICENSE=	APACHE20

USES=		uidfix zip ssl
USE_CXXSTD=	gnu++98
USE_LDCONFIG=	YES
SHLIB_MAJOR=	1
PLIST_SUB=	SHLIB_MAJOR="${SHLIB_MAJOR}"

WRKSRC=		${WRKDIR}/${PORTNAME}/src

post-patch:
	${MV} ${WRKSRC}/Makefile ${WRKSRC}/Makefile.prev
	@${SED} -e "s=%%SHLIB_MAJOR%%=${SHLIB_MAJOR}=g" \
		${FILESDIR}/Makefile > ${WRKSRC}/Makefile

.include <bsd.port.mk>

PATCH_ARGS+=	-l
