PORTNAME=	lmtp2nntp
PORTVERSION=	1.4.1
PORTREVISION=	1
CATEGORIES=	mail news
MASTER_SITES=	OSSP/tool/${PORTNAME}

MAINTAINER=	vd@FreeBSD.org
COMMENT=	OSSP mail to news gateway
WWW=		http://www.ossp.org/pkg/tool/lmtp2nntp/

LICENSE=	GPLv2+
LICENSE_FILE=	${WRKSRC}/COPYING

DEPRECATED=	Obsolete, last release in 2005 and uses legacy Local Mail Transfer Protocol
EXPIRATION_DATE=2025-07-31

LIB_DEPENDS=	libstr.so:devel/str \
		libex.so:devel/ossp-ex \
		libsa.so:net/ossp-sa \
		libvar.so:devel/ossp-var

GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=	${PREFIX}/share
CONFIGURE_ARGS=	--with-ex=${LOCALBASE} --with-str=${LOCALBASE} \
		--with-sa=${LOCALBASE} --with-var=${LOCALBASE}
TEST_TARGET=	check

PLIST_FILES=	sbin/lmtp2nntp \
		share/man/man8/lmtp2nntp.8.gz
PORTDOCS=	INSTALL README

OPTIONS_DEFINE=	DOCS

# Try to fix parallel builds (-jX)
post-patch:
	@${REINPLACE_CMD} -e '/^all:/s, _SUBDIRS_all,, ; \
		/^$$(PROG)/s,$$, _SUBDIRS_all,' ${WRKSRC}/Makefile.in

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${PORTDOCS:S,^,${WRKSRC}/,} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
