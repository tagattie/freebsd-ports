PORTNAME=	duktape
DISTVERSION=	2.7.0
CATEGORIES=	lang devel
MASTER_SITES=	https://github.com/svaarala/${PORTNAME}/releases/download/v${DISTVERSION}/ \
		https://duktape.org/

MAINTAINER=	desktop@FreeBSD.org
COMMENT=	Embeddable Javascript engine
WWW=		https://duktape.org/

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

USES=		tar:xz

MAKEFILE=	Makefile.cmdline
ALL_TARGET=	duk

PLIST_FILES=	bin/duk

PORTDOCS=	README.rst

OPTIONS_DEFINE=	DOCS

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/duk ${STAGEDIR}${PREFIX}/bin

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	cd ${WRKSRC} && ${INSTALL_DATA} ${PORTDOCS} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
