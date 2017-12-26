# Created by: Hiroki Tagato <tagattie@yandex.com>
# $FreeBSD$

PORTNAME=	mioswitch
DISTVERSION=	g20171226
CATEGORIES=	net

MAINTAINER=	tagattie@yandex.com
COMMENT=	Automatically disable IIJmio coupon use while packet usage is over preset amount

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	git:devel/git \
		go:lang/go

USES=		gmake
USE_GITHUB=	yes

GH_ACCOUNT=	tagattie
GH_PROJECT=	IIJmio-AutoSwitch
GH_TAGNAME=	0cacd156146d0f6c2ecd09a2d06398219a912f0c

PLIST_FILES=	bin/mioswitch etc/mioswitch.json.sample

pre-build:
	ln -s ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/GNUmakefile ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/Makefile

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/bin/mioswitch ${STAGEDIR}${PREFIX}/bin
	${INSTALL_DATA} ${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}/mioswitch.json.sample ${STAGEDIR}${PREFIX}/etc

.include <bsd.port.mk>
