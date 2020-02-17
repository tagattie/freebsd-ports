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

USES=		desktop-file-utils electron:5 gnome localbase:ldflags \
		node:12,build python:2.7,build

USE_GITHUB=	yes
GH_ACCOUNT=	kamranahmedse

INSTALLS_ICONS= yes

BINARY_ALIAS=	python=${PYTHON_CMD}

SUB_FILES=	${PORTNAME}
SUB_LIST=	ELECTRON_VER_MAJOR=${ELECTRON_VER_MAJOR}

PORTDOCS=	readme.md

OPTIONS_DEFINE= DOCS

USE_NODE=	yarn
USE_ELECTRON=	prefetch extract prebuild build:builder

PREFETCH_TIMESTAMP=	1579667052
UPSTREAM_ELECTRON_VER=	5.0.3

pre-build:
	# generate icons without using electron-icon-maker
	${MKDIR} ${WRKSRC}/public/icons/png
.for size in 16 24 32 48 64 128 256 512 1024
	convert -resize ${size}x${size} ${WRKSRC}/public/img/pennywise.png \
		${WRKSRC}/public/icons/png/${size}x${size}.png
.endfor
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${YARN_CMD} _build

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

.include <bsd.port.mk>
