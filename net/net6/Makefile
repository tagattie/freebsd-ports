PORTNAME=	net6
PORTVERSION=	1.3.14
PORTREVISION=	9
CATEGORIES=	net devel
MASTER_SITES=	http://releases.0x539.de/net6/

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Network access framework for IPv4/IPv6
WWW=		http://gobby.0x539.de

LICENSE=	LGPL20+ # though COPYING is LGPL21
LICENSE_FILE=	${WRKSRC}/COPYING

DEPRECATED=	Abandoned upstream, repo archived Nov 26, 2020
EXPIRATION_DATE=2025-06-30

LIB_DEPENDS=	libsigc-2.0.so:devel/libsigc++20 \
		libgnutls.so:security/gnutls

GNU_CONFIGURE=	yes
INSTALL_TARGET=	install-strip
USES=		cpe libtool pathfix pkgconfig compiler:c++11-lang
USE_CXXSTD=	c++11
USE_LDCONFIG=	yes

CPE_VENDOR=	armin_burgmeier

OPTIONS_DEFINE=	NLS
OPTIONS_SUB=	yes

NLS_USES=	gettext
NLS_CONFIGURE_ENABLE=	nls
NLS_CPPFLAGS=	-I${LOCALBASE}/include
NLS_LDFLAGS=	-L${LOCALBASE}/lib

.include <bsd.port.mk>
