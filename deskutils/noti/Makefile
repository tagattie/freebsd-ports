# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	noti
PORTVERSION=	2.7.0
DISTVERSIONPREFIX=v
CATEGORIES=	deskutils

MAINTAINER=	tagattie@yandex.com
COMMENT=	Trigger notifications when a process completes

RUN_DEPENDS=	notify-send:devel/libnotify

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		go
USE_GITHUB=	yes

GH_ACCOUNT=	variadico

GO_PKGNAME=	github.com/${GH_ACCOUNT}/${PORTNAME}
GO_TARGET=	github.com/${GH_ACCOUNT}/${PORTNAME}/cmd/noti

PLIST_FILES=	bin/noti

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/bin/noti ${STAGEDIR}${PREFIX}/bin

.include <bsd.port.mk>
