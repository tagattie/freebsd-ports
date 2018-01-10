# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	mioswitch
DISTVERSION=	g20180110
CATEGORIES=	net

MAINTAINER=	tagattie@yandex.com
COMMENT=	Automatically disable IIJmio coupon use while packet usage is over preset amount

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		gmake go
USE_GITHUB=	yes

GH_ACCOUNT=	tagattie
GH_PROJECT=	IIJmio-AutoSwitch
GH_TAGNAME=	5dc75be7b826add1dd465a6d08f063923d1b47f5

PLIST_FILES=	bin/mioswitch etc/mioswitch.json.sample

pre-build:
	ln -s ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/GNUmakefile ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/Makefile

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/bin/mioswitch ${STAGEDIR}${PREFIX}/bin
	${INSTALL_DATA} ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/mioswitch.json.sample ${STAGEDIR}${PREFIX}/etc

.include <bsd.port.mk>
