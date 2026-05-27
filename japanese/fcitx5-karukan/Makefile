PORTNAME=	fcitx5-karukan
DISTVERSION=	g20260523
CATEGORIES=	japanese textproc

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Japanese input method for fcitx5 powered by neural kana-kanji conversion
WWW=		https://github.com/togatoga/karukan

LICENSE=		APACHE20 MIT
LICENSE_COMB=		dual
LICENSE_FILE_APACHE20=	${WRKSRC}/LICENSE-APACHE
LICENSE_FILE_MIT=	${WRKSRC}/LICENSE-MIT

LIB_DEPENDS=	libonig.so:devel/oniguruma \
		libFcitx5Core.so:textproc/fcitx5 \
		libxkbcommon.so:x11/libxkbcommon

USES=		cargo cmake:noninja gmake llvm:build,lib localbase:ldflags ssl

USE_GITHUB=	yes
GH_ACCOUNT=	togatoga
GH_PROJECT=	karukan
GH_TAGNAME=	3a96300e1005a80f6acb98b3dba877ed2041f8ba

CMAKE_SOURCE_PATH=	${WRKSRC}/karukan-im/fcitx5-addon

CARGO_CARGOTOML=	${WRKSRC}/Cargo.toml
CARGO_CARGOLOCK=	${WRKSRC}/Cargo.lock
CARGO_BUILD=		no
CARGO_BUILD_ARGS=	--package karukan-cli
CARGO_INSTALL=		no
CARGO_TEST=		no

RUSTFLAGS=		-C link-arg=-lomp

post-patch:
	@${REINPLACE_CMD} -e "s|/usr|${PREFIX}|" \
		${WRKSRC}/karukan-cli/src/bin/server.rs

post-build:
	${CARGO_CARGO_RUN} build --manifest-path=${CARGO_CARGOTOML} \
		--verbose --verbose \
		${CARGO_BUILD_ARGS}

post-install:
.for prog in ajimee-bench karukan-dict karukan-server sudachi-dict
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/${prog} \
		${STAGEDIR}${PREFIX}/bin
.endfor
	@${MKDIR} ${STAGEDIR}${PREFIX}/share/karukan
	cd ${WRKSRC}/karukan-cli && \
	${COPYTREE_SHARE} static ${STAGEDIR}${PREFIX}/share/karukan
	@${MKDIR} ${STAGEDIR}${PREFIX}/share/fcitx5/karukan
.for f in default.toml default_user_dict.txt
	${INSTALL_DATA} ${WRKSRC}/karukan-im/config/${f} \
		${STAGEDIR}${PREFIX}/share/fcitx5/karukan
.endfor

.include <bsd.port.mk>
