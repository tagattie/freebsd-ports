PORTNAME=	lxinput
PORTVERSION=	0.3.5
PORTREVISION=	2
CATEGORIES=	sysutils
MASTER_SITES=	SF/lxde/LXInput%20%28Kbd%20and%20amp_%20mouse%20config%29/LXInput%20${PORTVERSION:R}.x/

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Keyboard and mouse settings
WWW=		https://lxde.org/

LICENSE=	GPLv2+  # though COPYING is GPLv3
LICENSE_FILE=	${WRKSRC}/COPYING

LIB_DEPENDS=	libfontconfig.so:x11-fonts/fontconfig \
		libfreetype.so:print/freetype2

USES=		gmake gnome localbase pkgconfig tar:xz xorg
USE_GNOME=	gtk20 cairo gdkpixbuf intltool
USE_XORG=	x11
GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=${PREFIX}/share
USE_LDCONFIG=	yes

OPTIONS_DEFINE=	NLS
OPTIONS_SUB=	yes

NLS_USES=	gettext
NLS_USES_OFF=	gettext-tools

post-patch-NLS-off:
	@${REINPLACE_CMD} -e 's|po man|man|' ${WRKSRC}/Makefile.in

.include <bsd.port.mk>
