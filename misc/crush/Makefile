PORTNAME=	crush
DISTVERSIONPREFIX=	v
DISTVERSION=	0.7.1
CATEGORIES=	misc devel

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Glamourous AI coding agent for your favourite terminal
WWW=		https://github.com/charmbracelet/crush

LICENSE=	FSL-1.1-MIT
LICENSE_NAME=	Functional Source License, Version 1.1, MIT Future License
LICENSE_FILE=	${WRKSRC}/LICENSE.md
LICENSE_PERMS=	dist-mirror pkg-mirror auto-accept

USES=		go:1.25,modules

GO_MODULE=	github.com/charmbracelet/crush
GO_BUILDFLAGS=	-ldflags '-s -w -X github.com/charmbracelet/crush/internal/version.Version=${DISTVERSION}'

PLIST_FILES=	bin/${PORTNAME} \
		etc/bash_completion.d/${PORTNAME} \
		share/fish/completions/${PORTNAME}.fish \
		share/man/man1/${PORTNAME}.1.gz \
		share/zsh/site-functions/_${PORTNAME}
PORTDOCS=	README.md

OPTIONS_DEFINE=	DOCS

post-build:
	@${MKDIR} ${GO_WRKSRC}/completions ${GO_WRKSRC}/manpages
.for sh in bash fish zsh
	${GO_WRKDIR_BIN}/${PORTNAME} completion ${sh} > \
		${GO_WRKSRC}/completions/${PORTNAME}.${sh}
.endfor
	${GO_WRKDIR_BIN}/${PORTNAME} man > ${GO_WRKSRC}/manpages/${PORTNAME}.1

post-install:
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/bash_completion.d \
		${STAGEDIR}${PREFIX}/share/fish/completions \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions
	${INSTALL_DATA} ${GO_WRKSRC}/completions/${PORTNAME}.bash \
		${STAGEDIR}${PREFIX}/etc/bash_completion.d/${PORTNAME}
	${INSTALL_DATA} ${GO_WRKSRC}/completions/${PORTNAME}.fish \
		${STAGEDIR}${PREFIX}/share/fish/completions/${PORTNAME}.fish
	${INSTALL_DATA} ${GO_WRKSRC}/completions/${PORTNAME}.zsh \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions/_${PORTNAME}
	${INSTALL_MAN} ${GO_WRKSRC}/manpages/${PORTNAME}.1 \
		${STAGEDIR}/${PREFIX}/share/man/man1

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${GO_WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
