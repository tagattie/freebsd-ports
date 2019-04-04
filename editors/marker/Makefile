# $FreeBSD$

PORTNAME=	marker
PORTVERSION=	2018.07.03
CATEGORIES=	editors

MAINTAINER=	tagattie@yandex.com
COMMENT=	Gtk3 markdown editor

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE.md

LIB_DEPENDS=	libwebkit2gtk-4.0.so:www/webkit2-gtk3 \
		libgtkspell3-3.so:textproc/gtkspell3
RUN_DEPENDS=	pandoc:textproc/hs-pandoc

USES=		desktop-file-utils gnome meson

USE_GITHUB=	yes
GH_ACCOUNT=	fabiocolacio
GH_PROJECT=	Marker
GH_TUPLE=	Mandarancio:scidown:b568bf3:scidown \
		Mandarancio:charter:bc544eb:charter \
		codeplea:tinyexpr:9476568:tinyexpr

USE_GNOME=	glib20 gtk30 gtksourceview3
GLIB_SCHEMAS=	com.github.fabiocolacio.marker.gschema.xml
INSTALLS_ICONS=	yes

OPTIONS_DEFINE=	DOCS

PORTDOCS=	CONTRIBUTING.md README.md

DATADIR=	${PREFIX}/share/com.github.fabiocolacio.marker

post-extract:
	${MV} ${WRKSRC_scidown}/* ${WRKSRC}/src/scidown
	${MV} ${WRKSRC_charter}/* ${WRKSRC}/src/scidown/src/charter
	${MV} ${WRKSRC_tinyexpr}/* ${WRKSRC}/src/scidown/src/charter/src/tinyexpr

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S/^/${WRKSRC}\//} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
