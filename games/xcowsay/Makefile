PORTNAME=	xcowsay
DISTVERSION=	1.6
PORTREVISION=	4
CATEGORIES=	games
MASTER_SITES=	http://www.nickg.me.uk/files/ \
		LOCAL/ehaupt

MAINTAINER=	ehaupt@FreeBSD.org
COMMENT=	Graphical configurable talking cow
WWW=		https://www.doof.me.uk/xcowsay/

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/COPYING

LIB_DEPENDS=	libfontconfig.so:x11-fonts/fontconfig \
		libfreetype.so:print/freetype2

USES=		gettext gmake gnome pkgconfig
USE_CSTD=	c99
USE_GNOME=	cairo gdkpixbuf gtk30
GNU_CONFIGURE=	yes

OPTIONS_DEFINE=	DBUS

DBUS_LIB_DEPENDS=	libdbus-1.so:devel/dbus \
			libdbus-glib-1.so:devel/dbus-glib
DBUS_CONFIGURE_ENABLE=	dbus
DBUS_CFLAGS=		-DWITH_DBUS

.include <bsd.port.mk>
