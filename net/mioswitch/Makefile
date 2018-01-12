# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	mioswitch
DISTVERSION=	0.3.0
CATEGORIES=	net

MAINTAINER=	tagattie@yandex.com
COMMENT=	Automatically disable IIJmio coupon use while packet usage is over preset amount

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		gmake go
USE_GITHUB=	yes

GH_ACCOUNT=	tagattie
GH_PROJECT=	IIJmio-AutoSwitch

PLIST_FILES=	bin/mioswitch etc/mioswitch.json.sample

pre-build:
	ln -s ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/GNUmakefile ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/Makefile

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/bin/mioswitch ${STAGEDIR}${PREFIX}/bin
	${INSTALL_DATA} ${WRKDIR}/${GH_PROJECT}-${DISTVERSION}/mioswitch.json.sample ${STAGEDIR}${PREFIX}/etc

.include <bsd.port.mk>
