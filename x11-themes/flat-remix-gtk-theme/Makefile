# $FreeBSD$

PORTNAME=	flat-remix-gtk
PORTVERSION=	20190414
CATEGORIES=	x11-themes
PKGNAMESUFFIX=	-theme

MAINTAINER=	tagattie@yandex.com
COMMENT=	Pretty simple GTK window theme inspired on material design

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	gtk-murrine-engine>0:x11-themes/gtk-murrine-engine \
		gnome-themes-extra>0:x11-themes/gnome-themes-extra

USES=		gmake

USE_GITHUB=	yes
GH_ACCOUNT=	daniruiz

NO_ARCH=	yes
NO_BUILD=	yes

REINPLACE_ARGS=	-i ''

post-patch:
	${REINPLACE_CMD} -E 's/^(MetacityTheme=Flat-Remix)/\1-GTK/; \
			s/^(IconTheme=).*/\1Adwaita/; \
			s/^(CursorTheme=).*/\1Adwaita/' \
		${WRKSRC}/*/index.theme

.include <bsd.port.mk>
