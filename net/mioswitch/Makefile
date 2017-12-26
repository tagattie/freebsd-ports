# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	mioswitch
DISTVERSION=	g20171104
CATEGORIES=	net

MAINTAINER=	tagattie@yandex.com
COMMENT=	Automatically disable IIJmio coupon use while packet usage is over preset amount

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		go
USE_GITHUB=	yes

GH_ACCOUNT=	tagattie
GH_PROJECT=	IIJmio-AutoSwitch
GH_TAGNAME=	f9201d58b2a6a286ba72b02f939d9d0875b552e5

PLIST_FILES=	bin/mioswitch etc/mioswitch.json.sample

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/bin/mioswitch ${STAGEDIR}${PREFIX}/bin
	${INSTALL_DATA} ${WRKDIR}/mioswitch.json.sample ${STAGEDIR}${PREFIX}/etc

# post-install:
# 	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/xcape

.include <bsd.port.mk>
