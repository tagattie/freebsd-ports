PORTNAME=	crush
DISTVERSIONPREFIX=	v
DISTVERSION=	0.2.1
CATEGORIES=	misc

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Glamourous AI coding agent for your favourite terminal
WWW=		https://github.com/charmbracelet/crush

LICENSE=	FSL-1.1-MIT
LICENSE_NAME=	Functional Source License, Version 1.1, MIT Future License
LICENSE_FILE=	${WRKSRC}/LICENSE
LICENSE_PERMS=	dist-mirror pkg-mirror auto-accept

USES=		go:modules

GO_MODULE=	github.com/charmbracelet/crush

PLIST_FILES=	bin/${PORTNAME}
PORTDOCS=	README.md

OPTIONS_DEFINE=	DOCS

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
