# $FreeBSD$

PORTNAME=	pennywise
DISTVERSIONPREFIX=	v
DISTVERSION=	0.8.0
CATEGORIES=	www

MAINTAINER=	tagattie@yandex.com
COMMENT=	Open any website or media in a floating window

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/license.md

BUILD_DEPENDS=	app-builder:devel/app-builder2 \
		convert:graphics/ImageMagick7

USES=		desktop-file-utils gnome localbase:ldflags python:2.7,build

USE_GITHUB=	yes
GH_ACCOUNT=	kamranahmedse

INSTALLS_ICONS= yes

BINARY_ALIAS=	python=${PYTHON_CMD}

SUB_FILES=	${PORTNAME}
SUB_LIST=	ELECTRON_VER_MAJOR=${ELECTRON_VER_MAJOR}

PORTDOCS=	readme.md

OPTIONS_DEFINE= DOCS

USE_NODE=	12:yarn
USE_ELECTRON=	5
USE_ELECTRON_HELPER=	fetch extract build

PREFETCH_TIMESTAMP=	1579667052
SYSTEM_ELECTRON_VER=	${ELECTRON_VER}
PKG_ELECTRON_VER=	5.0.3
REBUILD_NPM_CONFIG_ENV+=	SASS_FORCE_BUILD=true

do-build:
	# generate icons without using electron-icon-maker
	${MKDIR} ${WRKSRC}/public/icons/png
.for size in 16 24 32 48 64 128 256 512 1024
	convert -resize ${size}x${size} ${WRKSRC}/public/img/pennywise.png \
		${WRKSRC}/public/icons/png/${size}x${size}.png
.endfor
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${YARN_CMD} _build
	# rebuild native modules for electron
	cd ${PKGJSONSDIR} && \
	for dir in `${FIND} . -type f -name package.json -exec dirname {} ';'`; do \
		cd ${WRKSRC}/$${dir} && ${SETENV} ${MAKE_ENV} \
		SASS_FORCE_BUILD=true \
		npm_config_build_from_source=true \
		npm_config_runtime=electron \
		npm_config_target=${SYSTEM_ELECTRON_VER} \
		npm_config_nodedir=${LOCALBASE}/share/electron${ELECTRON_VER_MAJOR}/node_headers \
		${NPM_CMD} rebuild --no-progress; \
	done
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} \
		USE_SYSTEM_APP_BUILDER=true \
		${NPX_CMD} electron-builder --linux --dir \
		--config.npmRebuild=false \
		--config.electronVersion=${SYSTEM_ELECTRON_VER} \
		--config.electronDist=${LOCALBASE}/share/electron${ELECTRON_VER_MAJOR}

do-install:
	${INSTALL_SCRIPT} ${WRKDIR}/${PORTNAME} ${STAGEDIR}${PREFIX}/bin
	${INSTALL_DATA} ${FILESDIR}/${PORTNAME}.desktop \
		${STAGEDIR}${PREFIX}/share/applications
.for size in 16 24 32 48 64 128 256 512 1024
	${MKDIR} ${STAGEDIR}${PREFIX}/share/icons/hicolor/${size}x${size}/apps
	${INSTALL_DATA} ${WRKSRC}/public/icons/png/${size}x${size}.png \
		${STAGEDIR}${PREFIX}/share/icons/hicolor/${size}x${size}/apps/${PORTNAME}.png
.endfor
	${MKDIR} ${STAGEDIR}${DATADIR}
	cd ${WRKSRC}/dist/linux-unpacked && \
		${COPYTREE_SHARE} resources ${STAGEDIR}${DATADIR}

do-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include "${.CURDIR}/../../devel/electron4/bsd.electron.mk"
.include <bsd.port.mk>
