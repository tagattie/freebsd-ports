PORTNAME=	tinc
PORTVERSION=	1.0.36
PORTREVISION=	3
CATEGORIES=	security net-vpn
MASTER_SITES=	https://www.tinc-vpn.org/packages/ \
		http://www.tinc-vpn.org/packages/

MAINTAINER=	dinoex@FreeBSD.org
COMMENT=	Virtual Private Network (VPN) daemon
WWW=		https://www.tinc-vpn.org/

LICENSE=	GPLv3

LIB_DEPENDS=	liblzo2.so:archivers/lzo2

USERS=	tinc
GROUPS=	tinc
USES=		cpe ssl makeinfo
CPE_VENDOR=	tinc-vpn
GNU_CONFIGURE=	yes
GNU_CONFIGURE_MANPREFIX=	${PREFIX}/share
CPPFLAGS+=	-I${LOCALBASE}/include
LDFLAGS+=	-L${LOCALBASE}/lib
CONFIGURE_ARGS=	--localstatedir=/var
USE_RC_SUBR=	tincd
INFO=		tinc
PLIST_FILES=	"@(root,tinc,2550) sbin/tincd" \
		share/man/man5/tinc.conf.5.gz share/man/man8/tincd.8.gz

.include <bsd.port.mk>
