PORTNAME=	warble
DISTVERSION=	2.0.1
CATEGORIES=	games

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Word-guessing game built in Vala and Gtk
WWW=		https://github.com/avojak/warble

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libgee-0.8.so:devel/libgee \
		libgranite-7.so:x11-toolkits/granite7

USES=		gettext-tools gnome meson pkgconfig vala:build

USE_GITHUB=	yes
GH_ACCOUNT=	avojak

USE_GNOME=	cairo glib20 gtk40 libadwaita
GLIB_SCHEMAS=	com.github.avojak.warble.gschema.xml

PORTDOCS=	CONTRIBUTING.md README.md

OPTIONS_DEFINE=	DOCS

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
