# $FreeBSD$

PORTNAME=	meteo
PORTVERSION=	0.9.6
CATEGORIES=	deskutils

MAINTAINER=	tagattie@yandex.com
COMMENT=	Weather forecast application using OpenWeatherMap API

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/COPYING

BUILD_DEPENDS=	valac:lang/vala \
		appstream-util:devel/appstream-glib
LIB_DEPENDS=	libsoup-2.4.so:devel/libsoup \
		libjson-glib-1.0.so:devel/json-glib \
		libgeocode-glib.so:net/geocode-glib \
		libwebkit2gtk-4.0.so:www/webkit2-gtk3 \
		libappindicator3.so:devel/libappindicator

USES=		desktop-file-utils gettext gnome meson pkgconfig \
		python:3.5+,build shebangfix

USE_GITLAB=	yes
GL_ACCOUNT=	bitseater
GL_COMMIT=	1a36202057ddd502556afd9335a8f8d51b714537

USE_GNOME=	gtk30

SHEBANG_FILES=	meson/post_install.py

GLIB_SCHEMAS=	com.gitlab.bitseater.meteo.gschema.xml
INSTALLS_ICONS=	yes

OPTIONS_DEFINE=	DOCS

PORTDOCS=	AUTHORS CONTRIBUTING.md CREDITS.md README.md changelog

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
.for doc in AUTHORS CONTRIBUTING.md CREDITS.md README.md
	${INSTALL_MAN} ${WRKSRC}/${doc} ${STAGEDIR}${DOCSDIR}
.endfor
	${INSTALL_MAN} ${WRKSRC}/debian/changelog ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
