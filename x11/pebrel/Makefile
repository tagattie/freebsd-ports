PORTNAME=	pebrel
DISTVERSIONPREFIX=	v
DISTVERSION=	2.3.1
CATEGORIES=	x11

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	GPU-accelerated terminal, SSH workspace, and home for your AI CLI sessions
WWW=		https://github.com/Kuddev/pebrel

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libxkbcommon.so:x11/libxkbcommon \
		libfontconfig.so:x11-fonts/fontconfig

USES=		cargo desktop-file-utils pkgconfig xorg

USE_GITHUB=	yes
GH_ACCOUNT=	Kuddev

USE_XORG=	xcb

CARGO_BUILD_ARGS=	--package nebula \
			--bin pebrel \
			--bin pebrel-cli
CARGO_INSTALL_PATH=	nebula_app

PLIST_FILES=	bin/${PORTNAME} \
		bin/${PORTNAME}-cli \
		share/applications/io.github.kuddev.pebrel.desktop
PORTDOCS=	CHANGELOG.md CONTRIBUTING.md README.md SECURITY.md

OPTIONS_DEFINE=	COMPLETIONS DOCS
OPTIONS_DEFAULT=COMPLETIONS

COMPLETIONS_DESC=	Build and/or install shell completions

COMPLETIONS_PLIST_FILES=etc/bash_completion.d/${PORTNAME} \
			share/fish/completions/${PORTNAME}.fish \
			share/zsh/site-functions/_${PORTNAME}

post-extract:
	# prevent `cargo configure` from finding a wrong Cargo.toml
	@${MV} ${WRKSRC_crate_gpui}/tooling/lints/test_fixture/gpui/Cargo.toml \
		${WRKSRC_crate_gpui}/tooling/lints/test_fixture/gpui/Cargo.toml.bak
	@${MV} ${WRKSRC_crate_gpui_shared_string}/tooling/lints/test_fixture/gpui_shared_string/Cargo.toml \
		${WRKSRC_crate_gpui_shared_string}/tooling/lints/test_fixture/gpui_shared_string/Cargo.toml.bak

post-patch:
	@${BSDMAKE} PATCHDIR=${PATCHDIR}/gpui \
		WRKSRC=${WRKSRC_crate_gpui} do-patch
# https://github.com/RazrFalcon/fontdb/issues/61
	@${REINPLACE_CMD} -e 's|/etc|${LOCALBASE}&|g' \
		${WRKSRC_crate_fontdb}/src/lib.rs

post-install:
	${INSTALL_DATA} ${WRKSRC}/packaging/linux/io.github.kuddev.pebrel.desktop \
		${STAGEDIR}${DESKTOPDIR}

post-install-COMPLETIONS-on:
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/bash_completion.d \
		${STAGEDIR}${PREFIX}/share/fish/completions \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions
	${INSTALL_DATA} ${WRKSRC}/extra/completions/pebrel.bash \
		${STAGEDIR}${PREFIX}/etc/bash_completion.d/${PORTNAME}
	${INSTALL_DATA} ${WRKSRC}/extra/completions/pebrel.fish \
		${STAGEDIR}${PREFIX}/share/fish/completions/${PORTNAME}.fish
	${INSTALL_DATA} ${WRKSRC}/extra/completions/_${PORTNAME} \
		${STAGEDIR}${PREFIX}/share/zsh/site-functions/_${PORTNAME}

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
