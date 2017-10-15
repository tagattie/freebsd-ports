# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	xcape
PORTVERSION=	g20170807
CATEGORIES=	x11

MAINTAINER=	tagattie@yandex.com
COMMENT=	Linux utility to configure modifier keys to act as other keys when pressed and released on their own.

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

USE_GITHUB=	yes
GH_ACCOUNT=	alols
GH_TAGNAME=	6ded5b453b50642197044a170402664cbf6d2c96

USES=		pkgconfig
USE_XORG=	x11 xtst

PLIST_FILES=	bin/xcape man/man1/xcape.1.gz

post-patch:
	@${REINPLACE_CMD} -i '' -e 's|%%LOCALBASE%%|${LOCALBASE}|' ${WRKSRC}/Makefile

.include <bsd.port.mk>
