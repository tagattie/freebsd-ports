PORTNAME=	codex
DISTVERSIONPREFIX=	rust-v
DISTVERSION=	0.6.0
CATEGORIES=	misc

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Lightweight coding agent that runs in your terminal
WWW=		https://openai.com/codex \
		https://github.com/openai/codex

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libonig.so:devel/oniguruma

USES=		cargo ssl

USE_GITHUB=	yes
GH_ACCOUNT=	openai

CARGO_CARGOTOML=	${BUILD_WRKSRC}/Cargo.toml
CARGO_CARGOLOCK=	${BUILD_WRKSRC}/Cargo.lock
CARGO_INSTALL_PATH=	codex-rs
CARGO_ENV=		OPENSSL_NO_VENDOR=1

BUILD_WRKSRC=	${WRKSRC}/codex-rs

PLIST_FILES=	bin/codex \
		bin/codex-exec \
		bin/codex-execpolicy \
		bin/codex-file-search \
		bin/codex-mcp-client \
		bin/codex-mcp-server \
		bin/codex-tui
PORTDOCS=	README.md config.md

OPTIONS_DEFINE=	DOCS

post-build:
	@${MKDIR} ${WRKSRC}/completions
.for sh in bash fish zsh
	${CARGO_TARGET_DIR}/*/${PORTNAME} completion ${sh} > ${WRKSRC}/completions/${PORTNAME}.${sh}
.endfor

do-install:
.for f in ${PLIST_FILES:S/bin\///}
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/${f} ${STAGEDIR}${PREFIX}/bin
.endfor
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/bash_completion.d \
		${STAGEDIR}${PREFIX}/share/fish/completions \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions
	${INSTALL_DATA} ${WRKSRC}/completions/${PORTNAME}.bash \
		${STAGEDIR}${PREFIX}/etc/bash_completion.d/${PORTNAME}
	${INSTALL_DATA} ${WRKSRC}/completions/${PORTNAME}.fish \
		${STAGEDIR}${PREFIX}/share/fish/completions/${PORTNAME}.fish
	${INSTALL_DATA} ${WRKSRC}/completions/${PORTNAME}.zsh \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions/_${PORTNAME}

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${BUILD_WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
