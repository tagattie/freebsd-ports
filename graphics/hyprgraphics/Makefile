PORTNAME=	hyprgraphics
DISTVERSIONPREFIX=	v
DISTVERSION=	0.1.3
PORTREVISION=	2
CATEGORIES=	graphics

MAINTAINER=	jbeich@FreeBSD.org
COMMENT=	Hyprland graphics / resource utilities
WWW=		https://github.com/hyprwm/hyprgraphics

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libhyprutils.so:devel/hyprutils \
		libjxl.so:graphics/libjxl \
		libspng.so:graphics/libspng \
		libwebp.so:graphics/webp

USES=		cmake:testing compiler:c++11-lib gnome jpeg pkgconfig xorg
USE_GITHUB=	yes
USE_GNOME=	cairo
USE_LDCONFIG=	yes
USE_XORG=	pixman
GH_ACCOUNT=	hyprwm
PLIST_SUB=	VERSION=${DISTVERSION:C/-.*//}

.include <bsd.port.mk>
