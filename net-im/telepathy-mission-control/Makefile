PORTNAME=	telepathy-mission-control
PORTVERSION=	5.16.6
PORTREVISION=	4
CATEGORIES=	net-im devel
MASTER_SITES=	http://telepathy.freedesktop.org/releases/telepathy-mission-control/

MAINTAINER=	gnome@FreeBSD.org
COMMENT=	Telepathy component to apps to abstract details of connection managers
WWW=		https://mission-control.sourceforge.net/

LIB_DEPENDS=	libtelepathy-glib.so:net-im/telepathy-glib \
		libdbus-glib-1.so:devel/dbus-glib \
		libdbus-1.so:devel/dbus

USES=		gmake gnome libtool pathfix pkgconfig python:build
USE_GNOME=	glib20 libxslt
GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=${PREFIX}/share
CPPFLAGS+=	-I${LOCALBASE}/include
LIBS+=		-L${LOCALBASE}/lib
CONFIGURE_ARGS+=--disable-upower
USE_LDCONFIG=	yes
GLIB_SCHEMAS=	im.telepathy.MissionControl.FromEmpathy.gschema.xml
INSTALL_TARGET=	install-strip

PLIST_SUB=	VERSION=${PORTVERSION}

post-patch:
	@${REINPLACE_CMD} -e 's|$${datadir}/gtk-doc/html|${DOCSDIR}|' \
		${WRKSRC}/configure

.include <bsd.port.mk>
