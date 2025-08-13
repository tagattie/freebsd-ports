PORTNAME=	walker
DISTVERSIONPREFIX=	v
DISTVERSION=	0.13.26
CATEGORIES=	x11 wayland
PKGNAMESUFFIX=	-launcher

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Multi-purpose launcher with a lot of features
WWW=		https://github.com/abenz1267/walker

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libgraphene-1.0.so:graphics/graphene \
		libvips.so:graphics/vips \
		libvulkan.so:graphics/vulkan-loader \
		libharfbuzz.so:print/harfbuzz \
		libgtk4-layer-shell.so:x11-toolkits/gtk4-layer-shell

USES=		gettext-runtime gnome go:modules

USE_GNOME=	cairo gdkpixbuf glib20 gtk40 introspection pango

GO_MODULE=	github.com/abenz1267/walker
GO_TARGET=	./cmd

DOCSDIR=	${PREFIX}/share/doc/${PKGBASE}

PORTDOCS=	README.md

OPTIONS_DEFINE=	DOCS

post-patch:
	@${REINPLACE_CMD} -e 's|/etc|${PREFIX}&|' \
		${GO_WRKSRC}/internal/util/file.go

post-install:
	@${MV} ${STAGEDIR}${PREFIX}/bin/cmd ${STAGEDIR}${PREFIX}/bin/${PKGBASE}
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/xdg/${PORTNAME}
	${INSTALL_DATA} ${GO_WRKSRC}/internal/config/config.default.toml \
		${STAGEDIR}${PREFIX}/etc/xdg/${PORTNAME}
	cd ${GO_WRKSRC}/internal/config && \
		${COPYTREE_SHARE} themes ${STAGEDIR}${PREFIX}/etc/xdg/${PORTNAME}

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
