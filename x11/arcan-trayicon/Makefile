PORTNAME=	arcan-trayicon
DISTVERSION=	0.7.0.1
CATEGORIES=	x11

MAINTAINER=	hoanga@gmail.com
COMMENT=	Generic wrapper for registering Arcan clients as tray icons
WWW=		https://arcan-fe.com/

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC:H:H:H}/data/distr/LICENSE.BSD

LIB_DEPENDS=	libarcan_shmif_ext.so:multimedia/arcan

USES=		compiler:c11 meson pkgconfig
USE_GITHUB=	yes
GH_ACCOUNT=	letoram
GH_PROJECT=	arcan
WRKSRC_SUBDIR=	src/tools/${PORTNAME:S/arcan-//}
PLIST_FILES=	bin/${PORTNAME}

.include <bsd.port.mk>
